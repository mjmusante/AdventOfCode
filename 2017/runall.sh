#! /bin/bash

top=$PWD

pylist=$(grep -l __name__ */*.py)

for loc in $pylist
do
    dir=$(dirname $loc)
    pyprg=$(basename $loc)
    cd $top/$dir
    echo "================= ==== ================="
    echo "=                 $dir                 ="
    echo "================= ==== ================="
    time python $pyprg
    echo "\n\n"
done
