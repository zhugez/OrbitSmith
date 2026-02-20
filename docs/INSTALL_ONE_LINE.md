# OrbitSmith One-line Install

## Quick install
```bash
bash -c "$(curl -fsSL https://raw.githubusercontent.com/zhugez/orbitsmith/master/install/bootstrap.sh)"
```

## Custom path
```bash
bash -c "$(curl -fsSL https://raw.githubusercontent.com/zhugez/orbitsmith/master/install/bootstrap.sh)" -- "$HOME/tools/orbitsmith"
```

## Verify
```bash
export PATH="$HOME/.local/bin:$PATH"
orbitsmith help
```
