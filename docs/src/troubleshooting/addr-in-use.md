# AddrInUse

This error happens when figmaid is trying to bind a port that is already in use by another program.

## Solution

1. See what program is using the port 
   - `sudo netstat -ltnp | grep  '18412'` 
   - If you don't have `netstat` you can replace it with `ss`, the output will be a little different.
2. Kill the program
   
   ```txt
     0.0.0.0:*             LISTEN      70793/figmaid
                                       ^pid  ^process name
    ```
   - With process name: `pkill -e figmaid`
   - With process ID: `kill 70793`
 
For services you can forcibly stop the service by running above commands as sudo, but 
stopping the service with `sudo systemctl stop <process name>` is recommended. 