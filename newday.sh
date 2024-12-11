#!/bin/bash

# Check if the correct number of arguments is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <two_digit_number>"
    exit 1
fi

# Get the parameters
X="$1"

# Validate that X is a two-digit number
if ! [[ "$X" =~ ^[0-9]{2}$ ]]; then
    echo "Error: The parameter must be a two-digit number."
    exit 1
fi

cp src/template.rs src/day$X.rs
git add src/day$X.rs
touch input/day$X.txt

# Define the placeholder you want to replace
PLACEHOLDER="{{day}}"

# Use sed to replace the placeholder with the desired text
sed -i "s|$PLACEHOLDER|$X|g" "src/day$X.rs"
sed -i "s/.*\/\/ PLACEHOLDER.*/pub mod day$X;\n\/\/ PLACEHOLDER/" "src/lib.rs"
sed -i "s/.*\/\/ PLACEHOLDER.*/($X, day$X::day_main),\n\/\/ PLACEHOLDER/" "src/main.rs"

cargo fmt

echo "seems allright. created template for day $X"
