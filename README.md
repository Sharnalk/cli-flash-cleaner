# 🦀 CLI Flash Cleaner

A fast and lightweight Rust-based command-line utility for recursively scanning directories and optionally deleting files based on extensions and filename patterns.

---

## ✨ Features

- Recursive directory traversal
- Filter files by extension
- Filter files by filename pattern
- Case-sensitive or case-insensitive matching
- Dry-run mode (preview before deletion)
- Safe deletion mode
- Written in Rust for performance and safety

---

## Installation

_You need to have `Rust` and `Cargo` installed on your machine. Official installation steps [here.](https://www.rust-lang.org/tools/install)_

```bash
git clone https://github.com/Sharnalk/cli-flash-cleaner.git
cd cli-flash-cleaner
cargo build --release
```
Once the build finishes, the optimized binary will be located in the `target/release/` directory.

### Locate the Binary

Depending on your operating system, the file name differs:

- `Linux / macOS`: The file is named fclr (no extension).
- `Windows`: The file is named fclr.exe.

#### Linux / macOS

Move the binary to your local bin folder:
```bash
sudo mv target/release/fclr /usr/local/bin/fclr
```
#### Windows

Copy the file target\release\fclr.exe to a folder of your choice (e.g., C:\Tools\).

Add that folder (C:\Tools\) to your System PATH.

Search for "Edit the system environment variables" -> "Environment Variables" -> Select "Path" -> "Edit" -> "New" -> Paste your folder path.


Restart your terminal.

You can now run fclr --help from anywhere.

--- 

## Examples

### Simulation (Dry Run)
List all images (`.png`, `.jpg`, `.webp`) in the current directory without deleting them. This is the default behavior.
   
```bash
fclr --extension png,jpg,webp --dry-run
```

### Search in a specific directory
Search for files containing "log" in a specific folder (./server/logs), using case-insensitive matching (finds "Log", "LOG", "log").

```bash
fclr --path ./server/logs --name-contains "log" --ignore-case
```

### Delete by name pattern
Destructive Action: Delete all files containing "hello" in the current directory.   
```bash
fclr --name-contains hello --delete
```

### Delete by extension
Destructive Action: Delete all files with specific extensions in the current directory.
```bash
fclr --extension png,jpg,webp --delete
```

### Complex Cleaning
Delete all .tmp files in the Downloads folder, ignoring case.
```bash
fclr --path ~/Downloads --extension tmp --ignore-case --delete
```

| Option | Alias | Description | Default |
| :--- | :---: | :--- | :---: |
| `--path` | `-p` | The root directory for the search. | `.` (Current Dir) |
| `--extension` | `-e` | Filter by extension (e.g., `txt,png`). | None |
| `--name-contains` | `-n` | Filter files containing this pattern. | None |
| `--ignore-case` | `-i` | Perform a case-insensitive search. | `false` |
| `--delete` | | **Enable deletion** of found files. | `false` |
| `--dry-run` |`-d` | Enable simulation mode (preview only) | `false` |
