use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() {
    let articles_dir = Path::new("articles/published");
    let out_file = Path::new("src/utils/generated.rs");

    println!("cargo:rerun-if-changed=articles/published");

    let mut entries = fs::read_dir(articles_dir)
        .expect("Failed to read articles directory")
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
        .collect::<Vec<_>>();

    entries.sort_by_key(|e| e.path());

    let mut file = File::create(out_file).expect("Failed to create generated.rs");

    writeln!(file, "// AUTO-GENERATED — DO NOT EDIT\n").unwrap();
    /*
    pub const ARTICLES: &[(&str, &str)] = &[
        ("post-01", include_str!("post-01.md")),
       // ("post-02", include_str!("post-02.md")),
       // ("post-03", include_str!("post-03.md")),
    ];
        */
    writeln!(file, "pub const ARTICLES: &[(&str, &str)] = &[").unwrap();
    for i in entries {
        writeln!(
            file,
            "   (\"{}\", include_str!(\"../../articles/published/{}\")),",
            i.path()
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                .as_str(),
            i.path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                .as_str()
        )
        .unwrap();
    }
    writeln!(file, "];").unwrap();
}
