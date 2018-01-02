#! /bin/bash

f="day${1}_test.py"

if [ ! -f "$f" ]; then
    echo File not found: $f
    exit 1
fi

classname=$(grep '^class' $f | head -1 | cut -d\  -f2 | cut -d\( -f1)

python -m unittest "${f%.py}.${classname}.${2}"
