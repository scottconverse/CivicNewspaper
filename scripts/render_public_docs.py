#!/usr/bin/env python3
"""Render selected public Markdown docs into simple GitHub Pages HTML.

This is intentionally small and dependency-free. It supports the Markdown
features used by the public docs well enough for a visitor-grade rendered
manual: headings, paragraphs, bullets, numbered lists, fenced code, blockquotes,
horizontal rules, inline code, emphasis, strong text, and links.
"""

from __future__ import annotations

import html
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"

PUBLIC_DOCS = [
    "api.md",
    "user_manual.md",
    "install.md",
    "release-readiness.md",
    "architecture.md",
    "troubleshooting.md",
    "publishing-connectors.md",
    "discussion_seeds.md",
    "prd-local-llm-newsroom-v1.md",
    "implementation-plan-v0.3.0-to-v1.0.0.md",
]


def html_target_for(href: str) -> str:
    if href.startswith(("http://", "https://", "mailto:", "#")):
        return href
    if "#" in href:
        path, anchor = href.split("#", 1)
        suffix = f"#{anchor}"
    else:
        path, suffix = href, ""
    if path.endswith(".md"):
        return path[:-3] + ".html" + suffix
    return href


def inline_markup(text: str) -> str:
    placeholders: list[str] = []

    def stash_link(match: re.Match[str]) -> str:
        label = inline_markup(match.group(1))
        href = html.escape(html_target_for(match.group(2)), quote=True)
        token = f"\x00{len(placeholders)}\x00"
        placeholders.append(f'<a href="{href}">{label}</a>')
        return token

    def stash_code(match: re.Match[str]) -> str:
        token = f"\x00{len(placeholders)}\x00"
        placeholders.append(f"<code>{html.escape(match.group(1))}</code>")
        return token

    escaped = html.escape(text)
    escaped = re.sub(r"`([^`]+)`", stash_code, escaped)
    escaped = re.sub(r"&lt;((?:https?://|mailto:)[^&]+)&gt;", lambda m: f'<a href="{html.escape(m.group(1), quote=True)}">{html.escape(m.group(1))}</a>', escaped)
    escaped = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", stash_link, escaped)
    escaped = re.sub(r"\*\*([^*]+)\*\*", r"<strong>\1</strong>", escaped)
    escaped = re.sub(r"\*([^*]+)\*", r"<em>\1</em>", escaped)
    for idx, value in enumerate(placeholders):
        escaped = escaped.replace(f"\x00{idx}\x00", value)
    return escaped


def render_markdown(markdown: str) -> str:
    lines = markdown.replace("\r\n", "\n").split("\n")
    out: list[str] = []
    paragraph: list[str] = []
    in_code = False
    code_lines: list[str] = []
    list_type: str | None = None

    def flush_paragraph() -> None:
        nonlocal paragraph
        if paragraph:
            out.append(f"<p>{inline_markup(' '.join(paragraph).strip())}</p>")
            paragraph = []

    def close_list() -> None:
        nonlocal list_type
        if list_type:
            out.append(f"</{list_type}>")
            list_type = None

    for raw in lines:
        line = raw.rstrip()
        if line.startswith("```"):
            flush_paragraph()
            close_list()
            if in_code:
                out.append("<pre><code>" + html.escape("\n".join(code_lines)) + "</code></pre>")
                code_lines = []
                in_code = False
            else:
                in_code = True
            continue
        if in_code:
            code_lines.append(line)
            continue

        if not line.strip():
            flush_paragraph()
            close_list()
            continue

        if line.strip() == "---":
            flush_paragraph()
            close_list()
            out.append("<hr>")
            continue

        heading = re.match(r"^(#{1,6})\s+(.+)$", line)
        if heading:
            flush_paragraph()
            close_list()
            level = len(heading.group(1))
            text = inline_markup(heading.group(2).strip())
            slug = re.sub(r"[^a-z0-9]+", "-", heading.group(2).lower()).strip("-")
            out.append(f'<h{level} id="{slug}">{text}</h{level}>')
            continue

        quote = re.match(r"^>\s?(.+)$", line)
        if quote:
            flush_paragraph()
            close_list()
            out.append(f"<blockquote>{inline_markup(quote.group(1))}</blockquote>")
            continue

        bullet = re.match(r"^\s*[-*]\s+(.+)$", line)
        numbered = re.match(r"^\s*\d+\.\s+(.+)$", line)
        if bullet or numbered:
            flush_paragraph()
            desired = "ul" if bullet else "ol"
            if list_type != desired:
                close_list()
                out.append(f"<{desired}>")
                list_type = desired
            item = (bullet or numbered).group(1)
            out.append(f"<li>{inline_markup(item)}</li>")
            continue

        close_list()
        paragraph.append(line.strip())

    flush_paragraph()
    close_list()
    if in_code:
        out.append("<pre><code>" + html.escape("\n".join(code_lines)) + "</code></pre>")
    return "\n".join(out)


def title_from_markdown(markdown: str, fallback: str) -> str:
    for line in markdown.splitlines():
        if line.startswith("# "):
            return line[2:].strip()
    return fallback


def render_page(source: Path) -> str:
    markdown = source.read_text(encoding="utf-8")
    title = title_from_markdown(markdown, source.stem.replace("-", " ").title())
    body = render_markdown(markdown)
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{html.escape(title)} | The Civic Desk</title>
    <meta name="description" content="{html.escape(title)} for The Civic Desk public beta.">
    <link rel="stylesheet" href="style.css">
    <link rel="stylesheet" href="doc-page.css">
</head>
<body class="doc-page">
    <nav aria-label="Site navigation">
        <div class="nav-container">
            <a class="logo" href="index.html#top" aria-label="The Civic Desk home">
                <span>The Civic Desk</span>
            </a>
            <div class="nav-links">
                <a href="index.html#downloads">Downloads</a>
                <a href="user_manual.html">Manual</a>
                <a href="install.html">Install</a>
                <a href="troubleshooting.html">Troubleshooting</a>
                <a class="nav-btn-primary" href="https://github.com/scottconverse/CivicNewspaper" target="_blank" rel="noopener noreferrer">GitHub</a>
            </div>
        </div>
    </nav>
    <main class="doc-shell">
        <article class="doc-content">
{body}
        </article>
    </main>
    <footer>
        <div class="footer-container">
            <div>
                <a class="logo" href="index.html#top"><span>The Civic Desk</span></a>
                <p>Open-source local newsroom software. Repository name: CivicNewspaper. Installed app name: The Civic Desk.</p>
            </div>
            <div class="footer-links">
                <a href="user_manual.html">Manual</a>
                <a href="troubleshooting.html">Troubleshooting</a>
                <a href="architecture.html">Architecture</a>
                <a href="https://github.com/scottconverse/CivicNewspaper" target="_blank" rel="noopener noreferrer">GitHub</a>
            </div>
        </div>
    </footer>
</body>
</html>
"""


def main() -> int:
    for name in PUBLIC_DOCS:
        source = DOCS / name
        if not source.exists():
            raise SystemExit(f"missing public doc: {source}")
        target = source.with_suffix(".html")
        target.write_text(render_page(source), encoding="utf-8", newline="\n")
        print(f"rendered {source.relative_to(ROOT)} -> {target.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
