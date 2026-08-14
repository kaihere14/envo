import React from "react";
import {
  IconShieldLock,
  IconTopologyStar3,
  IconUsers,
} from "@tabler/icons-react";
import { Box } from "./box";
import { Subheading } from "./subheading";

const guarantees = [
  {
    title: "Zero-trust by design",
    description:
      "No server or relay can decrypt your secrets. That is a property of the math, not a promise in a policy document.",
    icon: (
      <IconShieldLock className="size-4 text-white drop-shadow-xl drop-shadow-black/40" />
    ),
    boxClassName:
      "bg-linear-to-b from-blue-400 to-blue-600 ring-offset-blue-500",
  },
  {
    title: "Per-recipient encryption",
    description:
      "No shared password to circulate or rotate. Access is tied to owning a private key, so it cannot be forwarded by accident.",
    icon: (
      <IconUsers className="size-4 text-white drop-shadow-xl drop-shadow-black/40" />
    ),
    boxClassName:
      "bg-linear-to-b from-emerald-400 to-emerald-600 ring-offset-emerald-500",
  },
  {
    title: "Decentralized",
    description:
      "Secrets live across Nostr relays instead of one company's box. No single server going down takes your access with it.",
    icon: (
      <IconTopologyStar3 className="size-4 text-white drop-shadow-xl drop-shadow-black/40" />
    ),
    boxClassName:
      "bg-linear-to-b from-violet-400 to-violet-600 ring-offset-violet-500",
  },
];

export const Guarantees = () => {
  return (
    <section id="guarantees" className="scroll-mt-8">
      <Subheading>What you actually get</Subheading>
      <div className="mt-6 grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3">
        {guarantees.map((item) => (
          <div key={item.title} className="flex flex-col gap-3">
            <div className="flex items-center gap-2">
              <Box className={item.boxClassName}>{item.icon}</Box>
              <p className="text-foreground text-sm font-medium text-balance">
                {item.title}
              </p>
            </div>
            <p className="text-foreground/70 text-sm text-pretty">
              {item.description}
            </p>
          </div>
        ))}
      </div>
    </section>
  );
};
