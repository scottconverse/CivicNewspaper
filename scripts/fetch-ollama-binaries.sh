#!/bin/bash
# Fetch Ollama binaries for all platforms as Tauri sidecars.
# Optimizes download size by only fetching the real binary for the host platform,
# and writing mock placeholder files for other platforms.
set -eu

BIN_DIR="src-tauri/binaries"
mkdir -p "$BIN_DIR"

VERSION="v0.1.48"
BASE_URL="https://github.com/ollama/ollama/releases/download/$VERSION"

MOCK_ALL=false
FORCE_ALL=false

for arg in "$@"; do
  if [ "$arg" = "--mock" ]; then
    MOCK_ALL=true
  elif [ "$arg" = "--all" ]; then
    FORCE_ALL=true
  fi
done

if [ "${OLLAMA_FETCH_MOCK:-}" = "true" ]; then
  MOCK_ALL=true
fi

if [ "$MOCK_ALL" = "true" ]; then
  echo "MOCK mode: Creating dummy Ollama binaries for all platforms..."
  echo "mock" > "$BIN_DIR/ollama-x86_64-apple-darwin"
  echo "mock" > "$BIN_DIR/ollama-aarch64-apple-darwin"
  echo "mock" > "$BIN_DIR/ollama-x86_64-unknown-linux-gnu"
  echo "mock" > "$BIN_DIR/ollama-x86_64-pc-windows-msvc.exe"
  chmod +x "$BIN_DIR/ollama-x86_64-apple-darwin" "$BIN_DIR/ollama-aarch64-apple-darwin" "$BIN_DIR/ollama-x86_64-unknown-linux-gnu" "$BIN_DIR/ollama-x86_64-pc-windows-msvc.exe"
  echo "MOCK binaries created successfully."
  exit 0
fi

# Detect host OS
OS="$(uname -s)"
echo "Detected host OS from uname: $OS"

HOST_WINDOWS=false
HOST_MAC=false
HOST_LINUX=false

# Check if we are running inside WSL (which means the host is actually Windows)
IS_WSL=false
if [ -f /proc/sys/kernel/osrelease ] && grep -qE "(Microsoft|microsoft|wsl|WSL)" /proc/sys/kernel/osrelease 2>/dev/null; then
  IS_WSL=true
fi

if [ "$IS_WSL" = "true" ]; then
  echo "WSL environment detected. Treating host as Windows."
  HOST_WINDOWS=true
elif [[ "$OS" == *"MINGW"* ]] || [[ "$OS" == *"MSYS"* ]] || [[ "$OS" == *"CYGWIN"* ]] || [[ "$OS" == *"Windows"* ]]; then
  HOST_WINDOWS=true
elif [ "$OS" = "Darwin" ]; then
  HOST_MAC=true
elif [ "$OS" = "Linux" ]; then
  HOST_LINUX=true
else
  echo "Warning: Unknown OS $OS, defaulting to download all."
  FORCE_ALL=true
fi

# 1. MacOS Intel
if $FORCE_ALL || $HOST_MAC; then
  if [ ! -f "$BIN_DIR/ollama-x86_64-apple-darwin" ]; then
    echo "Downloading macOS Intel binary..."
    curl -L -o "$BIN_DIR/ollama-x86_64-apple-darwin" "$BASE_URL/ollama-darwin"
    chmod +x "$BIN_DIR/ollama-x86_64-apple-darwin"
  fi
else
  if [ ! -f "$BIN_DIR/ollama-x86_64-apple-darwin" ]; then
    echo "Creating mock placeholder for macOS Intel..."
    echo "mock" > "$BIN_DIR/ollama-x86_64-apple-darwin"
    chmod +x "$BIN_DIR/ollama-x86_64-apple-darwin"
  fi
fi

# 2. MacOS Apple Silicon
if $FORCE_ALL || $HOST_MAC; then
  if [ ! -f "$BIN_DIR/ollama-aarch64-apple-darwin" ]; then
    echo "Copying macOS binary for Apple Silicon (universal/fallback)..."
    cp "$BIN_DIR/ollama-x86_64-apple-darwin" "$BIN_DIR/ollama-aarch64-apple-darwin"
    chmod +x "$BIN_DIR/ollama-aarch64-apple-darwin"
  fi
else
  if [ ! -f "$BIN_DIR/ollama-aarch64-apple-darwin" ]; then
    echo "Creating mock placeholder for macOS Apple Silicon..."
    echo "mock" > "$BIN_DIR/ollama-aarch64-apple-darwin"
    chmod +x "$BIN_DIR/ollama-aarch64-apple-darwin"
  fi
fi

# 3. Linux
if $FORCE_ALL || $HOST_LINUX; then
  if [ ! -f "$BIN_DIR/ollama-x86_64-unknown-linux-gnu" ]; then
    echo "Downloading Linux binary..."
    curl -L -o "$BIN_DIR/ollama-x86_64-unknown-linux-gnu" "$BASE_URL/ollama-linux-amd64"
    chmod +x "$BIN_DIR/ollama-x86_64-unknown-linux-gnu"
  fi
else
  if [ ! -f "$BIN_DIR/ollama-x86_64-unknown-linux-gnu" ]; then
    echo "Creating mock placeholder for Linux..."
    echo "mock" > "$BIN_DIR/ollama-x86_64-unknown-linux-gnu"
    chmod +x "$BIN_DIR/ollama-x86_64-unknown-linux-gnu"
  fi
fi

# 4. Windows
if $FORCE_ALL || $HOST_WINDOWS; then
  if [ ! -f "$BIN_DIR/ollama-x86_64-pc-windows-msvc.exe" ]; then
    echo "Downloading Windows zip..."
    curl -L -o "$BIN_DIR/ollama-windows.zip" "$BASE_URL/ollama-windows-amd64.zip"
    echo "Extracting Windows zip..."
    unzip -p "$BIN_DIR/ollama-windows.zip" ollama.exe > "$BIN_DIR/ollama-x86_64-pc-windows-msvc.exe"
    rm "$BIN_DIR/ollama-windows.zip"
    chmod +x "$BIN_DIR/ollama-x86_64-pc-windows-msvc.exe"
  fi
else
  if [ ! -f "$BIN_DIR/ollama-x86_64-pc-windows-msvc.exe" ]; then
    echo "Creating mock placeholder for Windows..."
    echo "mock" > "$BIN_DIR/ollama-x86_64-pc-windows-msvc.exe"
    chmod +x "$BIN_DIR/ollama-x86_64-pc-windows-msvc.exe"
  fi
fi

echo "Ollama binaries fetch complete!"

