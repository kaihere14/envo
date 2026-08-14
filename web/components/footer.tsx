"use client";
import React from "react";
import Container from "./container";
import { motion } from "motion/react";
import { LinkPreview } from "./link-preview";
import { site } from "@/lib/site";

export const Footer = () => {
  return (
    <Container className="pb-10">
      <footer className="my-8 flex flex-col items-center gap-4">
        <PushTranscript />
        <div className="flex flex-col items-center gap-1.5">
          <p className="text-foreground/40 text-center text-sm text-balance">
            Open source, MIT licensed. Here&apos;s the{" "}
            <LinkPreview url={site.github}>code</LinkPreview> and the{" "}
            <LinkPreview url={site.releases}>releases</LinkPreview>.
          </p>
          <p className="text-foreground/40 text-center text-sm text-balance">
            Built in Rust, encrypted with NIP-44, stored on Nostr.
          </p>
        </div>
      </footer>
    </Container>
  );
};

/**
 * The four symbols the CLI prints, drawn as a miniature `envo push` run.
 * Same output vocabulary as the tool itself: `-` working, `✓` done.
 */
const PushTranscript = () => {
  const lines = [
    { symbol: "-", text: "encrypting .env for 3 recipients" },
    { symbol: "-", text: "publishing to 4 relays" },
    { symbol: "✓", text: "pushed my-project" },
  ];

  return (
    <motion.div
      initial="hidden"
      whileInView="visible"
      viewport={{ once: true, amount: 0.6 }}
      variants={{
        hidden: {},
        visible: { transition: { staggerChildren: 0.35 } },
      }}
      className="text-foreground/40 flex flex-col items-start gap-1 font-mono text-xs"
      aria-hidden
    >
      {lines.map((line) => (
        <motion.div
          key={line.text}
          variants={{
            hidden: { opacity: 0, y: 4 },
            visible: { opacity: 1, y: 0 },
          }}
          transition={{ duration: 0.4, ease: "easeOut" }}
          className="flex items-baseline gap-2"
        >
          <span className={line.symbol === "✓" ? "text-emerald-500" : ""}>
            {line.symbol}
          </span>
          <span>{line.text}</span>
        </motion.div>
      ))}
    </motion.div>
  );
};
