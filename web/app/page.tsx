import type { Metadata } from "next";
import Container from "@/components/container";
import { Header } from "@/components/header";
import { Install } from "@/components/install";
import { Commands } from "@/components/commands";
import { HowItWorks } from "@/components/how-it-works";
import { Guarantees } from "@/components/guarantees";
import { UnderTheHood } from "@/components/under-the-hood";
import { Limitations } from "@/components/limitations";
import { DottedSeparator } from "@/components/separator";
import { site } from "@/lib/site";

export const metadata: Metadata = {
  title: site.tagline,
  description: site.description,
  alternates: {
    canonical: "/",
  },
};

export default function Home() {
  return (
    <Container>
      <Header />
      <DottedSeparator className="my-10" />
      <Install />
      <DottedSeparator className="my-10" />
      <Commands />
      <DottedSeparator className="my-10" />
      <HowItWorks />
      <DottedSeparator className="my-10" />
      <Guarantees />
      <DottedSeparator className="my-10" />
      <UnderTheHood />
      <DottedSeparator className="my-10" />
      <Limitations />
      <DottedSeparator className="my-10" />
    </Container>
  );
}
