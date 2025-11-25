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
  return (
    <fieldset
      className={`mt-[2ch] border-2 border-white px-[calc(1.5ch-0.5px)] pb-[1ch] pt-0 ${className}`}
    >
      {label && (
        <legend className="-mx-[0.5ch] px-[0.5ch] text-white">
          {label}
        </legend>
      )}

      {children}
    </fieldset>
  );
}

