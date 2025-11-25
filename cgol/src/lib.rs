mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;

use js_sys::Math;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, Window};

const CELL_SIZE: u32 = 20;
const SIMULATION_FPS: f64 = 60.0;
const SIMULATION_FRAME_MS: f64 = 1000.0 / SIMULATION_FPS;
const HUE_ROTATION_PERIOD_MS: f64 = 3000.0;

fn random_hue() -> u8 {
    (Math::random() * 256.0) as u8
}

fn mix_colors(hues: &[u8]) -> u8 {
    if hues.is_empty() {
        return 0;
    }

    let (sum_sin, sum_cos) = hues
        .iter()
        .map(|&hue| (hue as f64 * 360.0 / 255.0).to_radians())
        .fold((0.0, 0.0), |acc, h_rad| {
            (acc.0 + h_rad.sin(), acc.1 + h_rad.cos())
        });
    let avg_hue_degrees = sum_sin.atan2(sum_cos).to_degrees().rem_euclid(360.0);
    (avg_hue_degrees * 255.0 / 360.0) as u8
}

/// Convert HSL hue (0-255) to RGB
/// Using full saturation (100%) and lightness (50%)
fn hue_to_rgb(hue: u8) -> (u8, u8, u8) {
    let h = hue as f64 / 255.0 * 6.0;
    let x = 1.0 - (h % 2.0 - 1.0).abs();
    
    let (r, g, b) = match h as u32 {
        0 => (1.0, x, 0.0),
        1 => (x, 1.0, 0.0),
        2 => (0.0, 1.0, x),
        3 => (0.0, x, 1.0),
        4 => (x, 0.0, 1.0),
        _ => (1.0, 0.0, x),
    };
    
    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive { hue: u8 },
}

struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    next_cells: Vec<Cell>,
    neighbor_hues_buffer: Vec<u8>,
}

impl Universe {
    fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut u = Self {
            width,
            height,
            cells: vec![Cell::Dead; size],
            next_cells: vec![Cell::Dead; size],
            neighbor_hues_buffer: Vec::with_capacity(8),
        };
        u.randomize();
        u
    }

    fn randomize(&mut self) {
        for c in &mut self.cells {
            *c = if Math::random() < 0.5 {
                Cell::Alive { hue: random_hue() }
            } else {
                Cell::Dead
            };
        }
    }

    #[inline(always)]
    fn index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn count_neighbors_and_get_hues(&mut self, row: u32, col: u32) -> u8 {
        self.neighbor_hues_buffer.clear();

        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if col == 0 { self.width - 1 } else { col - 1 };
        let east = if col == self.width - 1 { 0 } else { col + 1 };

        let neighbors = [
            (north, west),
            (north, col),
            (north, east),
            (row, west),
            (row, east),
            (south, west),
            (south, col),
            (south, east),
        ];

        for (nr, nc) in neighbors {
            let idx = self.index(nr, nc);
            if let Cell::Alive { hue } = self.cells[idx] {
                self.neighbor_hues_buffer.push(hue);
            }
        }

        self.neighbor_hues_buffer.len() as u8
    }

    fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.index(row, col);
                let cell = self.cells[idx];
                let neighbor_count = self.count_neighbors_and_get_hues(row, col);

                self.next_cells[idx] = match (cell, neighbor_count) {
                    (Cell::Alive { .. }, x) if x < 2 => Cell::Dead,
                    (Cell::Alive { hue }, 2) | (Cell::Alive { hue }, 3) => Cell::Alive { hue },
                    (Cell::Alive { .. }, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => {
                        let mixed_hue = mix_colors(&self.neighbor_hues_buffer);
                        Cell::Alive { hue: mixed_hue }
                    }
                    (otherwise, _) => otherwise,
                };
            }
        }
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    fn set_alive_block(&mut self, center_row: i32, center_col: i32, half: i32, hue: u8) {
        let h = self.height as i32;
        let w = self.width as i32;
        for dr in -half..=half {
            for dc in -half..=half {
                let r = (center_row + dr).rem_euclid(h) as u32;
                let c = (center_col + dc).rem_euclid(w) as u32;
                let idx = self.index(r, c);
                self.cells[idx] = Cell::Alive { hue };
            }
        }
    }
}

struct AppState {
    window: Window,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    universe: Universe,
    cursor_row: i32,
    cursor_col: i32,
    last_frame_time: f64,
    cursor_active: bool,
    // Reusable pixel buffer for ImageData (avoids allocation each frame)
    pixel_buffer: Vec<u8>,
    canvas_width: u32,
    canvas_height: u32,
}

impl AppState {
    fn new() -> Result<Self, JsValue> {
        utils::set_panic_hook();

        let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
        let document = window
            .document()
            .ok_or_else(|| JsValue::from_str("no document"))?;

        let canvas = document
            .get_element_by_id("canvas")
            .and_then(|e| e.dyn_into::<HtmlCanvasElement>().ok())
            .ok_or_else(|| JsValue::from_str("canvas element not found"))?;

        let ctx = canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("no 2d context"))?
            .dyn_into::<CanvasRenderingContext2d>()?;

        // Disable image smoothing for sharp pixel scaling
        ctx.set_image_smoothing_enabled(false);

        let mut state = Self {
            window,
            canvas,
            ctx,
            universe: Universe::new(1, 1),
            cursor_row: 0,
            cursor_col: 0,
            last_frame_time: 0.0,
            cursor_active: false,
            pixel_buffer: Vec::new(),
            canvas_width: 0,
            canvas_height: 0,
        };

        state.resize_canvas_and_universe();
        Ok(state)
    }

    fn get_current_time(&self) -> f64 {
        self.window.performance().unwrap().now()
    }

    fn resize_canvas_and_universe(&mut self) {
        let width = self.window.inner_width().unwrap().as_f64().unwrap();
        let height = self.window.inner_height().unwrap().as_f64().unwrap();

        let dpr = self.window.device_pixel_ratio();
        let css_w = width;
        let css_h = height;
        let element = self.canvas.dyn_ref::<web_sys::Element>().unwrap();
        element
            .set_attribute(
                "style",
                &format!(
                    "position:fixed;inset:0;width:{}px;height:{}px;image-rendering:pixelated",
                    css_w, css_h
                ),
            )
            .ok();
        
        self.canvas_width = (css_w * dpr) as u32;
        self.canvas_height = (css_h * dpr) as u32;
        self.canvas.set_width(self.canvas_width);
        self.canvas.set_height(self.canvas_height);
        
        // Disable image smoothing after resize
        self.ctx.set_image_smoothing_enabled(false);
        
        // Clear canvas
        self.ctx.set_fill_style_str("black");
        self.ctx.fill_rect(0.0, 0.0, self.canvas_width as f64, self.canvas_height as f64);

        let cols = (self.canvas_width / CELL_SIZE).max(1);
        let rows = (self.canvas_height / CELL_SIZE).max(1);
        self.universe = Universe::new(cols, rows);
        
        // Allocate pixel buffer for the universe size (1 pixel per cell)
        // We'll draw at universe resolution and let CSS scale it up
        let buffer_size = (cols * rows * 4) as usize;
        self.pixel_buffer = vec![0u8; buffer_size];
    }

    fn draw_scaled(&mut self) {
        let grid_width = self.universe.width;
        let grid_height = self.universe.height;
        let cell_w = CELL_SIZE;
        let cell_h = CELL_SIZE;
        
        // Fill pixel buffer at full canvas resolution
        let canvas_w = self.canvas_width as usize;
        let canvas_h = self.canvas_height as usize;
        
        // Resize buffer if needed
        let needed_size = canvas_w * canvas_h * 4;
        if self.pixel_buffer.len() != needed_size {
            self.pixel_buffer.resize(needed_size, 0);
        }
        
        // Fill with black first (for dead cells)
        for chunk in self.pixel_buffer.chunks_exact_mut(4) {
            chunk[0] = 0;
            chunk[1] = 0;
            chunk[2] = 0;
            chunk[3] = 255;
        }
        
        // Draw each cell as a CELL_SIZE × CELL_SIZE block
        for row in 0..grid_height {
            for col in 0..grid_width {
                let cell_idx = self.universe.index(row, col);
                
                if let Cell::Alive { hue } = self.universe.cells[cell_idx] {
                    let (r, g, b) = hue_to_rgb(hue);
                    
                    let start_x = (col * cell_w) as usize;
                    let start_y = (row * cell_h) as usize;
                    
                    for py in 0..cell_h as usize {
                        let y = start_y + py;
                        if y >= canvas_h {
                            break;
                        }
                        
                        for px in 0..cell_w as usize {
                            let x = start_x + px;
                            if x >= canvas_w {
                                break;
                            }
                            
                            let pixel_idx = (y * canvas_w + x) * 4;
                            self.pixel_buffer[pixel_idx] = r;
                            self.pixel_buffer[pixel_idx + 1] = g;
                            self.pixel_buffer[pixel_idx + 2] = b;
                            self.pixel_buffer[pixel_idx + 3] = 255;
                        }
                    }
                }
            }
        }
        
        // Single putImageData call with full canvas
        if let Ok(image_data) = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.pixel_buffer),
            self.canvas_width,
            self.canvas_height,
        ) {
            self.ctx.put_image_data(&image_data, 0.0, 0.0).ok();
        }
    }
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let state = AppState::new()?;
    let state_rc = Rc::new(RefCell::new(state));

    let canvas = state_rc.borrow().canvas.clone();
    let window = state_rc.borrow().window.clone();
    let document = window.document().unwrap();

    // Mouse move handler
    let state_for_mouse = state_rc.clone();
    let canvas_for_mouse = canvas.clone();
    let window_for_mouse = window.clone();
    let mouse_closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let rect = canvas_for_mouse.get_bounding_client_rect();
        let dpr = window_for_mouse.device_pixel_ratio();
        let x = (event.client_x() as f64 - rect.left()) * dpr;
        let y = (event.client_y() as f64 - rect.top()) * dpr;
        let mut s = state_for_mouse.borrow_mut();
        s.cursor_col = (x / CELL_SIZE as f64) as i32;
        s.cursor_row = (y / CELL_SIZE as f64) as i32;
        s.cursor_active = true;
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("mousemove", mouse_closure.as_ref().unchecked_ref())?;
    mouse_closure.forget();

    // Touch move handler
    let state_for_touch_move = state_rc.clone();
    let canvas_for_touch_move = canvas.clone();
    let window_for_touch_move = window.clone();
    let touch_move_closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
        event.prevent_default();
        if let Some(touch) = event.touches().get(0) {
            let rect = canvas_for_touch_move.get_bounding_client_rect();
            let dpr = window_for_touch_move.device_pixel_ratio();
            let x = (touch.client_x() as f64 - rect.left()) * dpr;
            let y = (touch.client_y() as f64 - rect.top()) * dpr;
            let mut s = state_for_touch_move.borrow_mut();
            s.cursor_col = (x / CELL_SIZE as f64) as i32;
            s.cursor_row = (y / CELL_SIZE as f64) as i32;
            s.cursor_active = true;
        }
    }) as Box<dyn FnMut(_)>);

    document
        .add_event_listener_with_callback("touchmove", touch_move_closure.as_ref().unchecked_ref())?;
    touch_move_closure.forget();

    // Touch start handler
    let state_for_touch_start = state_rc.clone();
    let canvas_for_touch_start = canvas.clone();
    let window_for_touch_start = window.clone();
    let touch_start_closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
        event.prevent_default();
        if let Some(touch) = event.touches().get(0) {
            let rect = canvas_for_touch_start.get_bounding_client_rect();
            let dpr = window_for_touch_start.device_pixel_ratio();
            let x = (touch.client_x() as f64 - rect.left()) * dpr;
            let y = (touch.client_y() as f64 - rect.top()) * dpr;
            let mut s = state_for_touch_start.borrow_mut();
            s.cursor_col = (x / CELL_SIZE as f64) as i32;
            s.cursor_row = (y / CELL_SIZE as f64) as i32;
            s.cursor_active = true;
        }
    }) as Box<dyn FnMut(_)>);

    document
        .add_event_listener_with_callback("touchstart", touch_start_closure.as_ref().unchecked_ref())?;
    touch_start_closure.forget();

    // Touch end handler
    let state_for_touch_end = state_rc.clone();
    let touch_end_closure = Closure::wrap(Box::new(move |_event: web_sys::TouchEvent| {
        let mut s = state_for_touch_end.borrow_mut();
        s.cursor_active = false;
    }) as Box<dyn FnMut(_)>);

    document
        .add_event_listener_with_callback("touchend", touch_end_closure.as_ref().unchecked_ref())?;
    touch_end_closure.forget();

    // Resize handler
    let state_for_resize = state_rc.clone();
    let resize_closure = Closure::wrap(Box::new(move || {
        let mut s = state_for_resize.borrow_mut();
        s.resize_canvas_and_universe();
    }) as Box<dyn FnMut()>);

    window.add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())?;
    resize_closure.forget();

    // Animation loop
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();
    let state_for_anim = state_rc.clone();
    let window_for_anim = window.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let current_time = state_for_anim.borrow().get_current_time();

        {
            let mut s = state_for_anim.borrow_mut();
            
            // Run simulation FIRST (throttled to 60 FPS max)
            // This way cursor-placed cells won't be immediately killed
            if current_time - s.last_frame_time >= SIMULATION_FRAME_MS {
                s.last_frame_time = current_time;
                s.universe.tick();
            }
            
            // Process cursor input AFTER tick for responsiveness
            // Cells placed here survive until the next tick
            if s.cursor_active {
                let cursor_row = s.cursor_row;
                let cursor_col = s.cursor_col;
                
                let hue = ((current_time % HUE_ROTATION_PERIOD_MS) / HUE_ROTATION_PERIOD_MS * 256.0) as u8;
                s.universe.set_alive_block(cursor_row, cursor_col, 2, hue);
            }

            // Draw every frame
            s.draw_scaled();
        }

        window_for_anim
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .ok();
    }) as Box<dyn FnMut()>));

    window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}
