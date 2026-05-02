export type Game = {
  id: string;
  name: string;
  platform: string;
  install_dir?: string;
  size_bytes?: number;
  save_path?: string | null;
};
