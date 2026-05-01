# ledger

A minimal command-line task tracker. Stores everything locally in `~/.ledger/ledger.json`.

![CI](https://github.com/striberry/ledger/actions/workflows/ci.yml/badge.svg)
![Release](https://img.shields.io/github/v/release/striberry/ledger)
![License](https://img.shields.io/github/license/striberry/ledger)

## Install


## Linux

**Package Manager (Repology)**

[![Packaging status](https://repology.org/badge/vertical-allrepos/ledger-cli.svg)](https://repology.org/project/ledger-cli/versions)

**Pre-built binary**
```sh
curl -sL https://github.com/striberry/ledger/releases/latest/download/ledger-latest-x86_64-unknown-linux-gnu.tar.gz | tar -xz
sudo mv ledger /usr/local/bin/
```

**Cargo**
```sh
cargo install --git https://github.com/striberry/ledger
```

## macOS

**Pre-built binary**
```sh
# Apple Silicon
curl -sL https://github.com/striberry/ledger/releases/latest/download/ledger-latest-aarch64-apple-darwin.tar.gz | tar -xz

# Intel
curl -sL https://github.com/striberry/ledger/releases/latest/download/ledger-latest-x86_64-apple-darwin.tar.gz | tar -xz

sudo mv ledger /usr/local/bin/
```

> **Note:** macOS may block the binary on first run. If so, run `xattr -d com.apple.quarantine /usr/local/bin/ledger` to clear it.

**Cargo**
```sh
cargo install --git https://github.com/striberry/ledger
```

 ## Windows

> NOTE: Windows issues won't be handled. PRs are fine as long as they don't break MacOS / Linux compatibility.

**Pre-built binary**

Download `ledger-latest-x86_64-pc-windows-msvc.zip` from the [releases page](https://github.com/striberry/ledger/releases/latest), extract it, and move `ledger.exe` somewhere on your `PATH`.

**Cargo**
```powershell
cargo install --git https://github.com/striberry/ledger
```

## Build from source

```sh
git clone https://github.com/striberry/ledger
cd ledger
cargo build --release
```

Binary will be at `target/release/ledger` (or `ledger.exe` on Windows).

## Commands

| Command | Description |
|---|---|
| `ledger add <text>` | Add a new task |
| `ledger list` | List all open tasks |
| `ledger done <id>` | Mark a task as done |
| `ledger remove <id>` | Permanently delete a task |
| `ledger history` | Show all tasks including completed ones |

## Examples

```
$ ledger add "buy groceries"
Added [#1] buy groceries

$ ledger add "fix the bug"
Added [#2] fix the bug

$ ledger list
  ID  Task
──────────────────
   1  buy groceries
   2  fix the bug

$ ledger done 1
Done [#1] buy groceries

$ ledger remove 2
Removed [#2] fix the bug

$ ledger history
  ID  Status  Task
────────────────────────
   1  done    buy groceries
```

## Feature Ideas
- [ ] Tags
- [ ] Due Dates
- [ ] Json Output
- [ ] Filtering
- [ ] Editing Tasks
- [ ] Ledger Clearing
- [ ] Import / Export
- [ ] Git-friendly storage format
