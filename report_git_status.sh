#!/usr/bin/env bash
CHANGES=$(git status --porcelain)
echo "$CHANGES"
[ -z "$CHANGES" ]
