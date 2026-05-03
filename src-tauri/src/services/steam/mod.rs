//! Descobre jogos Steam via registo, `libraryfolders.vdf` e manifestos `.acf`.

mod parser;
mod path;
mod scanner;

pub use path::get_steam_path;
pub use scanner::scan_steam_games;
