#!/bin/bash

# Read the domain from .env file
if [ -f .env ]; then
    source .env
else
    echo "Error: .env file not found."
    exit 1
fi

# Set the domain in upload.sh
sed -i "s|PLACEHOLDER_DOMAIN|$DOMAIN|" ~/rust/image-tool/upload.sh

echo "Domain set in upload.sh: $DOMAIN"

