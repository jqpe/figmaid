# My fonts are not showing up

Check that you have the server running: `sudo netstat -ltnp | grep figmaid`, or with lsof: `sudo lsof |grep '18412'`
   
If it prints nothing, start the server with `figmaid`. 

If it prints something, but you are still not seeing your fonts:

1. Ensure you have the font installed or you have specified the directory where it lives in, see [usage](../usage.md).
2. Reload Figma. Figma requests fonts when it's initialized and adding new fonts requires a refresh.
3. Run `fimgaid -t` to check that your configuration file is valid. If you accidentally mispelled something figmaid will fallback to the default configuration.
   -  For a quick fix run DIRS=/your/directory figmaid
4. If all else fails, the font might not have correct tables. I'd apprecatiate if you'd take the time to open a issue with the font file included.

