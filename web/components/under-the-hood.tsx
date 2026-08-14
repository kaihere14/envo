import React from "react";
import { Subheading } from "./subheading";
import { InlineCode } from "./code";
import { LinkPreview } from "./link-preview";

const stack = [
  { label: "Rust", detail: "single static binary" },
  { label: "NIP-44", detail: "encryption scheme" },
  { label: "nostr-sdk", detail: "relays and signing" },
  { label: "tokio", detail: "async runtime" },
];

export const UnderTheHood = () => {
  return (
    <section id="under-the-hood" className="scroll-mt-8">
      <Subheading>Under the hood</Subheading>
      <div className="text-foreground/70 pt-4 text-sm text-pretty">
        Encryption is{" "}
        <LinkPreview url="https://github.com/nostr-protocol/nips/blob/master/44.md">
          NIP-44
        </LinkPreview>
        , keyed by an ECDH shared secret derived from your secret key and each
        recipient&apos;s public key, on the same secp256k1 curve Bitcoin uses. The
        ciphertext is written once per recipient into a signed, addressable
        event, so the recipient map is the only copy that exists.
      </div>
      <div className="text-foreground/70 pt-4 text-sm text-pretty">
        Because the event is addressable, pushing the same tag replaces the
        previous version rather than piling up history. That is why there is no
        separate &ldquo;first publish&rdquo; command: the first{" "}
        <InlineCode>push</InlineCode> and the fiftieth are the same operation.
      </div>
      <div className="text-foreground/70 pt-4 text-pretty">
        A tag is just a public label, so anyone can publish an event under it
        and address it to you. <InlineCode>pull</InlineCode> therefore refuses
        to guess: you name the publisher once with{" "}
        <InlineCode>--owner</InlineCode>, it gets pinned locally, and every pull
        after that only accepts events signed by that key. Your own keypair
        lives in <InlineCode>~/.envo/keys.json</InlineCode>, owner-only, and
        only ever the public half is printed.
      </div>
      <div className="mt-6 flex flex-wrap gap-2">
        {stack.map((item) => (
          <span
            key={item.label}
            className="text-foreground/70 rounded-md border border-neutral-300 bg-white/60 px-2 py-1 font-mono text-xs dark:border-neutral-700 dark:bg-neutral-900/60"
          >
            {item.label}
            <span className="text-foreground/40"> · {item.detail}</span>
          </span>
        ))}
      </div>
    </section>
  );
};
