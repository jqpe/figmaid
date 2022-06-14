# Usage

Install with Cargo:

`cargo install --git https://github.com/jqpe/figmaid` 

After the installation is complete you have `figmaid` ready to go!

## Configuration

The default configuration looks like this: 

```json
{{#include ../figmaid.json}}
```

The configuration file lives in $HOME/.config/figmaid/figmaid.json. Additionally you can use environment variables PORT and DIRS, they will have preference. To validate the configuration run `figmaid -t`, if the configuration doesn't exist it will ask if it should create it for you.

If you are using Ubuntu/Linux Mint no additional configuration is required if you only wish to use installed fonts. 

For other Linux distros, refer to their documentation.

## Running in the background

`figmaid > /dev/null &` or use a terminal multiplexer. The server will not be restored when you restart. To stop the server use `pkill -e figmaid`.

This delegates the process to your operating systems [job control](https://en.wikipedia.org/wiki/Job_control_(Unix)).