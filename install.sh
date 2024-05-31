#!/bin/sh

REPO="mitty1293/oboegaki"
BIN_NAME="obo"
LICENSE_NAME="LICENSE"

# Get the latest release version
VERSION=$(curl --silent "https://api.github.com/repos/$REPO/releases/latest" | jq -r .tag_name)
ZIP_NAME="obo_${VERSION}.zip"

# Create download URL
URL="https://github.com/$REPO/releases/latest/download/$VERSION/$ZIP_NAME"

# Download the ZIP file
curl -L $URL -o /tmp/$ZIP_NAME

# Unzip the file
unzip /tmp/$ZIP_NAME -d /tmp/

# Move the binary to /usr/local/bin and make it executable
sudo mv /tmp/$BIN_NAME /usr/local/bin/$BIN_NAME
sudo chmod +x /usr/local/bin/$BIN_NAME

# Move the LICENSE file to /usr/local/share/licenses/oboegaki
sudo mkdir -p /usr/local/share/licenses/oboegaki
sudo mv /tmp/$LICENSE_NAME /usr/local/share/licenses/oboegaki/$LICENSE_NAME

# Clean up
rm /tmp/$ZIP_NAME

echo "$BIN_NAME has been installed to /usr/local/bin"
echo "LICENSE has been moved to /usr/local/share/licenses/oboegaki"
