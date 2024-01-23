#!/bin/bash

# Check if xclip is installed
if ! command -v xclip &> /dev/null; then
    notify-send "Error: xclip is not installed. Please install it first."
    exit 1
fi

flameshot_cmd="flameshot gui"
$flameshot_cmd

temp_file=$(mktemp /tmp/flameshot_image_XXXXXX.png)
xclip -selection clipboard -t image/png -o > "$temp_file"

if [ ! -s "$temp_file" ]; then
    notify-send "Error: Temporary file does not contain an image. Exiting."
    rm "$temp_file"  
    exit 1
fi

upload_url="PLACEHOLDER_DOMAIN"
api_key="PLACEHOLDER_API_KEY"

response=$(curl -s -H "API-KEY: $api_key" -T "$temp_file" "$upload_url/new/")
image_url=$(echo "$response" | jq -r '.file')

echo -n "$image_url" | xclip -selection clipboard

notify-send "Screenshot Uploaded" "Server Response copied to clipboard."

rm "$temp_file"

