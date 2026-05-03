/**
 * Capas verticais via SteamGridDB (`get_steamgriddb_covers`). Precisa da API key no backend.
 */
import { invoke } from "@tauri-apps/api/core";

export async function getSteamGridCovers(
  appIds: string[]
): Promise<Record<string, string>> {
  if (appIds.length === 0) {
    return {};
  }

  return invoke<Record<string, string>>("get_steamgriddb_covers", { appIds });
}
