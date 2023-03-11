# donotcry

Encrypt/decrypt directories and files in AES cipher.

## How to use

You can test this program with the files that are in `test_files` directory.

### Encrypt

To encrypt a directory/file, run this command:

```sh
donotcry encrypt file.txt
```

### Decrypt

To decrypt a directory/file, run this command

```sh
donotcry decrypt file.txt
```

## Install

### Build from source

Requirements:

- Rust compiler
- cargo

#### Local

1. Clone the repo and cd into it.
2. Build and install in cargo path.

```sh
cargo install --path .
```

#### crates.io

```sh
cargo install do-not-cry
```

### Download binary

You can download a github release for your OS and then move it into your binaries path (/usr/bin for linux and mac).

## Auto install

Use `install.sh` to auto install donotcry for your OS.

This will download install script and run it.

```sh
curl -L https://raw.githubusercontent.com/RotrixLOL/do-not-cry/main/install.sh | sh
```

## TODO

Here are some tasks, you can check for more by searching `// TODO:` in source code.

- generate randomly key and iv on encrypt, then show it or send an email. when decrypting ask for it.

## Contribute

Contributions are open. Just follow [CONTRIBUTING.md](CONTRIBUTING.md).
