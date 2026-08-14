import React from "react";
import {
  IconCloudDownload,
  IconCloudUpload,
  IconKey,
} from "@tabler/icons-react";
import { Box } from "./box";
import { Subheading } from "./subheading";
import { InlineCode } from "./code";

const commands = [
  {
    command: "envo keygen",
    description: "Create the Nostr identity that is your access.",
    icon: (
      <IconKey className="size-4 text-white drop-shadow-xl drop-shadow-black/40" />
    ),
    boxClassName:
      "bg-linear-to-b from-amber-400 to-amber-600 ring-offset-amber-500",
  },
  {
    command: "envo push <tag>",
    description:
      "Encrypt this project's .env for your team and publish it. Same command the first time and every time after.",
    icon: (
      <IconCloudUpload className="size-4 text-white drop-shadow-xl drop-shadow-black/40" />
    ),
    boxClassName:
      "bg-linear-to-b from-blue-400 to-blue-600 ring-offset-blue-500",
  },
  {
    command: "envo pull <tag>",
    description: "Fetch your copy, decrypt it locally and write .env.",
    icon: (
      <IconCloudDownload className="size-4 text-white drop-shadow-xl drop-shadow-black/40" />
    ),
    boxClassName:
      "bg-linear-to-b from-emerald-400 to-emerald-600 ring-offset-emerald-500",
  },
];

export const Commands = () => {
  return (
    <section id="commands" className="scroll-mt-8">
      <Subheading>The whole CLI</Subheading>
      <div className="mt-4 flex flex-col gap-6 md:gap-4">
        {commands.map((item) => (
          <div
            key={item.command}
            className="flex flex-col items-start gap-1 md:flex-row md:items-center md:gap-2"
          >
            <Box className={`mr-4 ${item.boxClassName}`}>{item.icon}</Box>
            <p className="text-foreground shrink-0 font-mono text-sm font-medium">
              {item.command}
            </p>
            <div className="hidden size-1 shrink-0 rounded-full bg-neutral-200 md:block"></div>
            <p className="text-foreground/70 text-pretty">
              {item.description}
            </p>
          </div>
        ))}
      </div>
      <p className="text-foreground/70 mt-6 text-sm text-pretty">
        That is the entire surface. There is no command for adding a teammate:
        put their public key in <InlineCode>.env-share</InlineCode> and run{" "}
        <InlineCode>envo push</InlineCode> again, which re-encrypts for whoever
        is on the list at that moment.
      </p>
    </section>
  );
};
