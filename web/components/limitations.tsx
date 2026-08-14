import React from "react";
import { Subheading } from "./subheading";

const limitations = [
  {
    title: "Beta, and unaudited",
    description:
      "It works, and it is early. No third-party security audit has been done yet, so weigh that against what you are about to put in it.",
  },
  {
    title: "No forward secrecy yet",
    description:
      "If a private key leaks, it can retroactively open secrets that were published to that key in the past. Fixing this is planned work, not something done.",
  },
  {
    title: "One writer per project",
    description:
      "For now only the project owner publishes updates for a tag. Teammates pull; they do not push back.",
  },
];

export const Limitations = () => {
  return (
    <section id="limitations" className="scroll-mt-8">
      <Subheading>Where it isn&apos;t there yet</Subheading>
      <div className="mt-4 flex flex-col gap-4">
        {limitations.map((item) => (
          <div key={item.title} className="flex gap-3">
            <span
              aria-hidden
              className="text-foreground/30 mt-2.5 size-1 shrink-0 rounded-full bg-current"
            />
            <p className="text-foreground/70 text-sm text-pretty">
              <span className="text-foreground font-medium">{item.title}.</span>{" "}
              {item.description}
            </p>
          </div>
        ))}
      </div>
    </section>
  );
};
