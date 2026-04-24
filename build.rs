use std::{collections::HashMap, fs, fs::File, io::Write, path::Path};
use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=.");
    let project_root = Path::new(".");

    let mut total_lines = 0usize;
    let mut lines_per_ext: HashMap<String, usize> = HashMap::new();

    // Change here according to taste
    // Possibilities: rs, html, sql, toml, js, css, md, etc.
    //let allowed = ["rs", "html", "css", "js", "md", "sql", "toml"];
    let allowed = ["rs"];

    let mut output = File::create(".index.txt").expect("Failed to create output file");

    for entry in WalkDir::new(project_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            let p = e.path().to_string_lossy();
            !p.contains("target") && !p.contains(".git")
        })
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if !allowed.contains(&ext) {
                continue;
            }
        } else {
            continue;
        }

        let relative_path = path.strip_prefix(project_root).unwrap();

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let escaped = content.replace("\"", "\\\"");

        let line_count = content.lines().count();

        total_lines += line_count;

        // Extension counting
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let counter = lines_per_ext.entry(ext.to_string()).or_insert(0);

            *counter += line_count;
        }

        // Output format
        writeln!(
            output,
            "{}/{}:",
            relative_path
                .parent()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "".to_string()),
            relative_path.file_name().unwrap().to_string_lossy()
        )
        .unwrap();

        writeln!(output, "\"{}\"", escaped).unwrap();
        writeln!(output).unwrap();
    }

    // Summary
    println!("Index invoked");
    writeln!(output, "====================").unwrap();
    writeln!(output, "Total lines: {}", total_lines).unwrap();

    writeln!(output, "Lines per file type:").unwrap();

    for (ext, count) in lines_per_ext {
        writeln!(output, ".{}: {}", ext, count).unwrap();
    }
}
