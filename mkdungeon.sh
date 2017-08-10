#!/usr/bin/env bash

mkdir -p dungeons || exit 1
cd dungeons || exit 2

cargo build --release

while IFS= read -r seed; do
	../target/release/mount_arelle "$seed" || exit 3
done < <(tr -dc a-zA-Z0-9 < /dev/urandom | fold -w 32 | head -n 100)
