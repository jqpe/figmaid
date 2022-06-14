# My fonts are not showing up

Check that you have the server running: `sudo netstat -ltnp | grep figmaid`
   
If it prints nothing, start the server with `figmaid`. 

If it prints something, but you are still not seeing your fonts:

1. Ensure you have the font installed or you have specified the directory where it lives in, see [usage](../usage.md).
   
2. If all else fails, the font might not have correct tables. I'd apprecatiate if you'd take the time to open a issue with the font file included.

