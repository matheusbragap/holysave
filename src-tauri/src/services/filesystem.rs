//! Abrir pastas no explorador e remover caminhos (utilizado por comandos IPC).

use std::path::Path;
use std::process::Command;

/// Abre uma pasta usando o gerenciador padrão do sistema operacional
pub fn open_folder(path: &str) -> Result<(), String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("Caminho não existe: {}", path));
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(path_obj)
            .spawn()
            .map_err(|e| format!("Erro ao abrir pasta: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path_obj)
            .spawn()
            .map_err(|e| format!("Erro ao abrir pasta: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path_obj)
            .spawn()
            .map_err(|e| format!("Erro ao abrir pasta: {}", e))?;
    }

    Ok(())
}

/// Deleta uma pasta ou arquivo recursivamente
pub fn delete_path(path: &str) -> Result<(), String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("Caminho não existe: {}", path));
    }

    if path_obj.is_dir() {
        std::fs::remove_dir_all(path_obj)
            .map_err(|e| format!("Erro ao deletar pasta: {}", e))?;
    } else {
        std::fs::remove_file(path_obj)
            .map_err(|e| format!("Erro ao deletar arquivo: {}", e))?;
    }

    Ok(())
}