#!/usr/bin/env bash

mkdir -p dungeons || exit 1
cd dungeons || exit 2

while IFS= read -r seed; do
	cargo run --release "$seed"
done < <(tr -dc a-zA-Z0-9 < /dev/urandom | fold -w 32 | head -n 50)
