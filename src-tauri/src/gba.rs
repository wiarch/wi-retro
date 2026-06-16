use std::path::Path;

pub const SCAN_EXTENSIONS: &[&str] = &["gba", "zip"];

pub fn is_scannable_rom(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let lower = ext.to_ascii_lowercase();
            SCAN_EXTENSIONS.contains(&lower.as_str())
        })
        .unwrap_or(false)
}

pub fn game_id(path: &Path) -> String {
    blake3::hash(path.to_string_lossy().as_bytes())
        .to_hex()
        .to_string()
}

/// Limpia el nombre de archivo para usarlo como título provisional.
pub fn title_from_filename(path: &Path) -> String {
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let mut title = stem.replace(['_', '.'], " ");

    for token in [
        "(USA)", "(Europe)", "(Japan)", "(World)", "(J)", "(U)", "(E)", "(W)",
        "(Rev 1)", "(Rev 2)", "(Rev A)", "[!]", "[a]", "[t1]",
        "(En,Fr,De)", "(En,Fr,De,Es,It)", "(En)", "(Fr)", "(De)", "(Es)", "(It)",
    ] {
        title = title.replace(token, "");
    }

    while title.contains("  ") {
        title = title.replace("  ", " ");
    }

    title.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn cleans_rom_filename() {
        let path = PathBuf::from("/roms/Pokemon_Emerald_(USA).gba");
        assert_eq!(title_from_filename(&path), "Pokemon Emerald");
    }
}
