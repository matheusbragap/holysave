//! Caminho de instalação do Steam no Windows e agregação de pastas de biblioteca.

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

use super::parser::parse_libraryfolders;

pub fn get_steam_path() -> Option<PathBuf> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    hklm.open_subkey("SOFTWARE\\Valve\\Steam")
        .or_else(|_| hklm.open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam")) // fallback old steam
        .ok()
        .and_then(|key| {
            let path: String = key.get_value("InstallPath").ok()?;
            Some(PathBuf::from(path))
        })
}

pub fn get_library_folders() -> Vec<PathBuf> {
    let Some(steam_root) = get_steam_path() else {
        return Vec::new();
    };

    let mut paths = HashSet::new();
    paths.insert(steam_root.clone());

    let vdf_path = steam_root.join("steamapps").join("libraryfolders.vdf");
    if let Ok(contents) = fs::read_to_string(&vdf_path) {
        for path in parse_libraryfolders(&contents) {
            paths.insert(path);
        }
    }

    let mut result: Vec<PathBuf> = paths.into_iter().collect();
    result.sort();
    result
}
