#!/bin/bash
# checked ok
jupyter nbconvert --to markdown notebooks/README.ipynb --output-dir ./
sed -i "s/000%20Readme%20pictures\//notebooks\/000%20Readme%20pictures\//g" README.md
exit $?
