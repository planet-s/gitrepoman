# Git Repo Manager

To use, `secret.toml` must be present in the working directory.

## Examples

### secret.toml

```
gitlab = API_TOKEN
github = API_TOKEN
```

### Usage

```
gitrepoman github pop-os clone
gitrepoman github pop-os pull
gitrepoman github pop-os checkout

gitrepoman gitlab gitlab.redox-os.org clone
gitrepoman gitlab gitlab.redox-os.org pull
gitrepoman gitlab gitlab.redox-os.org checkout
```
