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

cargo publish --token $CRATES_IO_TOKEN


# --- Update release tags using Chandler ---------------------------------------

log "  Installing" "chandler gem"
gem install --user-install chandler

export GEM_PATH="$(ruby -r rubygems -e 'puts Gem.user_dir')"
export PATH="${GEM_PATH}/bin:$PATH"

log "    Updating" "GitHub release notes"
chandler push --github="$TRAVIS_REPO_SLUG"
