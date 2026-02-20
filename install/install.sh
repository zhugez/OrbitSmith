#!/usr/bin/env bash
set -euo pipefail

# ============================================================================
#  OrbitSmith Installer for Linux/macOS
#  Usage: curl -fsSL https://raw.githubusercontent.com/zhugez/OrbitSmith/master/install/install.sh | bash
# ============================================================================

REPO="zhugez/OrbitSmith"
BINARY_NAME="orbitsmith"
INSTALL_DIR="$HOME/.orbitsmith/bin"
VERSION="${ORBITSMITH_VERSION:-latest}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

info()  { echo -e "${CYAN}ℹ ${NC} $*"; }
ok()    { echo -e "${GREEN}✅${NC} $*"; }
warn()  { echo -e "${YELLOW}⚠️ ${NC} $*"; }
err()   { echo -e "${RED}❌${NC} $*"; exit 1; }

echo ""
echo -e "${CYAN}╔══════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║       🛰️  OrbitSmith Installer  🛰️       ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════╝${NC}"
echo ""

# Detect OS and architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux*)  PLATFORM="linux" ;;
  darwin*) PLATFORM="darwin" ;;
  *)       err "Unsupported OS: $OS" ;;
esac

case "$ARCH" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *)             err "Unsupported architecture: $ARCH" ;;
esac

ASSET_NAME="${BINARY_NAME}-${PLATFORM}-${ARCH}"
info "Detected platform: ${PLATFORM}/${ARCH}"

# Resolve version
if [ "$VERSION" = "latest" ]; then
  info "Fetching latest release..."
  DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ASSET_NAME}"
else
  DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET_NAME}"
fi

# Create install directory
mkdir -p "$INSTALL_DIR"

# Download binary
info "Downloading OrbitSmith from $DOWNLOAD_URL ..."
if command -v curl &>/dev/null; then
  curl -fsSL "$DOWNLOAD_URL" -o "${INSTALL_DIR}/${BINARY_NAME}"
elif command -v wget &>/dev/null; then
  wget -q "$DOWNLOAD_URL" -O "${INSTALL_DIR}/${BINARY_NAME}"
else
  err "Neither curl nor wget found. Please install one of them."
fi

# Make executable
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
ok "Binary installed to ${INSTALL_DIR}/${BINARY_NAME}"

# Add to PATH
add_to_path() {
  local shell_config="$1"
  local export_line="export PATH=\"\$HOME/.orbitsmith/bin:\$PATH\""

  if [ -f "$shell_config" ] && grep -q ".orbitsmith/bin" "$shell_config" 2>/dev/null; then
    info "PATH already configured in $(basename "$shell_config")"
    return
  fi

  echo "" >> "$shell_config"
  echo "# OrbitSmith" >> "$shell_config"
  echo "$export_line" >> "$shell_config"
  ok "Added to PATH in $(basename "$shell_config")"
}

# Detect shell and update config
SHELL_NAME="$(basename "$SHELL")"
case "$SHELL_NAME" in
  bash)
    add_to_path "$HOME/.bashrc"
    [ -f "$HOME/.bash_profile" ] && add_to_path "$HOME/.bash_profile"
    ;;
  zsh)
    add_to_path "$HOME/.zshrc"
    ;;
  fish)
    mkdir -p "$HOME/.config/fish"
    FISH_CONFIG="$HOME/.config/fish/config.fish"
    if ! grep -q ".orbitsmith/bin" "$FISH_CONFIG" 2>/dev/null; then
      echo "" >> "$FISH_CONFIG"
      echo "# OrbitSmith" >> "$FISH_CONFIG"
      echo "set -gx PATH \$HOME/.orbitsmith/bin \$PATH" >> "$FISH_CONFIG"
      ok "Added to PATH in config.fish"
    fi
    ;;
  *)
    warn "Unknown shell ($SHELL_NAME). Please add this to your shell config manually:"
    echo "  export PATH=\"\$HOME/.orbitsmith/bin:\$PATH\""
    ;;
esac

# Add to current session
export PATH="$HOME/.orbitsmith/bin:$PATH"

# Verify installation
echo ""
if command -v "$BINARY_NAME" &>/dev/null; then
  ok "OrbitSmith installed successfully! 🚀"
  echo ""
  "$BINARY_NAME" --version 2>/dev/null || true
  echo ""
  echo -e "${GREEN}Get started:${NC}"
  echo "  orbitsmith init         # Initialize workspace with 865+ AI skills"
  echo "  orbitsmith sync-skills  # Sync latest skills"
  echo "  orbitsmith status       # Check workspace status"
  echo ""
  warn "Restart your terminal or run: source ~/.${SHELL_NAME}rc"
else
  warn "Binary installed but not yet in PATH for this session."
  warn "Restart your terminal, then run: orbitsmith --help"
fi
