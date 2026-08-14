import React from "react";
import {
  IconBroadcast,
  IconFingerprint,
  IconLock,
  IconLockOpen,
} from "@tabler/icons-react";
import { Box } from "./box";
import { Subheading } from "./subheading";

const steps = [
  {
    title: "Everyone brings their own key",
    description:
      "Each teammate has a keypair of their own, a cryptographic identity rather than a password sitting in a vault somewhere. Nobody has to be handed anything to be given access.",
    icon: (
      <IconFingerprint className="size-4 text-white drop-shadow-xl drop-shadow-black/40" />
    ),
    boxClassName:
      "bg-linear-to-b from-amber-400 to-amber-600 ring-offset-amber-500",
  },
  {
    title: "Secrets are encrypted once per person",
    description:
      "Your machine encrypts the file separately for every teammate you trust, using a shared secret derived from their key and yours. There is no single team key that could leak and open everything.",
    icon: (
      <IconLock className="size-4 text-white drop-shadow-xl drop-shadow-black/40" />
    ),
    boxClassName:
      "bg-linear-to-b from-blue-400 to-blue-600 ring-offset-blue-500",
  },
  {
    title: "Only the sealed copies are published",
    description:
      "The encrypted bundle goes out to Nostr relays, which are decentralized storage rather than one company's server. A relay stores it and hands it back, and never sees anything but unreadable bytes.",
    icon: (
      <IconBroadcast className="size-4 text-white drop-shadow-xl drop-shadow-black/40" />
    ),
    boxClassName:
      "bg-linear-to-b from-violet-400 to-violet-600 ring-offset-violet-500",
  },
  {
    title: "You open your copy, and only yours",
    description:
      "Pulling fetches the copy addressed to you and decrypts it on your machine with your private key, which never leaves it. Nobody in the middle was ever in a position to read along.",
    icon: (
      <IconLockOpen className="size-4 text-white drop-shadow-xl drop-shadow-black/40" />
    ),
    boxClassName:
      "bg-linear-to-b from-emerald-400 to-emerald-600 ring-offset-emerald-500",
  },
];

export const HowItWorks = () => {
  return (
    <section id="how-it-works" className="scroll-mt-8">
      <Subheading>How it works</Subheading>
      <div className="mt-4 flex flex-col gap-6">
        {steps.map((item, index) => (
          <div key={item.title} className="flex flex-col">
            <div className="flex flex-row items-center gap-2">
              <Box className={item.boxClassName}>{item.icon}</Box>
              <p className="text-foreground/40 shrink-0 font-mono text-sm">
                {String(index + 1).padStart(2, "0")}
              </p>
              <p className="text-foreground font-medium text-balance">
                {item.title}
              </p>
            </div>
            <p className="text-foreground/70 mt-2 text-sm text-pretty">
              {item.description}
            </p>
          </div>
        ))}
      </div>
    </section>
  );
};
