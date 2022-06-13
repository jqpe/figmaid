# Usage

Install with Cargo:

`cargo install --git https://github.com/jqpe/figmaid` 

After the installation is complete you have `figmaid` ready to go!

## Configuration

The default configuration looks like this: 

```json
{{#include ../figmaid.json}}
```

If you are using Ubuntu/Linux Mint no additional configuration is required if you only wish to use installed fonts. 

The configuration file lives in $HOME/.config/figmaid/figmaid.json. Additionally you can use environment variables PORT and DIRS, they will have preference. To validate the configuration run `figmaid -t`, if the configuration doesn't exist it will ask if it should create it for you.

For other Linux distros, refer to their documentation.
