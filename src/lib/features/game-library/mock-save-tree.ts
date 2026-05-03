/**
 * Gera uma árvore de pastas fictícia para o modal (pré-visualização).
 * Doc: docs/codigo-fonte/frontend/lib/features/game-library.md
 */
import type { Game } from "$lib/types/game";

/** Nós de uma estrutura de pastas ilustrativa (não espelha o disco). */
export type MockTreeNode = {
  name: string;
  children?: MockTreeNode[];
};

function safeSlug(name: string): string {
  return name.replace(/[/\\?%*:|"<>]/g, "_").trim() || "Game";
}

/** Árvore fictícia só para pré-visualização no modal. */
export function buildMockSaveTree(game: Game): MockTreeNode {
  const rootName = `${safeSlug(game.name)}_Data`;

  return {
    name: rootName,
    children: [
      {
        name: "saves",
        children: [
          {
            name: "profile",
            children: [{ name: "savegame.dat" }, { name: "metadata.json" }],
          },
          {
            name: "backup",
            children: [{ name: "autosave.slot" }, { name: "manual_01.bak" }],
          },
        ],
      },
      {
        name: "config",
        children: [{ name: "graphics.ini" }, { name: "keybindings.xml" }],
      },
      {
        name: "screenshots",
        children: [{ name: ".gitkeep" }],
      },
    ],
  };
}
