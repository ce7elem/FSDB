#!/bin/bash
set -xe
DBFS=${OLIFS:-"/tmp/dbfs"}

if [ -z $1 ]; then
  echo "Usage: $0 <TABLE NAME>"
  exit -1;
fi

if ! [ -d "$DBFS/$1" ]; then
  echo "Table '$1' doesnt exist in DBFS $DBFS";
  exit -1;
fi

echo "[log] rm -rf $DBFS/$1"
rm -r "$DBFS/$1"
