// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Ponto de entrada binário: delega para `holysave_lib::run`.
// Doc: docs/codigo-fonte/backend/entrypoint-e-comandos.md

fn main() {
    holysave_lib::run()
}
