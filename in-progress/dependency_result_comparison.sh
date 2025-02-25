#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 sparse_checkout_command1.sh sparse_checkout_command2.sh"
    exit 1
fi

if [ ! -f "$1" ] || [ ! -f "$2" ]; then
    echo "Error: Both arguments must be existing files."
    exit 1
fi

echo "Lines present in $2 but missing in $1:"
diff <(sort "$1" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//') \
     <(sort "$2" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//') | \
     grep '^>' | cut -c 3-

echo -e "\nLines present in $1 but missing in $2:"
diff <(sort "$2" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//') \
     <(sort "$1" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//') | \
     grep '^>' | cut -c 3-