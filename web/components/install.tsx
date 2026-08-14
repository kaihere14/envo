"use client";

import React, { useEffect, useState } from "react";
import { createPortal } from "react-dom";
import { AnimatePresence, motion } from "motion/react";
import Link from "next/link";
import { IconCheck, IconCopy } from "@tabler/icons-react";
import { SPRING_CONFIG } from "@/lib/motion-config";
import { site } from "@/lib/site";
import { cn } from "@/lib/utils";
import { Subheading } from "./subheading";
import { InlineCode } from "./code";

export const Install = () => {
  const [copied, setCopied] = useState(false);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(site.installCommand);
      setCopied(true);
      setTimeout(() => setCopied(false), 3000);
    } catch (error) {
      console.error("Failed to copy install command", error);
    }
  };

  const toast = (
    <AnimatePresence mode="wait">
      {copied ? <CopyAnimation key="copy-command-toast" /> : null}
    </AnimatePresence>
  );

  return (
    <section id="install" className="scroll-mt-8">
      <Subheading>Install</Subheading>
      {mounted ? createPortal(toast, document.body) : null}
      <button
        type="button"
        onClick={handleCopy}
        aria-label="Copy the install command to clipboard"
        className="group mt-4 flex w-full cursor-pointer items-center gap-2 rounded-lg border border-neutral-300 bg-white/60 p-2.5 text-left shadow-sm transition-colors hover:bg-white dark:border-neutral-700 dark:bg-neutral-900/60"
      >
        <span className="text-foreground/40 shrink-0 font-mono text-sm select-none">
          $
        </span>
        <code className="text-foreground min-w-0 flex-1 font-mono text-xs break-all whitespace-pre-wrap">
          {site.installCommand}
        </code>
        {copied ? (
          <IconCheck className="text-foreground/70 size-4 shrink-0" />
        ) : (
          <IconCopy className="text-foreground/40 group-hover:text-foreground/70 size-4 shrink-0 transition-colors" />
        )}
      </button>
      <p className="text-foreground/70 mt-4 text-sm text-pretty">
        Detects your OS and CPU, downloads the matching binary from the latest
        release, verifies its SHA-256 and installs it to{" "}
        <InlineCode>~/.local/bin</InlineCode>. Set{" "}
        <InlineCode>ENVO_VERSION</InlineCode> to pin a release tag, or{" "}
        <InlineCode>ENVO_INSTALL_DIR</InlineCode> to install it somewhere else.
      </p>
      <p className="text-foreground/70 mt-2 text-sm text-pretty">
        Prebuilt for Linux x86_64, macOS on Intel and Apple Silicon, and Windows
        x86_64. On Windows use Git Bash or WSL, or grab the{" "}
        <InlineCode>.zip</InlineCode> from the{" "}
        <Link
          href={site.releases}
          target="_blank"
          rel="noopener noreferrer"
          className="underline decoration-neutral-400 decoration-dotted underline-offset-[0.2em]"
        >
          releases page
        </Link>
        .
      </p>
    </section>
  );
};

const CopyAnimation = () => {
  return (
    <motion.div
      initial={{
        opacity: 0,
        scale: 0.8,
        filter: "blur(10px)",
      }}
      animate={{
        opacity: 1,
        scale: 1,
        filter: "blur(0px)",
      }}
      exit={{
        opacity: 0,
        scale: 0.8,
        filter: "blur(10px)",
      }}
      transition={SPRING_CONFIG}
      className={cn(
        "pointer-events-none fixed inset-x-0 bottom-20 z-200 mx-auto flex w-fit items-center justify-center gap-2 rounded-lg bg-linear-to-b from-blue-400 to-blue-600 p-4 text-center text-white shadow-lg ring-1 shadow-black/10 ring-white/50 ring-offset-2 ring-offset-blue-500 ring-inset",
      )}
    >
      <TerminalIcon /> Install command copied to clipboard
    </motion.div>
  );
};

const TerminalIcon = () => {
  return (
    <motion.svg
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2.5"
      strokeLinecap="round"
      strokeLinejoin="round"
      className="size-4 perspective-distant"
      initial={{
        scale: 0.8,
      }}
      animate={{
        scale: [0.8, 1, 1.2, 1],
      }}
      transition={{
        duration: 0.3,
        delay: 0.5,
      }}
    >
      <motion.path
        d="M5 7l5 5l-5 5"
        initial={{ pathLength: 0 }}
        animate={{ pathLength: 1 }}
        transition={{ duration: 0.4, delay: 0.2 }}
      />
      <motion.path
        d="M13 17l6 0"
        initial={{ pathLength: 0 }}
        animate={{ pathLength: 1 }}
        transition={{ duration: 0.3, delay: 0.6 }}
      />
    </motion.svg>
  );
};
