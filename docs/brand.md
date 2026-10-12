# The brand layer

Atelier Scribe is WordCraft with a different name on the door. We keep the change small on purpose:
upstream lands many changes a day, and every file we touch is a file that can conflict when we pull
them in. So:

**Renamed (what a person sees):** the window title, dialogs and About (where Scribe comes from,
the Nation, this repository), the title-bar "Nation" button and its mark, the Help ribbon's and
Backstage's Nation entries and key tips, messages in the UI crate and the apps, Windows file
properties, the MCP server title and links, the LaTeX export's header comment, the sample
document's community link, installer and package names, vendor and links, the browser page, and
the app icon.

**Not renamed (machine names):** crate names (`wordcraft-*`), the binaries (`wordcraft`,
`wordcraft-cli`), command ids (`ui.discord` opens the Nation; the setting `show_discord` shows the
Nation button) and automation ids, the `ai.storyteller.wordcraft` app id and icon file names in
packaging, the default author name ("WordCraft User", written into documents), the generator tags
in exported RTF, ODT and HTML, the settings folder, environment variables (`WORDCRAFT_*`), and the
docs under `docs/`.

## Where the name lives

| Place | File |
|---|---|
| The constants and the translation rewrite | `crates/ui-egui/src/identity.rs` (`PRODUCT`, `UPSTREAM`, `rewrite`, `retired`) |
| Our own strings, with translations | `crates/ui-egui/src/i18n/brand.tsv` (added to each catalog as it loads; the upstream `.tsv` catalogs stay untouched) |
| Where the rewrite runs | `crates/ui-egui/src/i18n/catalog.rs` (`parse_entries`, `with_plain`) and `i18n/mod.rs` |
| Links | `crates/ui-egui/src/links.rs` |
| English UI strings | literal `"Atelier Scribe"` in `crates/ui-egui/src` and `apps/` |
| Nation button, mark, Help entries | `crates/ui-egui/src/{chrome,icons,ribbon,backstage,keytips}.rs` |
| About | `crates/ui-egui/src/dialogs.rs` |
| MCP | `crates/mcp/src/{server,backend}.rs` |
| Exports, sample | `crates/formats/src/latex.rs`, `crates/engine/src/sample.rs` |
| Windows file properties | `apps/wordcraft/build.rs` |
| macOS bundle | `packaging/macos/Info.plist.in` (display name, copyright; identifier unchanged) |
| Windows installer | `packaging/windows/wordcraft.wxs` (names, manufacturer, links, install folder, finish page; upgrade code unchanged) |
| Linux desktop entry, AppStream, nfpm | `packaging/linux/ai.storyteller.wordcraft.*`, `nfpm.yaml` |
| Browser build | `apps/wordcraft-web/index.html` |
| README and NOTICE | README: our header above upstream's feature sections; NOTICE: ours on top of theirs, kept on merge by `.gitattributes` |
| App icon | `assets/app-icon/` (ours; rows in `ATTRIBUTION.md`) |

Still upstream's name, for later passes: messages from the engine crates (they are still translated:
the UI looks them up under the rewritten name too), and the `docs/` manuals. For the packaging pass
(real installers): the macOS `.app` bundle and DMG names (`packaging/macos/package.sh`), the DMG
background, and the package scripts' file names.
