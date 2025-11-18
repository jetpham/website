"use client";

import { useEffect, useRef } from "react";

export function CgolCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const initializedRef = useRef(false);

  useEffect(() => {
    if (typeof window === "undefined") return;

    const initializeWasm = async () => {
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
        }
      } catch (error: unknown) {
        console.error("Failed to initialize CGOL WebAssembly module:", error);
      }
    };

    const timeoutId = setTimeout(() => {
      void initializeWasm();
    }, 100);

    return () => clearTimeout(timeoutId);
  }, []);

  return (
    <canvas
      ref={canvasRef}
      id="canvas"
      className="fixed top-0 left-0 w-screen h-screen -z-10"
    />
  );
}

