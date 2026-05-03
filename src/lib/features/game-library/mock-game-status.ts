/**
 * Textos e flags ilustrativos por jogo (determinísticos). Não refletem o disco.
 * Ver docs/codigo-fonte/frontend/lib/features/game-library.md
 */
import type { Game } from "$lib/types/game";

export type MockGameHealth = "ok" | "attention" | "idle";

/** Hash simples e estável a partir da string — só para dados ilustrativos. */
function hashStable(s: string): number {
  let h = 0;
  for (let i = 0; i < s.length; i++) h = Math.imul(31, h) + s.charCodeAt(i) | 0;
  return Math.abs(h);
}

const STATUS_OPTIONS: readonly { health: MockGameHealth; label: string }[] = [
  { health: "ok", label: "Sincronizado com a biblioteca" },
  { health: "ok", label: "Pronto para backup" },
  { health: "attention", label: "Save não alterado há algum tempo" },
  { health: "idle", label: "Último jogo há dias — sem novo save" },
] as const;

export function getMockGameStatusLabel(game: Game): string {
  const idx = hashStable(game.id + game.name) % STATUS_OPTIONS.length;
  return STATUS_OPTIONS[idx]?.label ?? STATUS_OPTIONS[0].label;
}

export function getMockGameHealth(game: Game): MockGameHealth {
  const idx = hashStable(game.id + game.name) % STATUS_OPTIONS.length;
  return STATUS_OPTIONS[idx]?.health ?? "ok";
}

/** Data fictícia de “último backup” consistente por jogo. */
export function getMockLastBackupAt(game: Game): string {
  const h = hashStable(`backup:${game.id}`);
  const daysAgo = h % 28;
  const d = new Date();
  d.setDate(d.getDate() - daysAgo);
  d.setMinutes(h % 60, 0, 0);
  return new Intl.DateTimeFormat("pt-BR", {
    day: "2-digit",
    month: "short",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(d);
}

export function getMockAutoBackupActive(game: Game): boolean {
  return hashStable(`auto:${game.id}`) % 5 !== 0;
}
