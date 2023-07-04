Smallet Command Line Interface

First, make sure you have the Solana CLI tools installed. [Follow the instructions here.](https://docs.solana.com/cli/install-solana-cli-tools)

Next, install Cosmic via Cargo like so:

```
cargo install cosmic
```

## Usage

### Setup

Go to any directory and run the following command:

```
cosmic init
```

This will create a `.cosmic` directory, which you should add to your `.gitignore`.

The `.cosmic` directory contains keypairs that will contain the SOL you use for program deployment. You may want to back up this folder via an encrypted filestore such as [Keybase](https://keybase.io/). You should not be storing any sensitive funds in this wallet-- **only use this for program deploys.**

### Upgrading a Program

To upgrade any existing program on Solana, run `cosmic upload-program-buffer`.

```
Uploads a Solana program buffer.

USAGE:
    cosmic upload-program-buffer [OPTIONS] --location <LOCATION> --program-id <PROGRAM_ID>

OPTIONS:
    -c, --cluster <CLUSTER>          Cluster to deploy to. Defaults to devnet. [default: devnet]
    -h, --help                       Print help information
    -l, --location <LOCATION>        The path to the Solana program buffer.
    -p, --program-id <PROGRAM_ID>    The program being upgraded.
```

For example, let's say you wanted to upgrade the [Smallet](https://crates.io/crates/smallet) on mainnet. You would run the following command:

```
cosmic upload-program-buffer --cluster mainnet --location gh:smallet:CosmicWire/smallet@0.11.1 --program-id 7vZw152zk65W5F59S6Svt29JjDhzZocWvF7PYvjNCAKB
```

If the command is successful, you should now have a buffer of the Smallet program at release v0.11.1 deployed somewhere on mainnet, owned by the current upgrade authority of the Smallet program. The upgrade authority would then be able to upgrade their program's bytecode to the contents of that uploaded buffer.

If you don't have enough SOL in your wallet, the command will fail and tell you what key you should be sending SOL to.

#### Location Location Location 

There are three formats of `location` that you may specify:

- a `.so` artifact of a GitHub release, for example `gh:smallet:CosmicWire/smallet@0.11.1`
- a URL,
- a file path, for example `./target/deploy/smallet.so`.
