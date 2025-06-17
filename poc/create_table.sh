#!/bin/bash
set -xe
DBFS=${DBFS:-"/tmp/dbfs"}

if [ -z $1 ]; then
  echo "Usage: $0 <TABLE NAME> ( <COL_NAME_1> <COL_NAME_2> ... )"
  echo "Example:"
  echo "\t $0 users --col username --col email --col password )"
  exit -1;
fi

echo "[log] mkdir $DBFS/$1"
mkdir "$DBFS/$1"
