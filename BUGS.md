# Bugs & Corrections - Zed Custom App

## Terminal (Claude Code / Codex / Gemini)

- [x] #1 - `terminal_element.rs:580` - `.take().unwrap()` → `let Some() else continue`
- [x] #2 - `terminal_element.rs:1027` - `.unwrap()` → `.expect()` avec message descriptif
- [x] #3 - `terminal_element.rs:1108` - `.advance().unwrap()` → `.unwrap_or(px(8.0))`
- [x] #4 - `terminal_panel.rs:1157` - `.move_to_border().unwrap()` → `.unwrap_or(false)`
- [x] #5 - `terminal_element.rs:196` - `bidi_info` inutilisé → supprimé
- [x] #6 - `terminal_element.rs:159,246,289` - `let _ =` → `.log_err()`
- [x] #7 - CLI pas installé → faux positif (shell affiche "command not found")
- [x] #20 - Menu + double dispatch → faux positif (.action() = raccourci clavier seulement)
- [x] #21 - `terminal_view.rs:1893` - `.ok()` → `.log_err()`

## Git (commit, push, panel)

- [x] #8 - `git_panel.rs:145` - `.nth().unwrap()` → `.ok_or_else(anyhow!)`
- [x] #9 - `repository.rs:1116` - `.stdin.take().unwrap()` → `.context()?`
- [x] #10 - `repository.rs:1361` - `.stdin.take().unwrap()` → `.context()?`
- [x] #11 - `git_panel.rs:407` - `.expect()` → gardé avec message (RepoPath::new retourne pas Option)
- [x] #22 - `git_panel.rs:2093` - `.unwrap()` → `.expect()` avec message descriptif
- [x] #23 - `git_panel.rs:2390` - `.ok()` → `.log_err()`
- [x] #24 - `repository.rs:522` - `.unwrap()` → `if let Some()`

## Fichiers (project panel)

- [x] #12 - `project_panel_settings.rs:93-142` - 24x `.unwrap()` → `.expect()` avec messages
- [x] #13 - `project_panel.rs:3071` - `.unwrap()` → `.log_err()?`
- [x] #14 - `project_panel.rs:4875` - gardé `.expect()` avec message (contexte garantit la valeur)

## Settings (thème, police, taille)

- [x] #15 - `item.rs:77-114` - 16x `.unwrap()` → `.expect()` avec messages
- [x] #16 - `theme.rs:476` - `.unwrap()` → `.expect("default dark theme must exist")`
- [x] #17 - `theme.rs:505` - `.unwrap()` → `.expect("default icon theme must exist")`

## Ressources

- [ ] #18 - `terminal.rs:581` - canal unbounded - TODO équipe Zed, complexe à changer sans risque
- [x] #19 - Thème Fedi - ajouté couleurs `version_control.*`
- [x] #25 - `main.rs:600-607` - telemetry::event! macros → supprimées
- [ ] #26 - 8 modules initialisés pour type registration - nécessaire pour éviter panics au démarrage

## Résumé : 24/26 corrigés | 2 non corrigés (risque très faible, complexité élevée)
