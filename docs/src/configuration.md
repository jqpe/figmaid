# Configuration

The default configuration looks like below if you're on Linux, for macOS the directories property defaults to `/Library/Fonts` and `~/Library/Fonts`, for Windows `C:\Windows\Fonts`.

```json
{{#include ../figmaid.json}}
```

The configuration file lives in $HOME/.config/figmaid/figmaid.json. Additionally you can use environment variables PORT and DIRS, they will have preference. 

If you are using Ubuntu, Linux Mint, macOS or Windows no additional configuration is required if you only wish to use installed fonts. 

For other Linux distros check your [font configuration file](https://linux.die.net/man/5/fonts-conf)

> **Custom directories** figmaid doesn't care about where your font files are located or whether they're installed or not.
> To add a custom directory, specify it in `directories`.
> The path to the directory doesn't need to be absolute, if you have fonts in /Design/fonts/<sub_folder>/**/* you can just specify /Design/fonts.

## Note about directories

1. The directory path doesn't have to be absolute, but operating system dependent shortcuts like the tilde ~ are not supported.
2. figmaid has to have access to the directory. This usually means that the user that installed figmaid has to have access.
