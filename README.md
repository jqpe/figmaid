# figmaid

Web server that allows you to use locally installed fonts in Figma on Linux.


## Install
`cargo install --git https://github.com/jqpe/figmaid` 

To add additional directories ([aside from the default configuration](https://figmaid.nykanen.me/usage.html#configuration)) run `figmaid config create && figmaid config open`.  

NB: the port option is exposed for use with reverse proxies, Figma always requests fonts from localhost:18412.

macOS includes user fonts in ~/Library/Fonts/ and system fonts in /Library/Fonts.
Windows has all fonts inside C:\Windows\Fonts.

> Although figmaid supports Windows/macOS you're probably better off using [Figma Agent](https://help.figma.com/hc/en-us/articles/360039956894-Access-local-fonts-on-your-computer#browser) unless you need custom directories.
> If you only intend on using installed fonts, Figma Agent will pick those up.

The default configuration will use directories for Windows/macOS/Ubuntu. For other Linux distros check your [font configuration file](https://linux.die.net/man/5/fonts-conf).
  
To start the web server run `figmaid` and you should see a log about figmaid being running on port 18412.
If it panics with Address already in use, check for running processes with `sudo netstat -ltnp | grep -w '18412'`.
If you're using [figma-linux-font-helper](https://github.com/Figma-Linux/figma-linux-font-helper) you need to stop the service with `sudo systemctl stop fonthelper`
  
## Running in the background (unix)
`figmaid > /dev/null &` or use a terminal multiplexer. The server will not be restored when you restart. To stop the server use `pkill -e figmaid`.
  
## Differences with figma-linux-font-helper
- Small. Includes only the barebones in terms of functionality.
- Correctly handles CORS preflight requests. Not implementing these is probably why some users couldn't get figma-linux-font-helper to work.
- Otherwise equilavent functionality, but figmaid provides correct font weights.
- Works without root access. 
- No service as of yet. figma-linux-font-helper runs on background by default and the server is restarted when you reboot. 
