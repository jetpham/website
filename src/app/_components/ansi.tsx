import Anser, { type AnserJsonEntry } from "anser";
import { escapeCarriageReturn } from "escape-carriage";
import React, { memo, useMemo } from "react";

const colorMap: Record<string, string> = {
  "ansi-black": "text-[var(--black)]",
  "ansi-red": "text-[var(--red)]",
  "ansi-green": "text-[var(--green)]",
  "ansi-yellow": "text-[var(--brown)]",
  "ansi-blue": "text-[var(--blue)]",
  "ansi-magenta": "text-[var(--magenta)]",
  "ansi-cyan": "text-[var(--cyan)]",
  "ansi-white": "text-[var(--light-gray)]",
  "ansi-bright-black": "text-[var(--dark-gray)]",
  "ansi-bright-red": "text-[var(--light-red)]",
  "ansi-bright-green": "text-[var(--light-green)]",
  "ansi-bright-yellow": "text-[var(--yellow)]",
  "ansi-bright-blue": "text-[var(--light-blue)]",
  "ansi-bright-magenta": "text-[var(--light-magenta)]",
  "ansi-bright-cyan": "text-[var(--light-cyan)]",
  "ansi-bright-white": "text-[var(--white)]",
} as const;

const bgColorMap: Record<string, string> = {
  "ansi-black": "bg-transparent",
  "ansi-red": "bg-[var(--red)]",
  "ansi-green": "bg-[var(--green)]",
  "ansi-yellow": "bg-[var(--brown)]",
  "ansi-blue": "bg-[var(--blue)]",
  "ansi-magenta": "bg-[var(--magenta)]",
  "ansi-cyan": "bg-[var(--cyan)]",
  "ansi-white": "bg-[var(--light-gray)]",
  "ansi-bright-black": "bg-[var(--dark-gray)]",
  "ansi-bright-red": "bg-[var(--light-red)]",
  "ansi-bright-green": "bg-[var(--light-green)]",
  "ansi-bright-yellow": "bg-[var(--yellow)]",
  "ansi-bright-blue": "bg-[var(--light-blue)]",
  "ansi-bright-magenta": "bg-[var(--light-magenta)]",
  "ansi-bright-cyan": "bg-[var(--light-cyan)]",
  "ansi-bright-white": "bg-[var(--white)]",
} as const;

const decorationMap: Record<string, string> = {
  bold: "font-bold",
  dim: "opacity-50",
  italic: "italic",
  hidden: "invisible",
  strikethrough: "line-through",
  underline: "underline",
  blink: "animate-pulse",
} as const;

function fixBackspace(txt: string): string {
  let tmp = txt;
  do {
    txt = tmp;
    tmp = txt.replace(/[^\n]\x08/gm, "");
  } while (tmp.length < txt.length);
  return txt;
}

function createClass(bundle: AnserJsonEntry): string | null {
  const classes: string[] = [];

  if (bundle.bg && bgColorMap[bundle.bg]) {
    classes.push(bgColorMap[bundle.bg]!);
  }
  if (bundle.fg && colorMap[bundle.fg]) {
    classes.push(colorMap[bundle.fg]!);
  }
  if (bundle.decoration && decorationMap[bundle.decoration]) {
    classes.push(decorationMap[bundle.decoration]!);
  }
  return classes.length ? classes.join(" ") : null;
}

interface Props {
  children?: string;
  className?: string;
}

const Ansi = memo(function Ansi({ className, children = "" }: Props) {
  const bundles = useMemo(() => {
    const input = escapeCarriageReturn(fixBackspace(children));
    return Anser.ansiToJson(input, {
      json: true,
      remove_empty: true,
      use_classes: true,
    });
  }, [children]);

  const renderedContent = useMemo(
    () =>
      bundles.map((bundle, key) => {
        const bundleClassName = createClass(bundle);
        return (
          <span key={key} className={bundleClassName ?? undefined}>
            {bundle.content}
          </span>
        );
      }),
    [bundles],
  );

  return (
    <div className="flex justify-center">
      <pre className={className ?? ""} style={{ textAlign: "left" }}>
        <code>{renderedContent}</code>
      </pre>
    </div>
  );
});

export default Ansi;
