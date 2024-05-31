# oboegaki

oboegaki is a command-line tool for registering and managing frequently used commands along with their categories and notes.

## Features

- Add new commands with category and note
- List all registered commands
- Run a command by its index
- Delete a command by its index

## Installation

### Download and install the latest release

1. Go to the [Releases](https://github.com/mitty1293/oboegaki/releases) page.
2. Download the latest release ZIP file.
3. Unzip the file and move the binary to your PATH.

    ```sh
    unzip obo_<version>.zip
    sudo mv obo /usr/local/bin/
    sudo chmod +x /usr/local/bin/obo
    sudo mkdir -p /usr/local/share/licenses/oboegaki
    sudo mv LICENSE /usr/local/share/licenses/oboegaki/
    ```

### Use the install script

Alternatively, you can use the install script:

```sh
curl -L https://github.com/mitty1293/oboegaki/raw/main/install.sh | sh
```

## Usage

### Add a new command

```sh
obo add --command "ls -l" --category "file" --note "List files in long format"
```

### List all registered commands

```sh
obo list
```

### Run a command by its index

```sh
obo run --index 1
```

### Delete a command by its index

```sh
obo delete --index 1
```

## Configuration File Location
The commands.json file, which stores your registered commands, is located in the following directory:
```
Path: ~/.config/oboegaki/commands.json
```
This file contains all the commands you have added, along with their categories and notes. You can manually back up or edit this file if necessary.

## Uninstallation

To uninstall oboegaki and remove the binary:

1. Delete the binary file

   ```sh
   sudo rm /usr/local/bin/obo
   ```
2. Optionally, remove the configuration directory and the commands.json file
   ```sh
   rm -rf ~/.config/oboegaki
   ```
## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
