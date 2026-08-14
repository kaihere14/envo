import { cn } from "@/lib/utils";
import React from "react";

/** Inline monospace token: file names, commands, flags. */
export const InlineCode = ({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) => {
  return (
    <code
      className={cn(
        "text-primary rounded bg-current/6 px-1 py-0.5 font-mono text-[0.85em]",
        className,
      )}
    >
      {children}
    </code>
  );
};
