# Atelier Scribe

**The Nation's word processor.** A familiar ribbon with styles, tables, track changes, references,
mail merge, and .docx, PDF, ODT, RTF, HTML and Markdown files. A native app written in Rust. Runs on
macOS, Windows, Linux and in the browser. Offline, no account, no cloud.

Scribe is one of the Atelier apps, Aetherion Nation's own creative suite. It is built on
[WordCraft](https://github.com/storytold/wordcraft) by the ArtCraft team (MIT OR Apache-2.0), and
keeps pulling their work in: WordCraft is the engine, Atelier is the house it lives in.

| What | Where |
|---|---|
| Run it | `cargo run --release -p wordcraft -- --sample` (desktop) · `apps/wordcraft-web` (browser) · `wordcraft-cli` (headless) |
| How it is built | `AGENTS.md`, `CLAUDE.md`, `docs/` |
| The brand layer, and what we change versus upstream | [`docs/brand.md`](docs/brand.md) |
| Pulling in upstream work | [`docs/upstream.md`](docs/upstream.md) |

## Names

The product is **Atelier Scribe**. Crates, binaries and command ids keep their upstream `wordcraft`
names on purpose, so that upstream changes merge cleanly; see `docs/brand.md` before renaming
anything.

---

## What's in the box

The sections below are WordCraft's own notes, kept as upstream writes them (they still say
"WordCraft"; in Atelier the app is Scribe). They are the best description of what the app does.

## A tour

Every screenshot below is WordCraft itself, rendered offscreen by its own UI test harness
(`cargo run -p wordcraft-ui-egui --example ui_shot`).

<table>
<tr>
<td width="50%" valign="top"><img src="docs/images/review.png" alt="The Review tab with Track Changes on: the word forty is inserted in magenta and thirty struck through; three commented phrases are shaded and joined by dashed leader lines to comment balloons in a grey markup area to the right of the page" width="100%"><p align="center"><sub><b>Review.</b> Track changes, comment balloons in the margin, accept and reject, spelling and grammar as you type.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/references.png" alt="The References tab with two pages side by side: a table of contents with dotted leaders and page numbers on page one, and a styled table, numbered list and hyperlink on page two" width="100%"><p align="center"><sub><b>References.</b> Tables of contents, footnotes, citations in APA, MLA, Chicago or IEEE, index and captions.</sub></p></td>
</tr>
<tr>
<td width="50%" valign="top"><img src="docs/images/design.png" alt="The Design tab showing style-set previews; the document is set in a serif theme with plum headings underlined by thin rules and a pale diagonal DRAFT watermark behind the text" width="100%"><p align="center"><sub><b>Design.</b> Themes, style sets, paragraph spacing, watermarks, page colour and borders.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/dark.png" alt="WordCraft in dark mode with the Insert tab open and formatting marks shown: pilcrows at paragraph ends and dots for spaces" width="100%"><p align="center"><sub><b>Dark mode</b> with formatting marks, and the Insert tab: tables, pictures, shapes, links, headers, footers, fields and symbols.</sub></p></td>
</tr>
<tr>
<td colspan="2"><img src="docs/images/layout.png" alt="The Layout tab at 80% zoom: the first paragraph opens with a three-line drop cap E, a dark blue circle sits beside the second section with its paragraph wrapping around it and hyphenating art-ist at the line end, every line is numbered in the left margin and a thin blue border surrounds the page" width="100%"><p align="center"><sub><b>Layout.</b> Drop caps, text wrapping around pictures and shapes, automatic hyphenation, line numbers and page borders.</sub></p></td>
</tr>
<tr>
<td colspan="2"><img src="docs/images/backstage.png" alt="The File tab start page with a Good morning greeting, thumbnails of the Blank document, Studio handbook, Letter, Résumé and Report templates, and a list of recent documents" width="100%"><p align="center"><sub><b>File.</b> Start from a template, open recent documents, edit properties, export to PDF and other formats.</sub></p></td>
</tr>
</table>

## Why WordCraft

- **Familiar.** Word's ribbon tabs, groups, shortcuts and behaviour: Enter continues a list,
  Tab demotes it, Ctrl/⌘+B bolds the word under the caret, the Styles gallery previews styles live,
  F4 repeats, F7 checks spelling, F8 extends the selection.
- **Your files.** Opens and saves .docx (OOXML), and also .odt, .rtf, .html, .md, .tex (LaTeX), .txt; opens Word 97-2003 .doc; exports PDF
  with real, selectable text, links and bookmarks.
- **Fast.** Paragraph layout is cached, so typing in a 188-page document re-lays it out in about
  1.4 ms; pages render on demand.
- **Everywhere.** One Rust codebase for macOS, Windows, Linux, BSD and the web. No Electron, no
  Tauri: native [egui](https://github.com/emilk/egui) on the GPU.
- **Built for agents.** Every action is a command with an id. The same 422 commands drive the
  ribbon, keyboard shortcuts, the command search, a command-line tool, a JSON control channel and
  an MCP server.
- **Private.** Spelling, grammar and everything else work offline.
- **Open.** MIT OR Apache-2.0. Clean-room: built from public specifications and observation, with
  every asset original or openly licensed.

## What works today

| Area | Highlights |
|---|---|
| **Writing** | Fast typing with IME, smart quotes, AutoCorrect, list autoformat (`* `, `1. `), dashes; word, sentence, paragraph selection; drag-select; clipboard with formatting; undo/redo; find and replace with regex |
| **Formatting** | Fonts, sizes, bold/italic/underline styles, strike, sub/superscript, caps, highlight, colours, character spacing, Format Painter, Change Case, Clear Formatting |
| **Paragraphs** | Alignment, indents (draggable on the ruler), spacing, line spacing, tabs with leaders, borders, shading, keep with next, widow/orphan control |
| **Styles** | Built-in style set, live gallery, Styles pane, create/modify/update styles, style sets, themes |
| **Lists** | Bullets, numbering, multilevel, restart, set value, custom formats |
| **Tables** | Insert by grid, merge/split, styles with banded rows, borders, shading, header rows repeated across pages, rows that split across pages, sort, formulas, text ↔ table |
| **Pages** | Margins, orientation, size, columns, page/column/section breaks, headers and footers (first page, odd/even), page numbers, watermark, page borders, line numbers, vertical alignment, drop caps, automatic hyphenation |
| **Objects** | Pictures (resize, crop, recolour, brightness/contrast, transparency, background removal, picture styles, rotate), shapes, text boxes, floating position with text wrapping (square, top and bottom, behind or in front of text) |
| **References** | Table of contents, footnotes and endnotes, citations and bibliography (APA, MLA, Chicago, IEEE), captions, table of figures, cross-references, index, table of authorities |
| **Review** | Spelling and grammar with suggestions, thesaurus, word count, comments in margin balloons or a pane, track changes, accept/reject, compare documents, restrict editing, accessibility checker, document inspector |
| **Mailings** | Mail merge from CSV, merge fields, address block, greeting line, rules, preview, finish to a document; envelopes and labels |
| **View** | Print layout, web layout, draft, read mode, focus, zoom, one/multiple pages, page width, Navigation pane, rulers, gridlines, dark mode; interface in English, 简体中文, 繁體中文, 日本語, Português (Brasil), Español or Українська (follows the system language by default) |
| **Files** | .docx read/write (opens in Word), Word 97-2003 .doc import, PDF export, .odt, .rtf, .html, .md, .tex, .txt import/export, page images |

The honest picture, area by area, is in [ROADMAP.md](ROADMAP.md), the
[parity assessment](docs/target-app-parity.md) and the generated
[feature checklist](docs/parity-checklist.md).

## Quick start

```sh
git clone https://github.com/storytold/wordcraft
cd wordcraft
cargo run --release -p wordcraft -- --sample        # the desktop app with the sample document
cargo run --release -p wordcraft -- report.docx     # open a document
```

Command line:

```sh
wordcraft-cli convert report.docx report.pdf        # docx, pdf, odt, rtf, html, md, tex, txt, png
wordcraft-cli text report.docx                      # plain text
wordcraft-cli inspect report.docx                   # structure as JSON
wordcraft-cli run --template sample \
  --cmd 'select.text={"text":"Membership"}' --cmd format.bold --save out.docx
wordcraft-cli run --script job.json                 # the same from a file: commands, then save to docx and pdf
```

Web: `cd apps/wordcraft-web && trunk serve`, then open <http://127.0.0.1:8771/?sample>.

## For agents: CLI and MCP

WordCraft was designed to be driven by people *and* by AI agents.

```sh
claude mcp add wordcraft -- wordcraft-cli mcp                  # headless documents
wordcraft --control 7981 &                                     # or drive the running app…
claude mcp add wordcraft-app -- wordcraft-cli mcp --connect 127.0.0.1:7981
```

Tools include `list_commands`, `execute`, `batch`, `type_text`, `select_text`, `inspect_document`,
`render_page`, `save_document`, and, with a running app, `screenshot`, `click` and `key`. Agents
can check their work through `inspect_document` without screenshots. See [docs/mcp.md](docs/mcp.md)
and the [control protocol](docs/control-protocol.md). Macros record any sequence of commands and
play it back (`tools.recordMacro`, `tools.macros`).

## Logs

The desktop app writes its log records to standard error and to `logs/wordcraft.log` next to its
preferences: on Linux `$XDG_CONFIG_HOME/wordcraft/logs/` (by default `~/.config/wordcraft/logs/`),
on macOS `~/Library/Application Support/WordCraft/logs/`, on Windows `%APPDATA%\WordCraft\logs\`.
A start from a desktop menu or the Dock has no terminal, so this file is what to attach to a bug
report: unreadable parts of a .docx, pictures or fonts a PDF export had to leave out and panics the
command guard recovered from land there. Each launch moves the previous log to `wordcraft.1.log`
(and that one to `wordcraft.2.log`), so the log of a run that crashed survives the next start; a
log stops growing at 16 MiB. `--version` writes no file, and runs with `WORDCRAFT_NO_PREFS` (agents'
test runs) log to standard error only. Document text is never logged; a panic message leaves out
the text it quotes.

By default WordCraft's own crates log at `info` and everything else at `warn`. `RUST_LOG` replaces
that with env_logger-style directives, for example `RUST_LOG=debug`,
`RUST_LOG=warn,wordcraft_docx=trace` or `RUST_LOG=info,wgpu_core=warn`; a directive ending in `*`
covers every target starting with it (`wordcraft*=debug`). The logger is
`apps/wordcraft/src/logging.rs`.

## Architecture

| Layer | Crate | Job |
|---|---|---|
| L0 | `wordcraft-geom` | units and measurements |
| L1 | `wordcraft-doc`, `wordcraft-fonts`, `wordcraft-proof` | document model and editing; fonts and shaping; spelling, grammar, hyphenation |
| L2 | `wordcraft-layout`, `wordcraft-docx`, `wordcraft-formats` | line breaking, pagination, tables, notes, hit testing; OOXML; ODT/RTF/HTML/Markdown/TXT |
| L3 | `wordcraft-render`, `wordcraft-pdf` | rasteriser (vello_cpu); PDF (krilla) |
| L4 | `wordcraft-engine` | session, undo, 422 commands, Word feature catalog |
| L5 | `wordcraft-mcp` | MCP server |
| L6 | `wordcraft-ui-egui` | the Word-style front end (swappable) |
| apps | `wordcraft`, `wordcraft-cli`, `wordcraft-web` | desktop, command line, browser |

`cargo xtask ci` runs formatting, clippy, ~800 tests, the asset-attribution check, the layering
check and the wasm build. Contributor and agent instructions: [AGENTS.md](AGENTS.md).

## Roadmap

WordCraft is in **alpha**. It covers 88% of Word's ribbon features with commands today; counting
depth, file fidelity, stability and the rest, we estimate it is about 60% of the way to replacing
all of Word for real work, about 55% for a typical professional's weekly work and about 63% for
everyday essentials. Beta needs testing against real-world .docx files, keeping charts and SmartArt
through a round trip, pagination closer to Word's, a stability sweep and native printing. Details
and estimates: [ROADMAP.md](ROADMAP.md) and [where we fall short](docs/gaps.md).

## License

MIT OR Apache-2.0, like WordCraft. Third-party material is listed in `ATTRIBUTION.md` and `NOTICE`.
The Atelier and Aetherion names and marks are not covered by the license.

<sub>Microsoft and Word are trademarks of the Microsoft group of companies. Atelier Scribe is an
independent, open-source project and is not affiliated with, sponsored by or endorsed by Microsoft;
these names are used only to describe the workflows it is compatible with.</sub>
