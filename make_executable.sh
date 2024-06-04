#!/bin/bash

# This script will find all .sh files and mark them as executable in the git index

find . -type f -name "*.sh" -exec git update-index --chmod=+x {} \;

echo "All .sh files have been marked as executable in the git index."
