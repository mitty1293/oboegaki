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
2. Download the latest release binary.
3. Make the binary executable and move it to your PATH.

    ```sh
    chmod +x obo
    sudo mv obo /usr/local/bin/
    ```

### Use the install script

Alternatively, you can use the install script:

```sh
curl -L https://github.com/mitty1293/oboegaki/releases/download/v0.1.0/install.sh | sh
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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
