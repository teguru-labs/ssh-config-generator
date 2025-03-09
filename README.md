# SSH Config Generator

## Installation

### Binary file

Please download the binary file from [the release page](https://github.com/teguru-labs/ssh-config-generator/releases/latest).

**MacOS:**

```bash
wget https://github.com/teguru-labs/ssh-config-generator/releases/download/v0.1.0/ssh-config_aarch64-apple-darwin -O ssh-config \
    && sudo chmod +x ssh-config \
    && sudo mv ssh-config /usr/local/bin
```

**Linux:**

```bash
wget https://github.com/teguru-labs/ssh-config-generator/releases/download/v0.1.0/ssh-config_x86_64-unknown-linux-gnu -O ssh-config \
    && sudo chmod +x ssh-config \
    && sudo mv ssh-config /usr/local/bin
```

**Verification:**

```bash
ssh-config --version
```

### Build from source

```bash
git clone git@github.com:teguru-labs/ssh-config-generator.git
cargo build --release --bin ssh-config
```

The output file is `./target/release/ssh-config`.

## Usage

```bash
SSH Config Generator

Usage: ssh-config [OPTIONS] --user <USER> --host <HOST>

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
ssh-config -u user -i ~/.ssh/id_rsa -h 127.0.0.1 -h 127.0.0.2
```

Output:

```bash
# generated via `ssh-config` - SSH Config Generator:
Host node1
  Hostname 127.0.0.1
  User user
  Identity file /home/kimyvgy/.ssh/id_rsa
Host node2
  Hostname 127.0.0.2
  User user
  Identity file /home/kimyvgy/.ssh/id_rsa
```
