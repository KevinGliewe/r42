#!/usr/bin/env python3
"""Utility script to bump the package version in `Cargo.toml`."""

import fileinput
import os
import sys

# Determine project paths relative to this script location
script_dir = os.path.dirname(os.path.realpath(__file__))
repo_dir = os.path.abspath(os.path.join(script_dir, '..'))
# Location of the Cargo manifest to update
cargo_file = os.path.join(repo_dir, 'Cargo.toml')

# Default version if none is supplied
# Default version if none is supplied
version = '0.0.0'

if len(sys.argv) > 1:
    # Use the version supplied on the command line
    version = sys.argv[1]

# Text to search for and its replacement
# Pattern we search for and its replacement string
find = "version = \"0.0.0\""
replace = f"version = \"{version}\""

# Inform the user about the change
print(f"Updating version in {cargo_file}")
print(f"  from {find} to {replace}")



# Perform the in-place update of Cargo.toml
with fileinput.FileInput(cargo_file, inplace=True, backup='.bak') as file:
    for line in file:
        # Replace the old version line with the new one
        print(line.replace(find, replace), end='')

