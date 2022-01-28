#!/usr/bin/env bash

for demo in test_data/*.json; do
	cp $demo $demo.readable
	cat $demo.readable | jq >$demo
	rm $demo.readable
done
