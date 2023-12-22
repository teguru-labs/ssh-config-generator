# SSH Config Generator

## Install

Please download the binary file from [the release page](https://github.com/devsuccess101/ssh-config-generator/releases/latest).

For example:

```bash
wget https://github.com/devsuccess101/ssh-config-generator/releases/download/v0.1.0/scg \
    && sudo chmod +x scg \
    && sudo mv scg /usr/local/bin
```

```bash
scg --version
```

## Usage

```bash
SSH Config Generator

Usage: scg [OPTIONS] --user <USER> --host <HOST>

Options:
  -u, --user <USER>                    SSH user
  -h, --host <HOST>                    SSH hostname
  -i, --identity-file <IDENTITY_FILE>  Indentity file
  -o, --output <OUTPUT>                Output file
      --help                           Print help
  -V, --version                        Print version
```

### Generate SSH Config

```bash
scg -u user -i ~/.ssh/id_rsa -h 127.0.0.1 -h 127.0.0.2
```

Output:

```bash
# generated via `scg` - SSH Config Generator:
Host node1
  Hostname 127.0.0.1
  User user
  Identity file /home/kimyvgy/.ssh/id_rsa
Host node2
  Hostname 127.0.0.2
  User user
  Identity file /home/kimyvgy/.ssh/id_rsa
```
