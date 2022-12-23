#!/usr/bin/env bash

if [[ -z $1 ]]; then
    echo "Usage: $0 day"
    echo "  e.g. $0 09"
    exit 1
fi

day_small="$1"
day=$(printf %02d "$1")
echo $day
cp -r template "day$day"
sed -i "s/DAY_SMALL/$day_small/g" "day$day/main.go"
