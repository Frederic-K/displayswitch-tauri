// Fenêtre Tauri : l'interface Svelte appelle ces commandes via `invoke("nom_de_la_commande")`.
// Chaque commande renvoie Ok (valeur reçue par le JS) ou Err (message de l'erreur levée côté JS).

use crate::config::{self, Settings};
use crate::displays::{self, Display};
use crate::{switch, win};
use std::process::ExitCode;

const WINDOW_TITLE: &str = "Display Switch";

// Ouvrir la fenêtre ; en cas d'échec, l'erreur est montrée dans une boîte Windows
// car l'exécutable distribué n'a pas de console.
pub fn run() -> ExitCode {
    let result = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_displays,
            telework,
            personal,
            detect_configuration,
            save_configuration
        ])
        .run(tauri::generate_context!());

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            win::show_error(
                WINDOW_TITLE,
                &format!("Impossible d'ouvrir la fenêtre : {error}"),
            );
            ExitCode::FAILURE
        }
    }
}

// Écrans disponibles, actifs ou non.
#[tauri::command]
fn list_displays() -> Result<Vec<Display>, String> {
    displays::read_displays().map(|(displays, _)| displays)
}

// Mode Télétravail : désactiver l'écran secondaire configuré.
#[tauri::command]
fn telework() -> Result<String, String> {
    config::load().and_then(|settings| switch::disable_secondary(&settings, false))
}

// Mode Personnel : réactiver le bureau étendu.
#[tauri::command]
fn personal() -> Result<String, String> {
    config::load().and_then(|settings| switch::enable_secondary(&settings))
}

// Rôles détectés dans Windows, proposés dans la boîte « Configurer Télétravail ».
#[tauri::command]
fn detect_configuration() -> Result<Settings, String> {
    switch::detect_telework_config()
}

// Revérifier Windows au moment d'enregistrer : les rôles ont pu changer entre-temps.
#[tauri::command]
fn save_configuration(proposal: Settings) -> Result<String, String> {
    let current = switch::detect_telework_config()?;
    if current != proposal {
        return Err(
            "La configuration Windows a changé. Rouvrez « Configurer Télétravail… ».".to_string(),
        );
    }
    config::save(&proposal)?;
    Ok("Configuration Télétravail enregistrée.".to_string())
}
