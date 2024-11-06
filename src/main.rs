use colored::*;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::error::Error;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

struct TreeOptions {
    gitignore: Option<Gitignore>,
    root_path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Start from current directory if no argument is provided
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let root_path = Path::new(&path).canonicalize()?;

    // Try to load .gitignore if it exists
    let gitignore = load_gitignore(&root_path)?;

    let options = TreeOptions {
        gitignore,
        root_path: root_path.clone(),
    };

    // Print root directory without the └── prefix
    let root_name = root_path
        .file_name()
        .unwrap_or(root_path.as_os_str())
        .to_string_lossy();
    println!("{}/", root_name.cyan());

    print_directory_contents(&root_path, "", &options)?;
    Ok(())
}

fn load_gitignore(root_path: &Path) -> Result<Option<Gitignore>, Box<dyn Error>> {
    let mut builder = GitignoreBuilder::new(root_path);

    // Add built-in rules first
    builder.add_line(None, "**/.git/")?;

    // Then add user's .gitignore if it exists
    let gitignore_path = root_path.join(".gitignore");
    if gitignore_path.exists() {
        builder.add(gitignore_path);
    }

    Ok(Some(builder.build()?))
}

fn should_ignore(path: &Path, options: &TreeOptions) -> bool {
    if let Some(ref gitignore) = options.gitignore {
        if let Ok(relative_path) = path.strip_prefix(&options.root_path) {
            return gitignore.matched(relative_path, path.is_dir()).is_ignore();
        }
    }
    false
}

#[cfg(unix)]
fn is_executable(metadata: &fs::Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable(_metadata: &fs::Metadata) -> bool {
    false // For Windows, we could check file extensions like .exe, .bat, etc.
}

fn print_directory_contents(
    path: &Path,
    prefix: &str,
    options: &TreeOptions,
) -> Result<(), Box<dyn Error>> {
    let entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| !should_ignore(&e.path(), options))
        .collect();

    let mut entries = entries;
    entries.sort_by_key(|e| {
        let is_dir = e.file_type().map(|ft| !ft.is_dir()).unwrap_or(true);
        (is_dir, e.file_name())
    });

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let path = entry.path();
        let metadata = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().to_string();

        // Determine the display style based on file type
        let display_name = if metadata.is_dir() {
            format!("{}/", name).cyan().to_string()
        } else if is_executable(&metadata) {
            name.green().to_string()
        } else {
            name.to_string()
        };

        println!(
            "{}{}{}",
            prefix,
            if is_last { "└── " } else { "├── " },
            display_name
        );

        if metadata.is_dir() {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            print_directory_contents(&path, &new_prefix, options)?;
        }
    }

    Ok(())
}
