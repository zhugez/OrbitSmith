<p align="center">
  <img src="assets/banner.svg" alt="OrbitSmith banner" width="100%" />
</p>

<h1 align="center">OrbitSmith 🛰️</h1>
<p align="center"><b>Antigravity-focused Rust toolkit (v0.9.1 beta)</b> for clean, fast, multi-platform workflows.</p>

<p align="center">
  <img alt="Release" src="https://img.shields.io/github/v/release/zhugez/orbitsmith?include_prereleases&label=release"/>
  <img alt="License" src="https://img.shields.io/badge/license-MIT-blue"/>
  <img alt="Platform" src="https://img.shields.io/badge/platform-linux%20%7C%20windows-22c55e"/>
  <img alt="Focus" src="https://img.shields.io/badge/focus-antigravity-7c3aed"/>
</p>

---

## ⚡ Install (one-liner)

Installs OrbitSmith to `~/.orbitsmith/bin/` and **automatically adds it to your PATH**.

### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/zhugez/OrbitSmith/master/install/install.sh | bash
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/zhugez/OrbitSmith/master/install/install.ps1 | iex
```

> After install, restart your terminal and run `orbitsmith --help` from anywhere! 🚀

---

## 🚀 Quick start

```bash
mkdir my-project && cd my-project
orbitsmith init
orbitsmith sync-skills
orbitsmith doctor
```

---

## 🧰 Commands

- `orbitsmith init` — initialize workspace
- `orbitsmith sync-skills` — sync skills from bundled kit
- `orbitsmith doctor` — verify local prerequisites
- `orbitsmith update` — pull latest OrbitSmith changes

---

## 📦 What’s inside

```text
orbitsmith/
├─ src/             # Rust CLI source
├─ bin/             # launcher/wrapper
├─ install/         # installers and bootstrap
├─ kit/             # canonical Antigravity-focused kit
├─ docs/            # quickstart + install docs
└─ assets/          # branding assets
```

---

## 🤝 Open Source

- [License](LICENSE)
- [Contributing](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

**Scope:** OrbitSmith currently focuses on **Antigravity workflows only**.
