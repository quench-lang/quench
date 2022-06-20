#!/usr/bin/env bash
node packages/cli/index.js "$1" | node --input-type=module - "${@:2}"
