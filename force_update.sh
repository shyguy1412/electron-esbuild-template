#! /bin/env bash
npm outdated -p | sed -E 's/[^:]*:(@?[^@]*).*/\1/' | xargs -I{} npm install {}@latest
