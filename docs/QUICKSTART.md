# Quickstart (Rust, multi-platform)

## 1) Clone
```bash
git clone https://github.com/zhugez/orbitsmith.git
cd orbitsmith
```

## 2) Install Rust (if missing)
- Linux/macOS: https://rustup.rs
- Windows: rustup-init.exe

## 3) Build + install CLI
```bash
./install/install.sh "$PWD"
export PATH="$HOME/.local/bin:$PATH"
```

## 4) Use
```bash
orbitsmith init
orbitsmith sync-skills
orbitsmith doctor
```
