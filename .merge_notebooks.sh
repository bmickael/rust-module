#!/bin/bash
# checked ok
sudo pip install nbmerge
nbmerge notebooks/README.ipynb notebooks/0*/*.ipynb > merged_notebook.ipynb
exit $?
