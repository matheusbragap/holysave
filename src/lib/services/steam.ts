import { invoke } from "@tauri-apps/api/core";
import type { Game } from "$lib/types/game";

export async function getSteamPath(): Promise<string | null> {
  return invoke<string | null>("get_steam_path");
}

export async function scanSteamGames(): Promise<Game[]> {
  return invoke<Game[]>("scan_steam_games");
}
