import { type ReactNode } from "react";

interface BorderedBoxProps {
  label?: string;
  children: ReactNode;
  className?: string;
}

export function BorderedBox({
  label,
  children,
  className = "",
}: BorderedBoxProps) {
  // Generate unique ID for this instance to avoid conflicts
  const maskId = `borderMask-${Math.random().toString(36).substring(2, 11)}`;

  // Calculate SVG mask values - approximate 1ch = 16px for IBM VGA font
  const chToPx = 16;
  const maskX = 4 + chToPx; // 4px + 1ch
  const maskWidth = label ? label.length * chToPx : 0;

  return (
    <div
      className={`relative my-[calc(2ch-2px)] px-[calc(1.5ch-0.5px)] py-[1ch] ${className}`}
    >
      <div
        className="absolute inset-0 border-2 border-white"
        style={{
          maskImage: label ? `url(#${maskId})` : "none",
          WebkitMaskImage: label ? `url(#${maskId})` : "none",
        }}
      />

      {label && (
        <svg
          className="pointer-events-none absolute inset-0 h-full w-full"
          style={{ zIndex: 1 }}
        >
          <defs>
            <mask id={maskId}>
              <rect width="100%" height="100%" fill="white" />
              <rect
                x={maskX}
                y="-8"
                width={maskWidth}
                height="16"
                fill="black"
              />
            </mask>
          </defs>
        </svg>
      )}

      {label && (
        <span
          className="absolute -top-[1ch] bg-transparent text-white"
          style={{ zIndex: 2 }}
        >
          {label}
        </span>
      )}

      <div className="relative z-10">{children}</div>
    </div>
  );
}

