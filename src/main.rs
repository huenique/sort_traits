use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <project_path>", args[0]);
        return;
    }

    let project_path = Path::new(&args[1]);

    if !project_path.exists() {
        eprintln!("The specified project path does not exist.");
        return;
    }

    let re = Regex::new(r"(?m)^\s*#\[(derive\(([^)]*)\))\]$").unwrap();

    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_map(Result::ok)
    {
        if is_rust_file(entry.path()) {
            process_file(entry.path(), &re);
        }
    }
}

fn is_rust_file(path: &Path) -> bool {
    path.is_file() && path.extension().map_or(false, |ext| ext == "rs")
}

fn process_file(file_path: &Path, re: &Regex) {
    if let Ok(content) = fs::read_to_string(file_path) {
        let new_content = re.replace_all(&content, |caps: &regex::Captures| {
            let mut traits: Vec<&str> = caps[2]
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
            traits.sort();
            format!("#[derive({})]", traits.join(", "))
        });

        if new_content != content {
            fs::write(file_path, new_content.to_string()).expect("Unable to write file");
            println!("Sorted derived traits in {}", file_path.display());
        }
    }
}
