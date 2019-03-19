#!/bin/sh

# --- Setup sscache ------------------------------------------------------------------
LATEST=$(cargo search sccache | grep sccache | cut -f2 -d"\"")
LOCAL=$((sccache --version 2>/dev/null || echo "none") | cut -f2 -d" ")

if [ "$LATEST" != "$LOCAL" ]; then
	echo "Installing latest sccache v$LATEST (local: v$LOCAL)"
	RUSTC_WRAPPER="" cargo install --path "$HOME/.local/bin" sccache
else
	echo "Using cached sccache v$LOCAL"
fi


# --- Setup cargo-tarpaulin ----------------------------------------------------------
LATEST=$(cargo search cargo-tarpaulin | grep cargo-tarpaulin | cut -f2 -d"\"")
LOCAL=$((cargo tarpaulin --version 2>/dev/null || echo "none") | cut -d" " -f3)

if [ "$LATEST" != "$LOCAL" ]; then
	echo "Installing latest cargo-tarpaulin v$LATEST (local: v$LOCAL)"
	cargo install --path "$HOME/.local/bin" cargo-tarpaulin
else
	echo "Using cached cargo-tarpaulin v$LOCAL"
fi
