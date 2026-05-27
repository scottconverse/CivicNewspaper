#!/bin/bash
# Fetch Ollama binaries for all platforms as Tauri sidecars.
# Pinned version: v0.3.14
set -eu

BIN_DIR="src-tauri/binaries"
mkdir -p "$BIN_DIR"

VERSION="v0.3.14"
BASE_URL="https://github.com/ollama/ollama/releases/download/$VERSION"
SHA_FILE="scripts/ollama-binaries-shas.txt"

get_sha256() {
  local file="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$file" | awk '{print $1}'
  else
    # Fallback to certutil on Windows if git bash utilities are missing
    certutil -hashfile "$file" SHA256 | grep -v "hash" | head -n 2 | tail -n 1 | tr -d ' \r\n'
  fi
}

verify_and_write() {
  local temp_file="$1"
  local triple="$2"
  local dest_binary="$3"

  # 1. Get expected SHA
  if [ ! -f "$SHA_FILE" ]; then
    echo "FAIL: SHA registry file missing at $SHA_FILE"
    exit 1
  fi
  local expected_sha
  expected_sha=$(grep "$triple" "$SHA_FILE" | awk '{print $1}' | tr -d '\r\n')
  if [ -z "$expected_sha" ]; then
    echo "FAIL: No pinned SHA found for triple $triple in $SHA_FILE"
    exit 1
  fi

  # 2. Get actual SHA
  local actual_sha
  actual_sha=$(get_sha256 "$temp_file")
  if [ "$actual_sha" != "$expected_sha" ]; then
    echo "FAIL: SHA256 mismatch for $triple! Expected: $expected_sha, Got: $actual_sha"
    rm -f "$temp_file"
    exit 1
  fi
  echo "SHA256 verified for $triple: $actual_sha"

  # 3. Size check
  local min_size=100000000
  if [[ "$triple" == *"apple-darwin"* ]]; then
    min_size=50000000
  fi
  local size
  size=$(wc -c < "$temp_file")
  if [ "$size" -lt "$min_size" ]; then
    echo "FAIL: Downloaded file size for $triple is too small ($size bytes), must be >= $min_size"
    rm -f "$temp_file"
    exit 1
  fi

  # 4. Write to final destination and make executable
  mv "$temp_file" "$dest_binary"
  chmod +x "$dest_binary"
  echo "Successfully wrote $dest_binary"
}

# 1. macOS Intel
echo "Downloading macOS Intel binary..."
TEMP_DARWIN_X86=$(mktemp "$BIN_DIR/tmp.XXXXXX")
curl -L -o "$TEMP_DARWIN_X86" "$BASE_URL/ollama-darwin"
verify_and_write "$TEMP_DARWIN_X86" "x86_64-apple-darwin" "$BIN_DIR/ollama-x86_64-apple-darwin"

# 2. macOS Apple Silicon
echo "Downloading macOS Apple Silicon binary..."
TEMP_DARWIN_AARCH=$(mktemp "$BIN_DIR/tmp.XXXXXX")
curl -L -o "$TEMP_DARWIN_AARCH" "$BASE_URL/ollama-darwin"
verify_and_write "$TEMP_DARWIN_AARCH" "aarch64-apple-darwin" "$BIN_DIR/ollama-aarch64-apple-darwin"

# 3. Linux
echo "Downloading Linux binary..."
TEMP_LINUX_TGZ=$(mktemp "$BIN_DIR/tmp.XXXXXX")
curl -L -o "$TEMP_LINUX_TGZ" "$BASE_URL/ollama-linux-amd64.tgz"

# Verify the tgz archive first
expected_linux_sha=$(grep "x86_64-unknown-linux-gnu" "$SHA_FILE" | awk '{print $1}' | tr -d '\r\n')
actual_linux_sha=$(get_sha256 "$TEMP_LINUX_TGZ")
if [ "$actual_linux_sha" != "$expected_linux_sha" ]; then
  echo "FAIL: SHA256 mismatch for Linux tgz! Expected: $expected_linux_sha, Got: $actual_linux_sha"
  rm -f "$TEMP_LINUX_TGZ"
  exit 1
fi
echo "SHA256 verified for Linux tgz: $actual_linux_sha"

# Extract bin/ollama from the tgz
TEMP_LINUX_BIN=$(mktemp "$BIN_DIR/tmp.XXXXXX")
tar -xzf "$TEMP_LINUX_TGZ" ./bin/ollama -O > "$TEMP_LINUX_BIN"
rm -f "$TEMP_LINUX_TGZ"

# Size check the extracted binary
linux_bin_size=$(wc -c < "$TEMP_LINUX_BIN")
if [ "$linux_bin_size" -lt 100000000 ]; then
  echo "FAIL: Extracted Linux binary size is too small ($linux_bin_size bytes), must be >= 100MB"
  rm -f "$TEMP_LINUX_BIN"
  exit 1
fi

mv "$TEMP_LINUX_BIN" "$BIN_DIR/ollama-x86_64-unknown-linux-gnu"
chmod +x "$BIN_DIR/ollama-x86_64-unknown-linux-gnu"
echo "Successfully wrote $BIN_DIR/ollama-x86_64-unknown-linux-gnu"

# 4. Windows
echo "Downloading Windows binary..."
TEMP_WIN_ZIP=$(mktemp "$BIN_DIR/tmp.XXXXXX")
curl -L -o "$TEMP_WIN_ZIP" "$BASE_URL/ollama-windows-amd64.zip"

# Verify the zip archive first
expected_win_sha=$(grep "x86_64-pc-windows-msvc" "$SHA_FILE" | awk '{print $1}' | tr -d '\r\n')
actual_win_sha=$(get_sha256 "$TEMP_WIN_ZIP")
if [ "$actual_win_sha" != "$expected_win_sha" ]; then
  echo "FAIL: SHA256 mismatch for Windows zip! Expected: $expected_win_sha, Got: $actual_win_sha"
  rm -f "$TEMP_WIN_ZIP"
  exit 1
fi
echo "SHA256 verified for Windows zip: $actual_win_sha"

# Extract ollama.exe from the zip
TEMP_WIN_BIN=$(mktemp "$BIN_DIR/tmp.XXXXXX")
unzip -p "$TEMP_WIN_ZIP" ollama.exe > "$TEMP_WIN_BIN"
rm -f "$TEMP_WIN_ZIP"

# Size check the extracted binary
win_bin_size=$(wc -c < "$TEMP_WIN_BIN")
if [ "$win_bin_size" -lt 25000000 ]; then
  echo "FAIL: Extracted Windows binary size is too small ($win_bin_size bytes), must be >= 25MB"
  rm -f "$TEMP_WIN_BIN"
  exit 1
fi

mv "$TEMP_WIN_BIN" "$BIN_DIR/ollama-x86_64-pc-windows-msvc.exe"
chmod +x "$BIN_DIR/ollama-x86_64-pc-windows-msvc.exe"
echo "Successfully wrote $BIN_DIR/ollama-x86_64-pc-windows-msvc.exe"

echo "Ollama binaries fetch complete!"
