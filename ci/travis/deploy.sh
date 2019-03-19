#!/bin/sh

log() {
		tput bold
		tput setaf 2
		echo -n "$1" ""
		tput sgr0
		shift 1
		echo $@
}


# --- Publish crate to `crates.io` ---------------------------------------------

cargo publish


# --- Update release tags using Chandler ---------------------------------------

log "  Installing" "chandler gem"
gem install --user-install chandler

log "    Updating" "GitHub release notes"
chandler push --github="$TRAVIS_REPO_SLUG"
