# Command-line interface

figmaid has a small CLI to perform common tasks. It ships as a single executable with a single subcommand to work with the configuration.

## `figmaid`
  Starts the server. It starts on the foreground by default.

## `figmaid config`
```txt
Create, open and validate configuration

Run without subcommands to print current directories and amount of loaded fonts

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