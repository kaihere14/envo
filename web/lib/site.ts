/**
 * Single source of truth for everything that names the project on the site.
 * Change it here and every page, the sitemap and the OG metadata follow.
 */
export const site = {
  name: "envo",
  tagline: "Zero-trust encrypted .env sync over Nostr.",
  description:
    "envo syncs .env secrets across your team over Nostr, encrypted per teammate so no server or relay can ever read them, even if it is fully compromised.",
  // Set NEXT_PUBLIC_SITE_URL at build time to the real domain; it drives
  // metadataBase, robots.txt and sitemap.xml.
  url: process.env.NEXT_PUBLIC_SITE_URL ?? "http://localhost:3000",
  github: "https://github.com/kaihere14/envo",
  releases: "https://github.com/kaihere14/envo/releases",
  installCommand:
    "curl -fsSL https://raw.githubusercontent.com/kaihere14/envo/main/install.sh | sh",
  status: "Beta",
} as const;
