/**
 * Invokes por jogo: abrir pasta e apagar save. Ver `docs/codigo-fonte/frontend/lib/features/game-library.md`.
 */
import { invoke } from "@tauri-apps/api/core";
import type { Game } from "$lib/types/game";

export async function openFolderPath(path: string | undefined | null): Promise<boolean> {
  if (!path?.trim()) return false;
  try {
    await invoke("open_folder", { path });
    return true;
  } catch {
    return false;
  }
}

export async function deleteGameSave(game: Game): Promise<boolean> {
  try {
    await invoke("delete_save", { game });
    return true;
  } catch {
    return false;
  }
}
