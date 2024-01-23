#!/bin/bash

# Load environment variables from .env file
if [ -f .env ]; then
  source .env
else
  echo "Error: .env file not found."
  exit 1
fi

# Check if xclip is installed
if ! command -v xclip &> /dev/null; then
    echo "Error: xclip is not installed. Please install it first."
    exit 1
fi

flameshot_cmd="flameshot gui"
$flameshot_cmd

temp_file=$(mktemp /tmp/flameshot_image_XXXXXX.png)
xclip -selection clipboard -t image/png -o > "$temp_file"

if [ ! -s "$temp_file" ]; then
    echo "Error: Temporary file does not contain an image. Exiting."
    rm "$temp_file"  
    exit 1
fi

upload_url="$DOMAIN"  
api_key="$API_KEY"  

response=$(curl -s -H "API-KEY: $api_key" -T "$temp_file" "$upload_url/new/")
echo $reponse
image_url=$(echo "$response" | jq -r '.file')

echo -n "$image_url" | xclip -selection clipboard

notify-send "Screenshot Uploaded" "Server Response copied to clipboard."

rm "$temp_file"

