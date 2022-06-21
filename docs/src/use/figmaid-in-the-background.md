# Running figmaid in the background

Figmaid doesn't run in the background by default. You can configure this yourself if you need it.

## pm2 

This is probably the easiest option if you have Node.js installed.

```sh
npm i -g pm2

pm2 start --name figmaid --interpreter none "figmaid"
```

Verify that process started successfully by running `pm2 ls`.

To see how to manage the process refer to [pm2's documentation](https://pm2.keymetrics.io/).

> Pm2 can [persist your applications](https://pm2.keymetrics.io/docs/usage/startup/) so they will be restarted when you reboot, similar how to how container restart policies work in Docker. 

### Some useful commands

|                      |                                                                          |
| :------------------- | :----------------------------------------------------------------------- |
| `pm2 reload figmaid` | You may need to reload the server if you changed the configuration file. |
| `pm2 logs figmaid`   | If `figmaid` failed to start this command shows you the context.         |

