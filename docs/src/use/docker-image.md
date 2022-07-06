# figmaid in Docker

You can also run figmaid inside a container. This comes with obvious benefits such as Docker's restart policies, which means you don't need to restart figmaid each time you login. The minor caveat is that specifying additional directories is more of a chore and the container includes an operating system, adding roughly 50MB.

Nevertheless, the benefits overshadow the caveats here for most people. Let's see it! 

```sh
user="$(whoami)"

docker run -d -p 18412:18412 \
-v /usr/share/fonts:/usr/share/fonts:ro \
-v /home/$user/.config/figmaid/figmaid.json:/root/.config/figmaid/figmaid.json:rw \
--name figmaid --restart always \
nykanen/figmaid
```

Step by step — what is the above telling Docker? First, the `-d` short flag starts the container in detached mode, meaning there's no shell output nor input. Secondly, you *must* bind the port 18412 to your local computer, this is done with the `-p` short flag. Otherwise figmaid will only be accessible from within the container which is not very useful.

The `-v` short flag stands for volume, and as you'd expect, it binds a volume from your computer to be accessible within the container. Again, this is a requirement in almost all cases, unless you intend to install fonts within the container, which by any means you _could_ do. Above, we expose two volumes: the directory for installed fonts in Ubuntu / Linux Mint, and the configuration file. The short `:ro` label enforces that `figmaid` doesn't have access to write to those files — completely optional as figmaid only reads those files, but explicitness is great! For the configuration file `:rw` is implied, this allows you to use commands like `figmaid config create` inside the container if you don't have a local installation of figmaid. 

That's the gist of it, but take the time to understand the arguments. Before running the command blindly it is probably beneficial to lay out a plan so figmaid works for you in the future with minimal hassle. These docs will still be waiting for you if you need to change stuff later.

After succesfully configuring and starting figmaid, the output of `docker container ls` would be similar to below:

```sh
CONTAINER ID   IMAGE             COMMAND                  CREATED      STATUS       PORTS
c07e070bb470   nykanen/figmaid   "sh -c 'HOST=${HOST}…"   0 days ago   Up 0 hours   0.0.0.0:18412->18412/tcp, :::18412->18412/tcp
```

Take note of the PORTS value. 0.0.0.0:18412 is available from 18412/tcp on your local machine. To make sure that everything really went as expected, open http://localhost:18412/figma/font-files on a browser and you should get a big blob of JSON.

## Adding additional directories

Simply changing your configuration file is not enough, you must bind the directory from your host to the container. If you find you're adding a lot of directories, you can use figmaid's ability to walk directories. If you have fonts in $HOME/design/fonts and $HOME/Downloads specifying just $HOME might be the way to go. This is not free though — figmaid would walk _all_ directories in $HOME — so pick a good middleground, unfortunately this is up to you to figure out. 

To add new bind mounts to figmaid you must first delete the first iteration, e.g. `docker kill figmaid -f` then create a new container with the additional bind mount:

```sh
user="$(whoami)"

docker run -d -p 18412:18412 \
-v /usr/share/fonts:/usr/share/fonts:ro \
-v /home/$user/Downloads:/root/Downloads:ro \
-v /home/$user/.config/figmaid/figmaid.json:/root/.config/figmaid/figmaid.json:rw \
--name figmaid --restart always \
nykanen/figmaid
```

Above we bound the default directory for downloaded files on Ubuntu to `/root/Downloads`. Remember to add this directory to your configuration file by editing `/home/$user/.config/figmaid/figmaid.json` and add the directory for *what the directory path is inside the container*. In this case our configuration file would contain the following directories: `/usr/share/fonts` and `/root/Downloads`.