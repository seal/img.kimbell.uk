#!/bin/bash
## added into i3 via:
# bindsym $mod+a exec --no-startup-id ~/rust/image-tool/upload.sh


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

upload_url=https://img.kimbell.uk/new/

response=$(curl -s -T "$temp_file" "$upload_url")

image_url=$(echo "$response" | jq -r '.file')


echo -n "$image_url" | xclip -selection clipboard

notify-send "Screenshot Uploaded" "Server Response copied to clipboard."

rm "$temp_file"

