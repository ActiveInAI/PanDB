import { LandingNav } from "@/components/landing/LandingNav";
import { LandingFooter } from "@/components/landing/LandingFooter";
import { buildMetadata } from "@/lib/metadata";
import type { Metadata } from "next";
import { resolveLang } from "@/lib/i18n";

const channels = [
  {
    id: "github",
    color: "#f0f0f0",
    href: "https://github.com/ActiveInAI/PanDB/discussions",
    icon: (
      <svg width={24} height={24} viewBox="0 0 24 24" fill="currentColor">
        <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z" />
      </svg>
    ),
  },
];

const i18n = {
  en: {
    title: "Feedback",
    desc: "Ask questions and share feedback through GitHub Discussions.",
    github: { name: "GitHub Discussions", desc: "Bug reports, feature requests, and long-form discussions." },
  },
  cn: {
    title: "问题反馈",
    desc: "通过 GitHub Discussions 提问和反馈。",
    github: { name: "GitHub Discussions", desc: "Bug 反馈、功能建议和深度讨论。" },
  },
};

export async function generateMetadata({ params }: { params: Promise<{ lang: string }> }): Promise<Metadata> {
  const { lang } = await params;
  const l = resolveLang(lang);
  const t = i18n[l];

  return buildMetadata({
    title: t.title,
    description: t.desc,
    path: `/${l}/community`,
    lang: l,
  });
}

export default async function CommunityPage({ params }: { params: Promise<{ lang: string }> }) {
  const { lang } = await params;
  const l = resolveLang(lang);
  const t = i18n[l];

  return (
    <main className="min-h-screen bg-landing-bg text-landing-ink">
      <LandingNav lang={l} active="community" />

      <div className="max-w-[860px] mx-auto px-6 pt-32 pb-4 max-[760px]:px-[18px] max-[760px]:pt-28">
        <h1 className="text-4xl font-[820] tracking-tight">{t.title}</h1>
        <p className="mt-3 text-landing-muted text-lg">{t.desc}</p>
      </div>

      <div className="max-w-[860px] mx-auto px-6 pb-24 max-[760px]:px-[18px]">
        <div className="grid gap-4 mt-8">
          {channels.map((ch) => {
            const meta = t[ch.id as keyof typeof t] as { name: string; desc: string };
            return (
              <a key={ch.id} href={ch.href} target="_blank" rel="noopener noreferrer" className="group flex items-center gap-5 rounded-xl border border-landing-line bg-landing-panel px-6 py-5 transition-colors hover:border-[rgba(173,176,182,0.32)]">
                <span className="grid place-items-center w-11 h-11 rounded-lg shrink-0" style={{ backgroundColor: `${ch.color}18`, color: ch.color }}>
                  {ch.icon}
                </span>
                <span className="grid gap-0.5 min-w-0">
                  <strong className="text-[15px] font-[700] leading-snug">{meta.name}</strong>
                  <span className="text-landing-muted text-[13px] leading-relaxed">{meta.desc}</span>
                </span>
                <svg className="ml-auto shrink-0 text-landing-muted opacity-0 -translate-x-1 transition-all group-hover:opacity-100 group-hover:translate-x-0" width={16} height={16} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2} strokeLinecap="round" strokeLinejoin="round">
                  <path d="M5 12h14M12 5l7 7-7 7" />
                </svg>
              </a>
            );
          })}
        </div>
      </div>

      <LandingFooter lang={l} />
    </main>
  );
}
