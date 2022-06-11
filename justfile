#!/usr/bin/env just --justfile

# By default, recipes are only listed.
default:
	@just --list

run:
	@cargo run

build:
	@cargo build

uninstall:
	@cargo uninstall sanctity

user-install:
	@cargo install --path .

system-install:
	@cargo install --path . --root /usr --no-track

# Install to a custom location
custom-install:
	@cargo install --path . --root "$SANCTITY_INSTALL_PATH" --no-track
