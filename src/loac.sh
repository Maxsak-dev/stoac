#!/bin/sh

# The suggested command text
suggestion="ls -l /some/directory"

# Use `readline` to "type" the suggestion into the shell
bind "\"\e[1~\": \"$suggestion\""

echo "Press Home key to see the autosuggestion: $suggestion"
