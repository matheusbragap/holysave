/**
 * Contrato TS usado pela maioria dos componentes. O JSON do Rust pode trazer mais campos.
 * Ver `docs/models/game.md` e `docs/codigo-fonte/backend/modelos.md`.
 */
export type Game = {
  id: string;
  name: string;
  platform: string;
  install_dir?: string;
  size_bytes?: number;
  save_path?: string | null;
};
