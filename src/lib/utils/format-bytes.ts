/** Formato legível de tamanhos para a UI (`docs/codigo-fonte/frontend/lib/types-e-utils.md`). */

export function formatBytes(value?: number): string | null {
  if (!value) return null;
  const units = ["B", "KB", "MB", "GB", "TB"];
  let size = value;
  let unitIndex = 0;
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex += 1;
  }
  return `${size.toFixed(size >= 100 ? 0 : 1)} ${units[unitIndex]}`;
}
