/**
 * Capas verticais via SteamGridDB (`get_steamgriddb_covers`). Precisa da API key no backend.
 */
import { convertFileSrc, invoke } from "@tauri-apps/api/core";

export async function getSteamGridCovers(
  appIds: string[]
): Promise<Record<string, string>> {
  if (appIds.length === 0) {
    return {};
  }

  const covers = await invoke<Record<string, string>>("get_steamgriddb_covers", { appIds });
  const normalized: Record<string, string> = {};

  for (const [appId, path] of Object.entries(covers)) {
    normalized[appId] = path.startsWith("http") ? path : convertFileSrc(path);
  }

  return normalized;
}
