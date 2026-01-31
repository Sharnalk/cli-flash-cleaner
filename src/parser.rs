use crate::args::Args;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

/// Extracts the file extension from the given filename.
fn get_extension_from_filename(filename: &OsStr) -> String {
    Path::new(filename)
        .extension()
        .and_then(|osstr| osstr.to_str())
        .map(|osstr| osstr.to_string())
        .unwrap_or_default()
}

/// Evaluates whether a directory entry matches the provided filter criteria.
/// Checks file type, extension, and name pattern (supporting case-insensitivity).
fn filter_args(args: &Args, entry: &DirEntry) -> bool {
    if !entry.file_type().is_file() {
        return false;
    }

    let extensions_matches = args.extension.is_empty()
        || args
            .extension
            .contains(&get_extension_from_filename(entry.file_name()));

    let filename_to_check = if args.ignore_case {
        entry.file_name().to_string_lossy().to_lowercase()
    } else {
        entry.file_name().to_string_lossy().to_string()
    };

    let name_contains_lowered = args.name_contains.as_deref().map(|s| {
        if args.ignore_case {
            s.to_lowercase()
        } else {
            s.to_owned()
        }
    });

    let name_matches = match &name_contains_lowered {
        Some(motif) => filename_to_check.contains(motif),
        None => true,
    };

    name_matches && extensions_matches
}

/// Creates an iterator that traverses the directory tree and yields only files matching the filters.
fn filter_folder(args: &Args) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(&args.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| filter_args(args, e))
}

/// Iterates through the filtered files to either print (dry-run) or delete them.
/// Performed actions depend on the `delete` and `dry_run` flags in arguments.
fn try_delete_files(walker: impl Iterator<Item = DirEntry>, args: &Args) {
    let mut count: i32 = 0;

    for entry in walker {
        if !filter_args(args, &entry) {
            continue;
        }

        if args.dry_run || args.delete {
            println!("{}", entry.file_name().to_string_lossy());
        }
        // try to delete only if user don't specify dry_run
        if !args.dry_run && args.delete {
            match fs::remove_file(entry.path()) {
                Ok(_) => count += 1,
                Err(e) => println!("Error : {}", e),
            }
        }
    }

    if args.delete {
        println!("[{}] - Files deleted.", count);
    }
}

/// Main entry point for the application logic.
/// Orchestrates file discovery and processing based on the parsed arguments.
pub fn cli_parser(args: &Args) {
    let walker = filter_folder(args);

    if args.dry_run || args.delete {
        try_delete_files(walker, args);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn walker_try_delete(args: &Args) {
        let walker = filter_folder(args);

        try_delete_files(walker, &args);
    }

    #[test]
    fn get_extension_returns_jpg_for_simple_file() {
        let filename = OsStr::new("photo.jpg");
        let result = get_extension_from_filename(filename);
        assert_eq!(result, "jpg");
    }

    #[test]
    fn get_extension_returns_last_extension_for_double_extension() {
        let result = get_extension_from_filename(OsStr::new("a.tar.gz"));
        assert_eq!(result, "gz");
    }
    #[test]
    fn get_extension_returns_empty_for_file_without_extension() {
        let result = get_extension_from_filename(OsStr::new("gitignore"));
        assert_eq!(result, "");
    }

    #[test]
    fn get_extension_returns_empty_for_dotfile() {
        let result = get_extension_from_filename(OsStr::new(".env"));
        assert_eq!(result, "");
    }
    #[test]
    fn try_delete_files_does_not_remove_files_in_dry_run_mode() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::write(root.join("a.txt"), "Meow1").unwrap();

        let args = Args {
            path: root.to_path_buf(),
            extension: vec![],
            name_contains: None,
            ignore_case: false,
            delete: true,
            dry_run: true,
        };

        let walker = WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());

        try_delete_files(walker, &args);

        assert!(root.join("a.txt").exists());
    }

    #[test]
    fn deletes_only_files_matching_name_filter() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::write(root.join("a.txt"), "Meow1").unwrap();
        fs::write(root.join("b.txt"), "Meow1").unwrap();

        let args = Args {
            path: root.to_path_buf(),
            extension: vec![],
            name_contains: Option::from("a".to_string()),
            ignore_case: false,
            delete: true,
            dry_run: false,
        };

        walker_try_delete(&args);
        assert!(!root.join("a.txt").exists());
        assert!(root.join("b.txt").exists());
    }

    #[test]
    fn case_sensitive_matching_does_not_delete_when_pattern_differs() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::write(root.join("aA.txt"), "Meow1").unwrap();

        let args = Args {
            path: root.to_path_buf(),
            extension: vec![],
            name_contains: Option::from("aa".to_string()),
            ignore_case: false,
            delete: true,
            dry_run: false,
        };

        walker_try_delete(&args);
        assert!(root.join("aA.txt").exists());
    }

    #[test]
    fn case_insensitive_matching_deletes_file() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::write(root.join("aA.txt"), "Meow1").unwrap();

        let args = Args {
            path: root.to_path_buf(),
            extension: vec![],
            name_contains: Option::from("aa".to_string()),
            ignore_case: true,
            delete: true,
            dry_run: false,
        };

        walker_try_delete(&args);
        assert!(!root.join("aA.txt").exists());
    }
    #[test]
    fn try_delete_files_removes_all_files_when_delete_is_true() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::write(root.join("a.txt"), "Meow1").unwrap();
        fs::write(root.join("b.txt"), "hxh >>>").unwrap();

        let args = Args {
            path: root.to_path_buf(),
            extension: vec![],
            name_contains: None,
            ignore_case: false,
            delete: true,
            dry_run: false,
        };

        walker_try_delete(&args);

        assert!(!root.join("a.txt").exists());
        assert!(!root.join("b.txt").exists());
    }

    #[test]
    fn try_delete_files_does_not_delete_when_filter_changes_after_walker_creation() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("a.txt"), "Meow2").unwrap();

        let args1 = Args {
            path: root.to_path_buf(),
            extension: vec!["txt".to_string()],
            name_contains: None,
            ignore_case: false,
            delete: true,
            dry_run: false,
        };

        let walker = filter_folder(&args1);

        let args2 = Args {
            path: root.to_path_buf(),
            extension: vec!["ttx".to_string()], //extension changed during filter
            name_contains: None,
            ignore_case: false,
            delete: true,
            dry_run: false,
        };

        try_delete_files(walker, &args2);

        assert!(root.join("a.txt").exists());
    }
}
