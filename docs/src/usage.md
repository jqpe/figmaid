# Usage

On *nix operating systems you can use the installation script. See the table below for supported architectures.

`curl -fsSL https://raw.githubusercontent.com/jqpe/figmaid/main/install.sh | sh`

| target                   | binary name               | install script |
| :----------------------- | ------------------------- | -------------- |
| x86_64-apple-darwin      | figmaid-macos-amd64       | ✓              |
| aarch64-apple-darwin     | figmaid-macos-aarch64     | ✓              |
| x86_64-unknown-linux-gnu | figmaid-ubuntu-amd64      | ✓              |
| x86_64-pc-windows-gnu    | figmaid-windows-amd64.exe | ⚠              |

<br/>
<br/>

If you are using Windows or an unsupported architecture you need to [install Rust](https://www.rust-lang.org/tools/install) and build from source.

`cargo install --git https://github.com/jqpe/figmaid` 

---

After the installation is complete you have `figmaid` ready to go!

## Configuration

The default configuration looks like this: 

```json
{{#include ../figmaid.json}}
```

The configuration file lives in $HOME/.config/figmaid/figmaid.json. Additionally you can use environment variables PORT and DIRS, they will have preference. 

figmaid provides the config subcommand to make working with this file less painful:

```txt
Create, open and validate configuration

USAGE:
    figmaid config <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    create      Create default configuration file
    help        Print this message or the help of the given subcommand(s)
    open        Open configuration file in text editor
    validate    Validate configuration
```


If you are using Ubuntu, Linux Mint, macOS or Windows no additional configuration is required if you only wish to use installed fonts. 

For other Linux distros, refer to their documentation.

> **Custom directories** figmaid doesn't care about where your font files are located or whether they're installed or not.
> To add a custom directory, specify it in `directories`.
> The path to the directory doesn't need to be absolute, if you have fonts in /Design/fonts/<sub_folder>/**/* you can just specify /Design/fonts.

## Note about directories

1. The directory path doesn't have to be absolute, but operating system dependent shortcuts like the tilde ~ are not supported.
2. figmaid has to have access to the directory. This usually means that the user that installed figmaid has to have access.

## Running in the background

`figmaid > /dev/null &` or use a terminal multiplexer. The server will not be restored when you restart. To stop the server use `pkill -e figmaid`.

This delegates the process to your operating systems [job control](https://en.wikipedia.org/wiki/Job_control_(Unix)).