#!/bin/bash

# Read the domain from .env file
if [ -f .env ]; then
    source .env
else
    echo "Error: .env file not found."
    exit 1
fi

# Set the domain & api-key in upload.sh
sed -i "s|PLACEHOLDER_DOMAIN|$DOMAIN|" ./upload.sh
sed -i "s|PLACEHOLDER_API_KEY|$API_KEY|" ./upload.sh

echo "Domain & api-key set in upload.sh: $DOMAIN , $API_KEY"

