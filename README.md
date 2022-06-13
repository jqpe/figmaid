# figmaid

Web server that allows you to use locally installed fonts in Figma on Linux. It works on Windows/macOS as well, but you should probably use [Figma's  official app](https://help.figma.com/hc/en-us/articles/360039956894-Access-local-fonts-on-your-computer). 


## Install
`cargo install --git https://github.com/jqpe/figmaid` 

You can configure directories by creating a file in $HOME/.config/figmaid/figmaid.json and pasting the following:
  
 ```json
{
  "$schema": "https://raw.githubusercontent.com/jqpe/figmaid/main/docs/schema.json",
  "port": 18412,
  "directories": ["/usr/share/fonts", "/usr/lib/share/fonts"]
}
```
  <sub>If the above works for you (you're using Ubuntu and don't need additional folders), you don't need to create the file.</sub>

NB: the port option is exposed for use with reverse proxies, Figma always requests fonts from localhost:18412.

macOS includes fonts in /Library/Fonts/ and Windows in C:\Windows\Fonts, the default configuration targets Ubuntu.
  
To start the web server run `figmaid` and you should see a log about figmaid being running on port 18412. If it panics with Address already in use, check for running processes with `sudo netstat -ltnp | grep -w '18412'`. If you're using [figma-linux-font-helper](https://github.com/Figma-Linux/figma-linux-font-helper) you need to stop the service with `sudo systemctl stop fonthelper`
  
## Running in the background (unix)
`figmaid > /dev/null &` or use a terminal multiplexer. The server will not be restored when you restart. To stop the server use `killall figmaid`.
  
## Differences with figma-linux-font-helper
- Small. Includes only the barebones in terms of functionality.
- Correctly handles CORS preflight requests. Not implementing these is probably why some users couldn't get figma-linux-font-helper to work.
- Otherwise equilavent functionality, but figmaid provides correct font weights.
- Works without root access. 
- No service as of yet. figma-linux-font-helper runs on background by default and the server is restarted when you reboot. 
