"use client";
import React from "react";
import Image from "next/image";
import { motion } from "motion/react";
import { GENERAL_VARIANT, SPRING_CONFIG } from "@/lib/motion-config";
import Link from "next/link";
import { cn } from "@/lib/utils";
import { site } from "@/lib/site";
import { DottedUnderline } from "./dotted-underline";

const links = [
  { title: "Install", href: "#install" },
  { title: "How it works", href: "#how-it-works" },
  { title: "Under the hood", href: "#under-the-hood" },
  { title: "GitHub", href: site.github, external: true },
];

export const Navbar = () => {
  return (
    <nav className="mx-auto flex max-w-2xl flex-col items-start gap-4 px-4 pt-4 md:pt-8">
      <div className="flex items-center gap-2 perspective-distant">
        <motion.div
          variants={GENERAL_VARIANT}
          initial="initial"
          animate="animate"
          exit="exit"
          transition={SPRING_CONFIG}
          className="rounded-md bg-transparent ark:bg-neutral-800"
        >
          <Image
            src="/envo.webp"
            alt=""
            width={40}
            height={40}
            priority
            className="aspect-square size-6 rounded-md object-contain shadow-2xl"
          />
        </motion.div>
        <h1 className="text-foreground text-xl font-medium tracking-tight md:text-2xl">
          {site.name}{" "}
          <span className="text-foreground/50 font-normal">—</span>{" "}
          <span className="font-normal italic">secrets that stay secret</span>
        </h1>
        <span className="text-foreground/60 shrink-0 rounded-full border border-neutral-300 px-2 py-0.5 font-mono text-[10px] tracking-wide uppercase dark:border-neutral-700">
          {site.status}
        </span>
      </div>
      <div className="flex flex-wrap items-center gap-4">
        {links.map((link) => (
          <Link
            key={link.href}
            href={link.href}
            {...(link.external
              ? { target: "_blank", rel: "noopener noreferrer" }
              : {})}
            className={cn(
              "group text-foreground/70 hover:text-primary relative transition-colors",
            )}
          >
            {link.title}
            <DottedUnderline className="mask-x-from-90% opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
          </Link>
        ))}
      </div>
    </nav>
  );
};
