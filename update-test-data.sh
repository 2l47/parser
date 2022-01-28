#!/usr/bin/env bash

for demo in test_data/*.dem; do
	export NAME=`echo "$demo" | grep -Po '(?<=/)[\w\d]+'`
	cargo run --bin parse_demo $demo >test_data/$NAME.json
	# Remove the trailing newline
	truncate -s -1 test_data/$NAME.json
done

# Not a test case at the moment?
rm test_data/unicode.json
