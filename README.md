# envelop

**Like `age`, but the keys come from GitHub.**

Send encrypted files to any GitHub user. No key exchange, no setup, no accounts to create.
If they have SSH keys on GitHub, you can send them a secret.

<!-- TODO: Add terminal demo recording (asciinema/vhs) -->

## How it works

1. You run `envelop send secret.txt --to alice`
2. Envelop fetches Alice's SSH public keys from GitHub
3. Your file is encrypted using [age](https://github.com/FiloSottile/age) with those keys
4. The ciphertext is uploaded as a private GitHub Gist
5. A comment is posted on the Gist to notify Alice

Alice runs `envelop open <gist-url>` and the file is decrypted with her local SSH private key.

That's it. No PGP, no key servers, no pre-shared secrets.

## Install

### From source

```bash
cargo install --path envelop-cli
```

### From crates.io

```bash
# coming soon
cargo install envelop-cli
```

## Quick start

### Send a file

```bash
envelop send secret.txt --to alice
```

Send multiple files (they get bundled into a tar archive):

```bash
envelop send report.pdf notes.txt --to alice
```

Add a custom description and comment:

```bash
envelop send secret.txt --to alice \
  --description "Q4 financials" \
  --comment "Hey Alice, here are the numbers you asked for"
```

### Open a received envelop

```bash
envelop open https://gist.github.com/bob/abc123def456
```

Specify a different SSH key or output directory:

```bash
envelop open https://gist.github.com/bob/abc123def456 \
  --identity ~/.ssh/id_rsa \
  --output ~/downloads
```

### Look up someone's keys

```bash
envelop keys alice
```

## Authentication

To create Gists on your behalf, envelop needs a GitHub token. It checks these sources in order:

1. The `--token` flag
2. The `GITHUB_TOKEN` environment variable
3. The output of `gh auth token` (if you have the [GitHub CLI](https://cli.github.com/) installed)

Your token needs the `gist` scope. You can create one at
[github.com/settings/tokens](https://github.com/settings/tokens).

**Recipients do not need a token.** The `open` command only reads public Gist data and uses
a local SSH key for decryption.

## How encryption works

Envelop uses the [age](https://age-encryption.org/) encryption format under the hood, specifically
its SSH key support. When you send a file:

- All of the recipient's SSH public keys are fetched from `github.com/<user>.keys`
- The file is encrypted to every key, so the recipient can decrypt with any of their keys
- The ciphertext is ASCII-armored for safe transport as text in a Gist

When opening an envelop:

- Your local SSH private key (defaults to `~/.ssh/id_ed25519`) is used
- Both Ed25519 and RSA keys are supported

The sender never sees or handles private keys. GitHub acts as a public key directory.

## Supported key types

| Key type | Send (encrypt) | Open (decrypt) |
|----------|----------------|-----------------|
| Ed25519  | Yes            | Yes             |
| RSA      | Yes            | Yes             |

## Project structure

```
envelop/
  envelop-core/   # Library: encryption, archiving, GitHub API
  envelop-cli/    # Binary: the `envelop` command
```

## Contributing

Contributions are welcome! Some areas that could use help:

- Platform-specific packaging (Homebrew, AUR, Nix)
- Better error messages and progress output
- Support for alternative transports beyond GitHub Gists

## License

[MIT](LICENSE)
