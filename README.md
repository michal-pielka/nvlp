# e*nv*e*l*o*p*e

**Like `age`, but the keys come from GitHub.**

Encrypt files to any GitHub user. No key exchange, no setup, no accounts to create.
If they have SSH keys on GitHub, you can encrypt a file for them.

![demo](https://github.com/user-attachments/assets/c0b79f5c-0125-42b5-a7ee-761dc4e04d22)

## How it works

1. You run `nvlp encrypt secret.env --to alice -o secret.env.age`
2. nvlp fetches Alice's SSH public keys from GitHub
3. Your file is encrypted using [age](https://github.com/FiloSottile/age) with those keys
4. You get `secret.env.age` and send it however you want (Slack, email, etc.)

Alice runs `nvlp decrypt secret.env.age -o secret.env` and the file is decrypted with her local SSH private key.

That's it. No PGP, no key servers, no pre-shared secrets.

## Install

### From source

```bash
cargo install --path nvlp
```

### From crates.io

```bash
cargo install nvlp
```

## Quick start

### Encrypt a file

```bash
nvlp encrypt secret.env --to alice -o secret.env.age
```

Encrypt for multiple recipients:

```bash
nvlp encrypt secret.env --to alice --to bob -o secret.env.age
```

Encrypt from stdin:

```bash
echo "the password is hunter2" | nvlp encrypt --to alice -o message.age
```

Output defaults to stdout, so you can pipe or redirect:

```bash
nvlp encrypt secret.env --to alice > secret.env.age
```

To encrypt multiple files, bundle them first:

```bash
tar czf bundle.tar.gz file1.txt file2.txt
nvlp encrypt bundle.tar.gz --to alice -o bundle.tar.gz.age
```

### Decrypt a file

```bash
nvlp decrypt secret.env.age -o secret.env
```

Decrypt from stdin:

```bash
cat secret.env.age | nvlp decrypt -o secret.env
```

Output defaults to stdout, so you can pipe to other tools:

```bash
nvlp decrypt secret.json.age | jq '.api_key'
```

Specify a different SSH key:

```bash
nvlp decrypt secret.env.age --identity ~/.ssh/id_rsa -o secret.env
```

### Send via GitHub Gist

If you want nvlp to handle delivery too, use `send`. It encrypts the file and uploads it
as a private Gist.

```bash
nvlp send secret.env --to alice
```

Send to multiple recipients:

```bash
nvlp send secret.env --to alice --to bob
```

Add a custom description and comment:

```bash
nvlp send secret.env --to alice \
  --description "Q4 financials" \
  --comment "Hey Alice, here are the numbers you asked for"
```

### Open a Gist

The `open` command fetches the gist, decrypts it, and restores the original filename:

```bash
nvlp open https://gist.github.com/bob/abc123def456
# -> secret.env (original filename preserved)
```

Print to stdout instead of saving:

```bash
nvlp open https://gist.github.com/bob/abc123def456 --stdout
```

Specify a different SSH key or output file:

```bash
nvlp open https://gist.github.com/bob/abc123def456 \
  --identity ~/.ssh/id_rsa \
  --output secret.env
```

### Look up someone's keys

```bash
nvlp keys alice
```

## Authentication

The `encrypt`, `decrypt`, and `keys` commands need no authentication at all.

The `send` and `open` commands interact with the GitHub API. nvlp needs a GitHub token,
which it checks for in this order:

1. The `--token` flag
2. The `GITHUB_TOKEN` environment variable
3. The output of `gh auth token` (if you have the [GitHub CLI](https://cli.github.com/) installed)

Your token needs the `gist` scope. You can create one at
[github.com/settings/tokens](https://github.com/settings/tokens).

## How encryption works

nvlp uses the [age](https://age-encryption.org/) encryption format under the hood, specifically
its SSH key support. When you encrypt a file:

- All of the recipient's SSH public keys are fetched from `github.com/<user>.keys`
- When encrypting to multiple recipients, all of their keys are combined
- The file is encrypted to every key, so any recipient can decrypt with any of their keys
- The ciphertext is ASCII-armored for safe transport

When decrypting:

- Your local SSH private key (defaults to `~/.ssh/id_ed25519`) is used
- Both Ed25519 and RSA keys are supported

The sender never sees or handles private keys. GitHub acts as a public key directory.

## Supported key types

| Key type | Encrypt | Decrypt |
|----------|---------|---------|
| Ed25519  | Yes     | Yes     |
| RSA      | Yes     | Yes     |

## Project structure

```
nvlp/
  nvlp-core/   # Library: encryption, GitHub API
  nvlp/        # Binary: the 'nvlp' command
```

## Contributing

Contributions are welcome! Some areas that could use help:

- Platform-specific packaging (Homebrew, AUR, Nix)
- Better error messages and progress output
- Support for alternative transports beyond GitHub Gists

## License

[MIT](LICENSE)
