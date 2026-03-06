#!/usr/bin/env bash
set -e

# ── Couleurs ──────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

log()  { echo -e "${CYAN}▶ $1${NC}"; }
ok()   { echo -e "${GREEN}✔ $1${NC}"; }
warn() { echo -e "${YELLOW}⚠ $1${NC}"; }
die()  { echo -e "${RED}✖ $1${NC}"; exit 1; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# ── 1. Rustup / toolchain ─────────────────────────────────────────────────────
if ! command -v rustup &>/dev/null; then
    log "Installation de rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    source "$HOME/.cargo/env"
    ok "rustup installé"
else
    source "$HOME/.cargo/env" 2>/dev/null || true
    ok "rustup présent"
fi

# ── 2. Target WASM ───────────────────────────────────────────────────────────
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    log "Ajout de la target wasm32-unknown-unknown..."
    rustup target add wasm32-unknown-unknown
    ok "target wasm32 installée"
else
    ok "target wasm32 présente"
fi

# ── 3. wasm-pack ─────────────────────────────────────────────────────────────
if ! command -v wasm-pack &>/dev/null; then
    log "Installation de wasm-pack..."
    cargo install wasm-pack
    ok "wasm-pack installé"
else
    ok "wasm-pack présent ($(wasm-pack --version))"
fi

# ── 4. Serveur HTTP ───────────────────────────────────────────────────────────
# On préfère python3 (dispo partout), sinon npx serve
if command -v python3 &>/dev/null; then
    SERVER="python3"
elif command -v npx &>/dev/null; then
    SERVER="npx"
else
    die "Aucun serveur HTTP trouvé (python3 ou npx requis)"
fi

# ── 5. Build ──────────────────────────────────────────────────────────────────
log "Compilation WASM (mode dev)..."
wasm-pack build --target web --out-dir www --dev
cp index.html www/
ok "Build terminé → www/"

# ── 6. Lancement ─────────────────────────────────────────────────────────────
PORT=8080
URL="http://localhost:$PORT"

log "Démarrage du serveur sur $URL ..."
echo ""
echo -e "  ${GREEN}Ouvre ton navigateur sur : ${CYAN}$URL${NC}"
echo -e "  ${YELLOW}Ctrl+C pour arrêter${NC}"
echo ""

# Ouvre le navigateur automatiquement après 1s
(sleep 1 && \
    if command -v xdg-open &>/dev/null; then xdg-open "$URL"; \
    elif command -v open &>/dev/null; then open "$URL"; \
    fi) &

if [ "$SERVER" = "python3" ]; then
    python3 -m http.server $PORT --directory www
else
    npx serve www -p $PORT
fi