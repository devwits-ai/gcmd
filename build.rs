fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "nlp")]
    build_nlp()?;

    Ok(())
}

#[cfg(feature = "nlp")]
fn build_nlp() -> Result<(), nlprule_build::Error> {
    println!("cargo:rerun-if-changed=build.rs");

    let mut languages = vec![];

    #[cfg(feature = "lang-en")]
    languages.push("en");
    #[cfg(feature = "lang-fr")]
    languages.push("fr");
    #[cfg(feature = "lang-de")]
    languages.push("de");
    #[cfg(feature = "lang-es")]
    languages.push("es");

    if languages.is_empty() {
        println!("cargo:warning=No language feature enabled. Defaulting to English.");
        languages.push("en");
    }

    nlprule_build::BinaryBuilder::new(
        &languages,
        std::env::var("OUT_DIR").expect("OUT_DIR is set when build.rs is running"),
    )
    .build()?
    .validate()
}
