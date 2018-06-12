# Git Repo Manager

To use, `secret.toml` must be present in the working directory.

## Examples

### secret.toml

```
gitlab = API_TOKEN
github = API_TOKEN
```

### Man

```
USAGE:
    gitrepoman [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -f, --force      
    -h, --help       Prints help information
    -s, --ssh        
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    

SUBCOMMANDS:
    github    
    gitlab    
    help      Prints this message or the help of the given subcommand(s)
```

### Github Man

```
USAGE:
    gitrepoman github <DOMAIN> <ACTION>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <DOMAIN>    
    <ACTION>  
```

### Gitlab Man

```
USAGE:
    gitrepoman gitlab <DOMAIN> <NAMESPACE> <ACTION>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <DOMAIN>       
    <NAMESPACE>    
    <ACTION> 
```

### Examples

```
gitrepoman github pop-os clone
gitrepoman github pop-os pull
gitrepoman github pop-os checkout

gitrepoman gitlab gitlab.redox-os.org redox-os clone
gitrepoman gitlab gitlab.redox-os.org redox-os pull
gitrepoman gitlab gitlab.redox-os.org redox-os checkout
```
