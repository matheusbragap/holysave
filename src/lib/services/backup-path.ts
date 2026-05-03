/**
 * Pasta padrão de backup (`get_default_backup_directory` no Rust).
 * Doc: `docs/codigo-fonte/frontend/lib/servicos.md`.
 */
import { invoke } from "@tauri-apps/api/core";

export async function getDefaultBackupDirectory(): Promise<string> {
  return invoke<string>("get_default_backup_directory");
}
