# Duplicate File Cleaner CLI

A Rust-based command-line tool to scan and remove duplicate files in a directory based on file name and size.

## Features

- Scans the current or user-specified directory and its subdirectories.

- Identifies duplicate files with the same name and size.

- Provides options to:

- Delete all duplicate files.

- Move duplicate files to a separate folder named `duplicates`.

## Installation

### Prerequisites

- Install Rust and Cargo: [Rust Installation Guide](https://www.rust-lang.org/tools/install)

### Build the Binary

```sh
cargo  build  --release
```

The compiled binary will be available at `target/release/mem_cleaner`.

### Run the CLI

To execute the tool:

```sh
./target/release/mem_cleaner
```

(Windows users should use `.\target\release\mem_cleaner.exe`)

## Usage

- **Choose the directory to scan**
  - The tool prompts whether to scan the current directory or enter a custom directory path.
- **Scanning for duplicate files**
  - The tool identifies duplicate files based on their name and size.
- **Choose an action**
  - **Delete All:** Permanently removes all duplicate files.
  - **Move to Separate Folder:** Moves duplicates to a `duplicates` folder inside the chosen directory.
  - **Cancel:** Exits without any modifications.

## Distributing the Binary

To share the CLI with other users, create a standalone executable:

```sh

cargo  build  --release

```

Then, distribute the binary found in `target/release/mem_cleaner` (or `.exe` on Windows).

---

To trigger your CLI tool like `mem_cleaner` from anywhere, follow these steps:

### 1. **Move Binary to System Path**

To use `mem_cleaner` from anywhere, move it to a directory in your system `PATH`:

#### **Linux/macOS**

```sh

sudo  mv  target/release/mem_cleaner  /usr/local/bin/

```

#### **Windows**

1. Move the binary to a known location:

```powershell

Move-Item  -Path target\release\mem_cleaner.exe  -Destination C:\Program Files\mem_cleaner.exe

```

2. Add `C:\Program Files\` to your system `PATH`:

- Open **Environment Variables** (`sysdm.cpl` → Advanced → Environment Variables).

- Edit the `Path` variable under **System Variables**.

- Add `C:\Program Files\`.

- Click OK and restart the terminal.

### 2. **Run the CLI**

Now, you can run your tool from anywhere:

```sh

mem_cleaner

```

Let me know if you need any modifications! 🚀
