// Appels aux commandes Rust (src-tauri/src/app.rs).
// Chaque promesse est rejetée avec le message d'erreur renvoyé par Rust.
import { invoke } from "@tauri-apps/api/core";

export const listDisplays = () => invoke("list_displays");
export const telework = () => invoke("telework");
export const personal = () => invoke("personal");
export const detectConfiguration = () => invoke("detect_configuration");
export const saveConfiguration = (proposal) => invoke("save_configuration", { proposal });
