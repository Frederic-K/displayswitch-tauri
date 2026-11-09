# Display Switch (Tauri 2 + Svelte 5)

Désactive ou réactive l'écran secondaire configuré, en deux boutons :

- **Télétravail** : désactive l'écran secondaire, le principal reste actif et principal.
- **Personnel** : réactive le bureau étendu mémorisé par Windows (retour arrière automatique en cas d'échec).
- **Configurer Télétravail…** : enregistre les rôles actuels de Windows (écran principal = écran à conserver).

Même logique et même apparence que `display-switch-rs` (version egui).

## Structure d'un projet Tauri

Tauri = un exécutable Rust qui ouvre une fenêtre WebView2 (le moteur d'Edge) affichant l'interface web.
L'interface appelle le Rust avec `invoke("nom_commande")`, qui renvoie une promesse.

```
index.html, vite.config.js   Interface : projet Vite + Svelte classique
src/
  main.js                    Monte App.svelte
  App.svelte                 Fenêtre principale et état
  theme.css                  Couleurs, tailles, boutons (repris de theme.rs)
  lib/api.js                 Appels invoke() vers le Rust
  components/                Liste des écrans, ligne, badge d'état, boîte de configuration
src-tauri/                   Partie Rust (un projet Cargo)
  tauri.conf.json            Fenêtre, commandes de build, paquet MSI
  capabilities/default.json  Droits de la fenêtre (fonctions de base de Tauri uniquement)
  build.rs, icons/           Exigés par Tauri (intégration des icônes)
  src/
    main.rs                  Point d'entrée : fenêtre, ou commande de développement si argument
    app.rs                   Démarrage Tauri et commandes appelables depuis le JS
    win.rs                   API Windows (QueryDisplayConfig / SetDisplayConfig) — repris tel quel
    displays.rs              Lecture des écrans — repris, + sérialisation vers le JS
    switch.rs                Modes et règles de sécurité — repris tel quel
    config.rs                Préférences JSON — repris, emplacement changé (voir ci-dessous)
    cli.rs                   Commandes list / check / disable / enable — repris tel quel
```

## Commandes

```bash
npm install                 # une fois
npm run tauri dev           # fenêtre de développement, rechargement à chaud de l'interface
npm run tauri build         # MSI dans src-tauri/target/release/bundle/msi/
```

Le premier `tauri build` télécharge automatiquement WiX (outil de création de MSI).

Commandes de développement (console), depuis `src-tauri/` :

```bash
cargo run -- list           # écrans disponibles
cargo run -- check          # valide la désactivation auprès de Windows, sans rien appliquer
cargo test                  # tests des règles de sécurité et de la configuration
```

## Configuration

`%APPDATA%\Display Switch\config.json` (mêmes clés que les versions précédentes).
Ce n'est plus à côté de l'exécutable : installé par le MSI dans Program Files, le programme n'a pas le droit d'y écrire.
