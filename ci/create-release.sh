#!/bin/bash
set -x
name="$1"

notes=$(cat <<EOT
See https://github.com/YatogamiRaito/HekaTerm/releases/tag/$name for the changelog

If you're looking for nightly downloads or more detailed installation instructions:

[Windows](https://github.com/YatogamiRaito/HekaTerm/releases)
[macOS](https://github.com/YatogamiRaito/HekaTerm/releases)
[Linux](https://github.com/YatogamiRaito/HekaTerm/releases)
[FreeBSD](https://github.com/YatogamiRaito/HekaTerm/releases)
EOT
)

gh release view "$name" || gh release create --prerelease --notes "$notes" --title "$name" "$name"
