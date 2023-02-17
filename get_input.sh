#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

SESSION_VALUE="53616c7465645f5f51d6056235df36a32464c9b8b254b68ee2c431c85c1cd38d211fcafcc060c986c2da5c8fbd8c7acf81f5e9c0757b5971166dec4a0be1d353"

curl -b "session=$SESSION_VALUE" "https://adventofcode.com/2022/day/"$1"/input" > ~/Misc/advent_of_code_2022/day"$1"/input.txt