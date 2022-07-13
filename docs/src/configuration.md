# Configuration

The default configuration looks like below:

```json
{{#include ../figmaid.json}}
```

The configuration file lives in $HOME/.config/figmaid/figmaid.json. Additionally you can use environment variables PORT and DIRS, they will have preference. 

If you are using Ubuntu or Fedora no additional configuration is required if you only wish to use installed fonts. Other Linux distros may or may not have the same directories, see your [font configuration file](https://linux.die.net/man/5/fonts-conf) to be sure. 

> **Custom directories** figmaid doesn't care about where your font files are located or whether they're installed or not.
> To add a custom directory, specify it in `directories`.
> The path to the directory doesn't need to be absolute, if you have fonts in /Design/fonts/<sub_folder>/**/* you can just specify /Design/fonts.

## Note about directories

1. The directory path doesn't have to be absolute, but shortcuts like the tilde ~ are not supported.
2. figmaid has to have access to the directory. This usually means that the user that started figmaid has to have access.
   1. On a related note: if you run as sudo, the home directory which figmaid uses to locate your configuration file will not be what you expect. If you for some reason want to run figmaid as sudo run the command with
      `HOME=/home/<user> figmaid` 
