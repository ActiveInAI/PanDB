"use client";

import Link from "next/link";
import { Languages } from "lucide-react";
import { GithubIcon } from "@/components/GithubIcon";
import { LanguageSelect } from "fumadocs-ui/layouts/shared/slots/language-select";
import { ThemeSwitch } from "fumadocs-ui/layouts/shared/slots/theme-switch";
import type { DocsLang } from "@/lib/i18n";

const iconButton = "inline-flex size-8 items-center justify-center rounded-md text-fd-muted-foreground transition-colors hover:bg-fd-accent hover:text-fd-accent-foreground";

// 文案与 LandingNav 保持一致；文档区没有横向主导航，这里补回站内入口。
const i18n: Record<DocsLang, { navLabel: string; home: string; plugins: string; changelog: string; drivers: string }> = {
  en: { navLabel: "Site navigation", home: "Home", plugins: "Plugins", changelog: "Changelog", drivers: "Offline Drivers" },
  cn: { navLabel: "站内导航", home: "首页", plugins: "插件", changelog: "更新日志", drivers: "离线驱动" },
};


export function DocsSidebarLanguageButton() {
  return (
    <div className="flex justify-end pe-1">
      <LanguageSelect className={iconButton} variant="ghost">
        <Languages className="size-4.5" />
      </LanguageSelect>
    </div>
  );
}

export function DocsSidebarFooter({ lang }: { lang: DocsLang }) {
  const t = i18n[lang];
  const siteLinks = [
    { href: `/${lang}`, label: t.home },
    { href: `/${lang}/plugins`, label: t.plugins },
    { href: `/${lang}/changelog`, label: t.changelog },
    { href: `/${lang}/drivers`, label: t.drivers },
  ];

  return (
    <div className="dbx-docs-sidebar-footer">
      <nav className="dbx-docs-sidebar-sites" aria-label={t.navLabel}>
        {siteLinks.map((link) => (
          <Link key={link.href} href={link.href} prefetch={false}>
            {link.label}
          </Link>
        ))}
      </nav>
      <div className="dbx-docs-sidebar-tools">
        <div className="flex items-center gap-1">
          <a className={iconButton} href="https://github.com/ActiveInAI/PanDB" target="_blank" rel="noreferrer" aria-label="GitHub">
            <GithubIcon />
          </a>
        </div>
        <ThemeSwitch mode="light-dark" className="dbx-docs-theme-switch" />
      </div>
    </div>
  );
}
