//! Parser minimalista para linhas VDF citadas em `libraryfolders.vdf` e manifestos Steam.

use std::path::PathBuf;

pub(crate) struct AppManifest {
    pub(crate) app_id: String,
    pub(crate) name: String,
    pub(crate) install_dir: String,
    pub(crate) size_bytes: u64,
}

pub(crate) fn parse_libraryfolders(contents: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for line in contents.lines() {
        let tokens = quoted_tokens(line);
        if tokens.len() != 2 {
            continue;
        }

        let key = tokens[0].trim().to_ascii_lowercase();
        let value = tokens[1].trim();

        if key == "path" || key.chars().all(|c| c.is_ascii_digit()) {
            if looks_like_path(value) {
                paths.push(PathBuf::from(normalize_vdf_path(value)));
            }
        }
    }

    paths
}

pub(crate) fn parse_appmanifest(contents: &str) -> Option<AppManifest> {
    let mut app_id = None;
    let mut name = None;
    let mut install_dir = None;
    let mut size_bytes = None;

    for line in contents.lines() {
        let tokens = quoted_tokens(line);
        if tokens.len() != 2 {
            continue;
        }

        let key = tokens[0].trim().to_ascii_lowercase();
        let value = tokens[1].trim();

        match key.as_str() {
            "appid" => app_id = Some(value.to_string()),
            "name" => name = Some(value.to_string()),
            "installdir" => install_dir = Some(value.to_string()),
            "sizeondisk" => size_bytes = value.parse::<u64>().ok(),
            _ => {}
        }
    }

    Some(AppManifest {
        app_id: app_id?,
        name: name?,
        install_dir: install_dir?,
        size_bytes: size_bytes.unwrap_or(0),
    })
}

fn quoted_tokens(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in line.chars() {
        if in_quotes {
            if ch == '"' {
                tokens.push(current);
                current = String::new();
                in_quotes = false;
            } else {
                current.push(ch);
            }
        } else if ch == '"' {
            in_quotes = true;
        }
    }

    tokens
}

fn looks_like_path(value: &str) -> bool {
    value.contains(":\\") || value.contains(":/") || value.starts_with("\\\\")
}

fn normalize_vdf_path(value: &str) -> String {
    value.replace("\\\\", "\\")
}
