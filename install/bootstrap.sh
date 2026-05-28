#!/usr/bin/env sh
set -eu

VERSION="${VERSION:-latest}"
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64|amd64) ARCH="x64" ;;
  aarch64|arm64) ARCH="arm64" ;;
  *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
esac

PLATFORM="${OS}-${ARCH}"
ARCHIVE="brain-${PLATFORM}.tar.gz"

if [ "$VERSION" = "latest" ]; then
  BASE_URL="https://github.com/Oerum/oerum-agent/releases/latest/download"
else
  BASE_URL="https://github.com/Oerum/oerum-agent/releases/download/${VERSION}"
fi

URL="${BASE_URL}/${ARCHIVE}"
SUM_URL="${URL}.sha256"
DEST="${HOME}/.brain/bin"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$DEST"

echo "Downloading ${URL}"
if ! curl -fsSL "$URL" -o "${TMP}/${ARCHIVE}"; then
  cat >&2 <<EOF
Release asset not found at:
  ${URL}

No matching GitHub release asset is currently published.

Maintainer path:
  Publish a release containing '${ARCHIVE}' and '${ARCHIVE}.sha256'.

Local fallback:
  cargo install --path crates/brain-cli --locked --root "${HOME}/.brain"
  Then run: brain init
EOF
  exit 1
fi

if ! curl -fsSL "$SUM_URL" -o "${TMP}/${ARCHIVE}.sha256"; then
  cat >&2 <<EOF
Release checksum file not found at:
  ${SUM_URL}

Publish '${ARCHIVE}.sha256' with the release, or install locally:
  cargo install --path crates/brain-cli --locked --root "${HOME}/.brain"
EOF
  exit 1
fi

EXPECTED="$(awk '{print tolower($1)}' "${TMP}/${ARCHIVE}.sha256")"
if [ -z "$EXPECTED" ]; then
  echo "Empty checksum file at $SUM_URL" >&2
  exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL="$(sha256sum "${TMP}/${ARCHIVE}" | awk '{print tolower($1)}')"
else
  ACTUAL="$(shasum -a 256 "${TMP}/${ARCHIVE}" | awk '{print tolower($1)}')"
fi

if [ "$EXPECTED" != "$ACTUAL" ]; then
  echo "Checksum mismatch for ${ARCHIVE}" >&2
  echo "  expected: $EXPECTED" >&2
  echo "  actual:   $ACTUAL"   >&2
  exit 1
fi
echo "Checksum verified ($ACTUAL)"

tar -xzf "${TMP}/${ARCHIVE}" -C "$DEST"
chmod +x "${DEST}/brain" 2>/dev/null || true

echo ""
echo "Installed brain to ${DEST}"

# Path guidance: never silently edit shell rc files; advise instead.
case ":$PATH:" in
  *":${DEST}:"*) ;;
  *)
    echo ""
    echo "Add this to your shell rc (e.g. ~/.zshrc, ~/.bashrc) and reopen your shell:"
    echo "  export PATH=\"\$HOME/.brain/bin:\$PATH\""
    ;;
esac

echo ""
echo "Next: brain init"
