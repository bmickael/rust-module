#!/bin/bash
# Cells cuted between pages
# invalid unicode characters
sudo apt install pandoc
sudo apt install texlive-xetex
find notebooks -type d -name "pictures" -exec cp -rv {} ./ \;
jupyter nbconvert --to pdf merged_notebook.ipynb
# rm -r pictures
exit $?
