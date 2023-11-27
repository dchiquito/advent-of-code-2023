#!/bin/sh

templatize_day () {
  local DAY=$1
  echo "Templatizing day ${DAY}"
  cat template.rs | envsubst > src/day${DAY}.rs
}

if [ -z "$1" ]; then
  for i in $(seq 1 25); do
    templatize_day $i
  done
else
  templatize_day $1
fi
