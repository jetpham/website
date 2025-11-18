mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use js_sys::Math;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window, OffscreenCanvas, OffscreenCanvasRenderingContext2d, ImageData};

const CELL_SIZE: u32 = 20;
const TARGET_FPS: f64 = 45.0;
const FRAME_DURATION_MS: f64 = 1000.0 / TARGET_FPS;

struct CellCache {
    ctx: OffscreenCanvasRenderingContext2d,
    cached_cells: std::collections::HashMap<u8, ImageData>,
}

impl CellCache {
    fn new() -> Result<Self, JsValue> {
        let offscreen_canvas = OffscreenCanvas::new(CELL_SIZE, CELL_SIZE)?;
        let context_options = js_sys::Object::new();
        js_sys::Reflect::set(&context_options, &"willReadFrequently".into(), &true.into())?;
        let ctx = offscreen_canvas
            .get_context_with_context_options("2d", &context_options)?
            .ok_or_else(|| JsValue::from_str("no 2d context"))?
            .dyn_into::<OffscreenCanvasRenderingContext2d>()?;
        
        Ok(Self {
            ctx,
            cached_cells: std::collections::HashMap::new(),
        })
    }
    
    fn get_or_create_cell(&mut self, hue: u8) -> Result<&ImageData, JsValue> {
        if !self.cached_cells.contains_key(&hue) {
            self.ctx.set_fill_style_str("#000");
            self.ctx.fill_rect(0.0, 0.0, CELL_SIZE as f64, CELL_SIZE as f64);
            
            let hue_degrees = hue as f64 * 360.0 / 255.0;
            let color_str = format!("hsl({}, 100%, 50%)", hue_degrees);
            self.ctx.set_fill_style_str(&color_str);
            self.ctx.fill_rect(0.0, 0.0, CELL_SIZE as f64, CELL_SIZE as f64);
            
            let image_data = self.ctx.get_image_data(0.0, 0.0, CELL_SIZE as f64, CELL_SIZE as f64)?;
            self.cached_cells.insert(hue, image_data);
        }
        
        Ok(self.cached_cells.get(&hue).unwrap())
    }
}


fn random_hue() -> u8 {
    (Math::random() * 256.0) as u8
}

fn mix_colors(hues: Vec<u8>) -> u8 {
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
}

impl Universe {
    fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut u = Self { 
            width, 
            height, 
            cells: vec![Cell::Dead; size],
            next_cells: vec![Cell::Dead; size],
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

    fn index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn get_neighbor_hues(&self, row: u32, col: u32) -> Vec<u8> {
        let mut hues = Vec::new();
        
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
            if let Cell::Alive { hue } = &self.cells[idx] {
                hues.push(*hue);
            }
        }
        
        hues
    }

    fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.index(row, col);
                let cell = self.cells[idx];
                let neighbor_hues = self.get_neighbor_hues(row, col);
                let neighbor_count = neighbor_hues.len() as u8;
                self.next_cells[idx] = match (cell, neighbor_count) {
                    (Cell::Alive { .. }, x) if x < 2 => Cell::Dead,
                    (Cell::Alive { hue }, 2) | (Cell::Alive { hue }, 3) => Cell::Alive { hue },
                    (Cell::Alive { .. }, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => {
                        let mixed_hue = mix_colors(neighbor_hues);
                        Cell::Alive { hue: mixed_hue }
                    },
                    (otherwise, _) => otherwise,
                };
            }
        }
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    fn set_alive_block(&mut self, center_row: i32, center_col: i32, half: i32) {
        let h = self.height as i32;
        let w = self.width as i32;
        for dr in -half..=half {
            for dc in -half..=half {
                let r = (center_row + dr).rem_euclid(h) as u32;
                let c = (center_col + dc).rem_euclid(w) as u32;
                let idx = self.index(r, c);
                self.cells[idx] = Cell::Alive { hue: random_hue() };
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
    cell_cache: CellCache,
}

impl AppState {
    fn new() -> Result<Self, JsValue> {
        utils::set_panic_hook();

        let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
        let document = window.document().ok_or_else(|| JsValue::from_str("no document"))?;

        let canvas = document
            .get_element_by_id("canvas")
            .and_then(|e| e.dyn_into::<HtmlCanvasElement>().ok())
            .ok_or_else(|| JsValue::from_str("canvas element not found"))?;

        let ctx = canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("no 2d context"))?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let mut state = Self {
            window,
            canvas,
            ctx,
            universe: Universe::new(1, 1),
            cursor_row: 0,
            cursor_col: 0,
            last_frame_time: 0.0,
            cursor_active: false,
            cell_cache: CellCache::new()?,
        };

        state.resize_canvas_and_universe();
        Ok(state)
    }

    fn get_current_time(&self) -> f64 {
        self.window
            .performance()
            .unwrap()
            .now()
    }

    fn resize_canvas_and_universe(&mut self) {
        let width = self.window.inner_width().unwrap().as_f64().unwrap();
        let height = self.window.inner_height().unwrap().as_f64().unwrap();

        let dpr = self.window.device_pixel_ratio();
        let css_w = width;
        let css_h = height;
        let style = self.canvas.style();
        style.set_property("position", "fixed").ok();
        style.set_property("inset", "0").ok();
        style.set_property("width", &format!("{}px", css_w)).ok();
        style.set_property("height", &format!("{}px", css_h)).ok();
        self.canvas.set_width((css_w * dpr) as u32);
        self.canvas.set_height((css_h * dpr) as u32);
        self.ctx.set_image_smoothing_enabled(false);
        self.ctx.set_fill_style_str("black");
        self.ctx.fill_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);

        let cols = (self.canvas.width() / CELL_SIZE).max(1);
        let rows = (self.canvas.height() / CELL_SIZE).max(1);
        self.universe = Universe::new(cols, rows);
    }

    fn draw(&mut self) {
        self.ctx.set_fill_style_str("#000");
        self.ctx.fill_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64);
        
        let cell_w = CELL_SIZE as f64;
        let cell_h = CELL_SIZE as f64;

        for row in 0..self.universe.height {
            for col in 0..self.universe.width {
                let idx = self.universe.index(row, col);
                if let Cell::Alive { hue } = self.universe.cells[idx] {
                    let x = col as f64 * cell_w;
                    let y = row as f64 * cell_h;
                    
                    if let Ok(image_data) = self.cell_cache.get_or_create_cell(hue) {
                        self.ctx.put_image_data(image_data, x, y).ok();
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let state = AppState::new()?;

    {
        let canvas = state.canvas.clone();
        let window = state.window.clone();
        let state_rc = Rc::new(RefCell::new(state));
        let state_for_mouse = state_rc.clone();
        let mouse_closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let rect = canvas.get_bounding_client_rect();
            let dpr = window.device_pixel_ratio();
            let x = (event.client_x() as f64 - rect.left()) * dpr;
            let y = (event.client_y() as f64 - rect.top()) * dpr;
            let mut s = state_for_mouse.borrow_mut();
            s.cursor_col = (x / CELL_SIZE as f64) as i32;
            s.cursor_row = (y / CELL_SIZE as f64) as i32;
            s.cursor_active = true;
        }) as Box<dyn FnMut(_)>);
        state_rc
            .borrow()
            .window
            .document()
            .unwrap()
            .add_event_listener_with_callback("mousemove", mouse_closure.as_ref().unchecked_ref())?;
        mouse_closure.forget();
        
        let canvas_touch_move = state_rc.borrow().canvas.clone();
        let window_touch_move = state_rc.borrow().window.clone();
        let state_for_touch_move = state_rc.clone();
        
        let touch_move_closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            event.prevent_default();
            if let Some(touch) = event.touches().get(0) {
                let rect = canvas_touch_move.get_bounding_client_rect();
                let dpr = window_touch_move.device_pixel_ratio();
                let x = (touch.client_x() as f64 - rect.left()) * dpr;
                let y = (touch.client_y() as f64 - rect.top()) * dpr;
                let mut s = state_for_touch_move.borrow_mut();
                s.cursor_col = (x / CELL_SIZE as f64) as i32;
                s.cursor_row = (y / CELL_SIZE as f64) as i32;
                s.cursor_active = true;
            }
        }) as Box<dyn FnMut(_)>);
        
        let canvas_touch_start = state_rc.borrow().canvas.clone();
        let window_touch_start = state_rc.borrow().window.clone();
        let state_for_touch_start = state_rc.clone();
        
        let touch_start_closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
            event.prevent_default();
            if let Some(touch) = event.touches().get(0) {
                let rect = canvas_touch_start.get_bounding_client_rect();
                let dpr = window_touch_start.device_pixel_ratio();
                let x = (touch.client_x() as f64 - rect.left()) * dpr;
                let y = (touch.client_y() as f64 - rect.top()) * dpr;
                let mut s = state_for_touch_start.borrow_mut();
                s.cursor_col = (x / CELL_SIZE as f64) as i32;
                s.cursor_row = (y / CELL_SIZE as f64) as i32;
                s.cursor_active = true;
            }
        }) as Box<dyn FnMut(_)>);
        
        state_rc
            .borrow()
            .window
            .document()
            .unwrap()
            .add_event_listener_with_callback("touchmove", touch_move_closure.as_ref().unchecked_ref())?;
        state_rc
            .borrow()
            .window
            .document()
            .unwrap()
            .add_event_listener_with_callback("touchstart", touch_start_closure.as_ref().unchecked_ref())?;
        
        touch_move_closure.forget();
        touch_start_closure.forget();

        let state_for_touch_end = state_rc.clone();
        let touch_end_closure = Closure::wrap(Box::new(move |_event: web_sys::TouchEvent| {
            let mut s = state_for_touch_end.borrow_mut();
            s.cursor_active = false;
        }) as Box<dyn FnMut(_)>);
        
        state_rc
            .borrow()
            .window
            .document()
            .unwrap()
            .add_event_listener_with_callback("touchend", touch_end_closure.as_ref().unchecked_ref())?;
        touch_end_closure.forget();

        let state_for_resize = state_rc.clone();
        let resize_closure = Closure::wrap(Box::new(move || {
            let mut s = state_for_resize.borrow_mut();
            s.resize_canvas_and_universe();
        }) as Box<dyn FnMut()>);
        state_rc
            .borrow()
            .window
            .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())?;
        resize_closure.forget();

        let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
        let g = f.clone();
        let state_for_anim = state_rc.clone();
        let window_for_anim = state_rc.borrow().window.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let current_time = state_for_anim.borrow().get_current_time();
            let mut should_update = false;
            
            {
                let mut s = state_for_anim.borrow_mut();
                if current_time - s.last_frame_time >= FRAME_DURATION_MS {
                    should_update = true;
                    s.last_frame_time = current_time;
                }
                
                if should_update {
                    if s.cursor_active {
                        let cursor_row = s.cursor_row;
                        let cursor_col = s.cursor_col;
                        s.universe.set_alive_block(cursor_row, cursor_col, 2);
                    }
                    s.universe.tick();
                }
                
                s.draw();
            }
            
            window_for_anim
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .ok();
        }) as Box<dyn FnMut()>));

        state_rc
            .borrow()
            .window
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;
    }

    Ok(())
}
