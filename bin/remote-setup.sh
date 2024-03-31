#!/bin/bash

BINARY_NAME="gowithdev-rsapi"
SERVICE_FILE_NAME="gowithdev-rsapi.service"
# Define remote paths
REMOTE_BINARY_PATH="/usr/local/bin/$BINARY_NAME"
REMOTE_SERVICE_PATH="/etc/systemd/system/$SERVICE_FILE_NAME"

# Move the binary to /usr/local/bin
sudo mv /tmp/$BINARY_NAME $REMOTE_BINARY_PATH
sudo chmod +x $REMOTE_BINARY_PATH

# Move the service file to /etc/systemd/system
sudo mv /tmp/$SERVICE_FILE_NAME $REMOTE_SERVICE_PATH

# Reload systemd to recognize the new service
sudo systemctl daemon-reload

# Enable and start the service
sudo systemctl enable $SERVICE_FILE_NAME
sudo systemctl start $SERVICE_FILE_NAME

echo "Binary and systemd service deployed and started successfully on the remote server."
