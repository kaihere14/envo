"use client";

import { ThemeProvider } from "next-themes";

/**
 * The palettes in `components/settings.tsx` are all light (`*-50` backgrounds)
 * and there is no dark toggle in the UI, so the site is light-only. Without
 * `forcedTheme` next-themes follows the OS and adds `.dark` to <html>, which
 * flips `--foreground` to near-white while the background stays light,
 * leaving the page unreadable for every visitor whose system is in dark mode.
 */
export default function Providers({ children }) {
  return (
    <ThemeProvider attribute="class" defaultTheme="light" forcedTheme="light">
      {children}
    </ThemeProvider>
  );
}
