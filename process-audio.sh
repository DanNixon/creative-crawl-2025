#!/usr/bin/env bash

set -exo pipefail

for dir in stories intro; do
    outdir="audio/$dir"
    mkdir -p "$outdir"

    for infile in "$dir"/*.mp3; do
        outfile="$outdir/$(basename "${infile%.mp3}.sln")"
        ffmpeg -i "$infile" -f wav - | sox -t wav - -t raw -r 8000 -c 1 "$outfile"
    done
done
