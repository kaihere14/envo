import React from "react";
import { LinkPreview } from "./link-preview";
import { InlineCode } from "./code";

export const Header = () => {
  return (
    <div>
      <p className="text-foreground pt-6 text-xl font-medium text-pretty md:text-2xl">
        Share <InlineCode className="text-[0.85em]">.env</InlineCode> secrets
        with your team, and keep them unreadable to everyone else, even to a
        server that has been completely taken over.
      </p>
      <div className="text-foreground pt-4 text-base">
        envo is a command line tool that syncs your{" "}
        <InlineCode>.env</InlineCode> files across a team. Every secret is
        encrypted on your machine, once for each teammate you trust, before it
        ever leaves it. What gets stored is ciphertext nobody but the intended
        recipient can open.
      </div>
      <div className="text-foreground pt-4 text-base">
        Every other option asks you to trust somebody. Doppler and Infisical
        ask you to trust their company. A password manager asks you to trust a
        shared vault. Pasting into Slack asks you to trust Slack. If any of
        those get breached or served a subpoena, your secrets go with them.
      </div>
      <div className="text-foreground pt-4 text-base">
        envo removes the trust requirement instead of asking you to extend it.
        There is no server that could decrypt your secrets if it wanted to,
        because there is no server that ever holds the keys. Data lives on{" "}
        <LinkPreview url="https://nostr.com">Nostr</LinkPreview> relays, and a
        relay only ever sees noise.
      </div>
    </div>
  );
};
