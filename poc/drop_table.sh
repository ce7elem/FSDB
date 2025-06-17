#!/bin/bash
set -xe
FSDB=${FSDB:-"/tmp/fsdb"}

if [ -z $1 ]; then
  echo "Usage: $0 <TABLE NAME>"
  exit -1;
fi

if ! [ -d "$FSDB/$1" ]; then
  echo "Table '$1' doesnt exist in bucket $FSDB";
  exit -1;
fi

echo "[log] rm -rf $FSDB/$1"
rm -r "$FSDB/$1"
