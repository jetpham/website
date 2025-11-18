import React from "react";
import Ansi from "./ansi";

interface HeaderProps {
  content: string;
  className?: string;
}

export default function Header({ content, className }: HeaderProps) {
  return (
    <div className={className}>
      <Ansi>{content}</Ansi>
    </div>
  );
}

