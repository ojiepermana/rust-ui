#!/usr/bin/env python3
"""Check a deliberately supplied release version without writing or publishing."""
import json
import re
import subprocess
import sys

if len(sys.argv) != 2 or not re.fullmatch(r"v(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)", sys.argv[1]):
    sys.exit("Expected vMAJOR.MINOR.PATCH, with no leading zeroes")
metadata = json.loads(subprocess.check_output(
    ["cargo", "metadata", "--no-deps", "--locked", "--format-version", "1"], text=True))
versions = {p["version"] for p in metadata["packages"]}
if versions != {sys.argv[1][1:]}:
    sys.exit(f"Requested version does not match workspace versions: {versions}")
print(f"Validated {sys.argv[1]}; no publication or release was created.")
