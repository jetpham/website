import { type ReactNode } from "react";

interface FrostedBoxProps {
  label?: string;
  children: ReactNode;
  className?: string;
}

export function FrostedBox({
  label,
  children,
  className = "",
}: FrostedBoxProps) {
  return (
    <div
      className={`relative my-[calc(2ch-2px)] px-[calc(0.5ch-0.5px)] py-[1ch] ${className}`}
    >
      {/* Extended frosted glass backdrop with mask */}
      <div
        className="pointer-events-none absolute inset-0 h-[200%] bg-black/60 backdrop-blur-lg"
        style={{
          maskImage:
            "linear-gradient(to bottom, black 0% 50%, transparent 50% 100%)",
          WebkitMaskImage:
            "linear-gradient(to bottom, black 0% 50%, transparent 50% 100%)",
        }}
      />

      {/* Border */}
      <div className="absolute inset-0 border-2 border-white" />

      {/* Content */}
      <div className="relative z-10">
        {label && (
          <span className="absolute -top-[1ch] left-2 bg-transparent text-white">
            {label}
          </span>
        )}
        {children}
      </div>
    </div>
  );
}

