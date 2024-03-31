#!/bin/bash

# Check if required environment variables are set
if [ -z "$PRIVATE_KEY_CONTENT" ] || [ -z "$BINARY_PATH" ] || [ -z "$SERVICE_FILE_PATH" ] || [ -z "$DESTINATION_USER" ] || [ -z "$DESTINATION_HOST" ]; then
  echo "Error: One or more required environment variables are not set."
  exit 1
fi

mkdir -p ~/.ssh/ && touch ~/.ssh/known_hosts
ssh-keyscan "$DESTINATION_HOST" >> ~/.ssh/known_hosts
chmod 600 ~/.ssh/known_hosts
eval $(ssh-agent)
ssh-add - <<< "$PRIVATE_KEY_CONTENT"

echo $BINARY_PATH | sed 's/./& /g'

# Copy the binary to the remote server
scp "$BINARY_PATH" "$DESTINATION_USER@$DESTINATION_HOST:/tmp/$(basename $BINARY_PATH)"

# Check if scp was successful
if [ $? -ne 0 ]; then
  echo "Error: Failed to copy the binary to the remote server."
  exit 1
fi

# Copy the service file to the remote server
scp "$SERVICE_FILE_PATH" "$DESTINATION_USER@$DESTINATION_HOST:/tmp/$(basename $SERVICE_FILE_PATH)"

# Check if scp was successful
if [ $? -ne 0 ]; then
  echo "Error: Failed to copy the service file to the remote server."
  exit 1
fi

# Execute the remote script to move files and start the service
ssh "$DESTINATION_USER@$DESTINATION_HOST" "bash -s" < bin/remote-setup.sh

echo "Files copied successfully to the remote server. Executing remote setup."
