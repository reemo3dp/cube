#!/bin/sh
set -euxo pipefail

seq 1 1000 | parallel -N0 -j 100% --halt now,done=1  --line-buffer "$@"