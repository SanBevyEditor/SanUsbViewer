use std::{collections::BTreeMap, env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=locales");
    let languages: Vec<serde_json::Value> =
        serde_json::from_str(&fs::read_to_string("locales/languages.json")?)?;
    let source: BTreeMap<String, String> =
        serde_json::from_str(&fs::read_to_string("locales/en.json")?)?;
    let mut generated = String::from("pub(crate) static LANGUAGES: &[Language] = &[\n");
    for language in &languages {
        let code = language["code"].as_str().ok_or("missing locale code")?;
        let name = language["name"].as_str().ok_or("missing native name")?;
        let english = language["english"].as_str().ok_or("missing English name")?;
        let rtl = language["rtl"].as_bool().unwrap_or(false);
        generated.push_str(&format!(
            "Language {{ code: {code:?}, name: {name:?}, english: {english:?}, rtl: {rtl} }},\n"
        ));
    }
    generated.push_str("];\npub(crate) static CATALOGS: &[&[(&str, &str)]] = &[\n");
    for language in &languages {
        let code = language["code"].as_str().ok_or("missing locale code")?;
        let path = format!("locales/{code}.json");
        let catalog: BTreeMap<String, String> = serde_json::from_str(&fs::read_to_string(&path)?)?;
        if !source.keys().eq(catalog.keys()) || catalog.values().any(|s| s.trim().is_empty()) {
            return Err(format!("Incomplete translation catalog: {path}").into());
        }
        generated.push_str("&[\n");
        for (key, value) in catalog {
            if key.matches("{count}").count() != value.matches("{count}").count() {
                return Err(format!("Invalid translation parameter in {path}: {key}").into());
            }
            generated.push_str(&format!("({key:?}, {value:?}),\n"));
        }
        generated.push_str("],\n");
    }
    generated.push_str("];\n");
    fs::write(
        PathBuf::from(env::var("OUT_DIR")?).join("locale_data.rs"),
        generated,
    )?;
    Ok(())
}
