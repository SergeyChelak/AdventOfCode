#!/bin/bash

AOC_SESSION=$(cat etc/session)

if [ "$#" -ne 2 ]; then
    echo "Year and day are required argments"
    exit 1
fi

YEAR=$1
DAY=$2

echo "Fetching $YEAR day $DAY..."
INPUT_FILE=input/aoc"$YEAR"_"$DAY"
curl https://adventofcode.com/$YEAR/day/$DAY/input --cookie "session=$AOC_SESSION" -o $INPUT_FILE

wc -l $INPUT_FILE