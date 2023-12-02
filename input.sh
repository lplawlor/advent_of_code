#!/usr/bin/zsh

fetch_input() {
    # If either params is not an integer
    if ! [[ $1 =~ '^[0-9]+$' ]] || ! [[ $2 =~ '^[0-9]+$' ]] then
        echo "year and day parameters must be integers"
        exit
    fi

    # Interpret params as base10 (removing leading 0s)
    YEAR=$((10#$1))
    DAY=$((10#$2))

    # If the day param it is not in the range [1, 25]
    if (( $YEAR < 2015 )); then
        echo "year must be at least 2015."
        exit
    fi

    # If the day param it is not in the range [1, 25]
    if (( $DAY < 1 || $DAY > 25 )); then
        echo "day must be an from 1 to 25."
        exit
    fi

    FOLDER=${0:a:h}/$YEAR/$DAY

    echo "Making GET request to https://adventofcode.com/$YEAR/day/$DAY/input"

    # Fetch that year-day's input and write it, creating the folders if necessary
    curl --cookie session=$SESSION https://adventofcode.com/$YEAR/day/$DAY/input -o $FOLDER/input --create-dirs

    echo "Input saved to $FOLDER/input."
}

# .env should be in the same directory as this script
ENV_FILE=${0:a:h}/.env

if [[ ! -f $ENV_FILE ]]; then
    echo "Error: .env file not found at $ENV_FILE"
    exit
fi

# Load .env
source $ENV_FILE

if [[ -z $SESSION ]]; then
    echo "Error SESSION variable not found in $ENV_FILE"
    exit
fi

# If the number of arguments is not exactly 2
if [ $# -ne 2 ]
then
    echo "Usage: $0 <year> <day>"
    exit
fi

# Run the function
fetch_input $1 $2
