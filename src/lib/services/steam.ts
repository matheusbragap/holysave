/**
 * IPC para o comando Rust `scan_steam_games` (ver `docs/codigo-fonte/ponte-tauri-invoke.md`).
 */
import { invoke } from "@tauri-apps/api/core";
import type { Game } from "$lib/types/game";

export async function scanSteamGames(): Promise<Game[]> {
  return invoke<Game[]>("scan_steam_games");
}
