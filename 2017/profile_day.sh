#! /bin/bash

if [ "$1" = "" ]; then
    echo "Specify day" 1>&2
    exit 1
fi

if [ ! -d "ms$1" ]; then
    echo "Directory for day $1 (ms$1) not found" 1>&2
    exit 1
fi

cd "ms$1"
py_script=$(grep -l __name__ *.py)
if [ "$py_script" == "" ]; then
    echo "Could not find runnable script in ms$1" 1>&2
    exit 1
fi

python -m cProfile -o "/tmp/ms$1.cprof" $py_script
pyprof2calltree -k -i "/tmp/ms$1.cprof"
