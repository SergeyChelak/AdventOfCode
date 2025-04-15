#!/bin/sh

AOC_SESSION=$(cat etc/session)
if [ "${#AOC_SESSION}" -eq 0 ]; then
    echo "Missing session id"
    exit 1
fi

if [ "$#" -ne 2 ]; then
    echo "Year and day are required arguments"
    exit 1
fi

YEAR=$1
DAY=$2

FORMATTED_DAY=$DAY
if [ "${#FORMATTED_DAY}" -lt 2 ]; then
    FORMATTED_DAY=0"$FORMATTED_DAY"
fi

mkdir -p input

echo "Fetching $YEAR day $DAY..."
INPUT_FILE=input/aoc"$YEAR"_"$FORMATTED_DAY"
curl https://adventofcode.com/$YEAR/day/$DAY/input --cookie "session=$AOC_SESSION" -o $INPUT_FILE

head $INPUT_FILE
wc -l $INPUT_FILE
