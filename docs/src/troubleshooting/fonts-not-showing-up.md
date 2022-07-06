# My fonts are not showing up

Check that you have the server running: `sudo netstat -l | grep 18412`, or with ss: `sudo ss -l | grep 18412`
   
If it prints nothing, start the server with `figmaid`. At this point if `figmaid` panics with `AddrInUse`, see [relevant docs](addr-in-use.md).

If it prints something, but you are still not seeing your fonts:

1. Ensure you have the font installed or you have specified the directory where it lives in, see [usage](../usage.md).
2. Reload Figma. Figma requests fonts when it's initialized and adding new fonts requires a refresh.
3. Run `figmaid config validate` to check that your configuration file is valid. If you accidentally mispelled something figmaid will fallback to the default configuration.
   -  For a quick fix run DIRS=/your/directory figmaid
4. Run `figmaid config open` and check that you specified the correct directory.  
5. Run `figmaid config`, this lists all directories and amount of loaded font files per directory. If you want to be extra spicy you can specify an absolute path to the direct parent of your font file with `DIRS=/my/fontdir figmaid config`. If figmaid picked up some fonts from this directory it would print your directory path followed by amount of fonts greater than zero.
6. If all else fails, the font might not have correct tables. I'd apprecatiate if you'd take the time to open a issue with the font file included.

figmaid figures out the font family by looking at it's metadata. In some cases the font family and filename don't match — 
in this case you should install the font and then open it in your platform's GUI and see what the font family actually is. 