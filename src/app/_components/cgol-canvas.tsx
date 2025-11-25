"use client";

import { useEffect, useRef, useCallback } from "react";

export function CgolCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const initializedRef = useRef(false);
  const cleanupRef = useRef<(() => void) | null>(null);

  const initializeWasm = useCallback(async () => {
    if (typeof window === "undefined") return;

    try {
      const canvas = canvasRef.current;
      if (!canvas || initializedRef.current) return;

      const cgolModule = await import("cgol");

      // Initialize WASM module
      const initFunction = cgolModule.default;
      if (initFunction && typeof initFunction === "function") {
        await initFunction();
      }

      // Start CGOL
      if (typeof cgolModule.start === "function") {
        cgolModule.start();
        initializedRef.current = true;

        const cleanupFn = (cgolModule as { cleanup?: () => void }).cleanup;
        if (typeof cleanupFn === "function") {
          cleanupRef.current = cleanupFn;
        }
      }
    } catch (error: unknown) {
      console.error("Failed to initialize CGOL WebAssembly module:", error);
    }
  }, []);

  useEffect(() => {
    // Initialize immediately without delay
    void initializeWasm();

    return () => {
      // Call cleanup if available from WASM module
      if (cleanupRef.current) {
        cleanupRef.current();
      }
      // Reset initialization state on unmount
      initializedRef.current = false;
    };
  }, [initializeWasm]);

  return (
    <canvas
      ref={canvasRef}
      id="canvas"
      className="fixed top-0 left-0 -z-10 h-screen w-screen"
      aria-hidden="true"
    />
  );
}
