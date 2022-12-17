# Running figmaid in the background

figmaid doesn't run in the background by default. You can configure this yourself if you need it.

## Docker

Docker has a powerful feature called restart policies, which [allows containers to start automatically if they exit or if Docker restarts](https://docs.docker.com/config/containers/start-containers-automatically/), as such, running figmaid inside Docker is a great way of keeping figmaid alive. Read more in the dedicated Docker section: [figmaid in Docker](./docker-image.md).

## As systemd unit

If you are using Ubuntu, or a similar distro that uses systemd, you can create a unit file for figmaid in just a few minutes.

Prerequisites:
- You are a sudoer
- You know the executable path of figmaid (use `which figmaid`) 
- You know your username
- Ideally you'd have a basic understanding of what systemd units do. Read more at the [official systemd website](https://systemd.io).

While systemd supports user units, having them reload on reboot is not as straightforward. It would require enabling lingering with `loginctl`, with the major caveat that every single user service would be started on reboot. 

### Create the unit file

> 1. Below, replace `<your username>` with the user you installed figmaid to.
> 2. If you used the installation script, the executable path is `/home/<your username>/.local/bin/figmaid` .
> 3. Replace `<figmaid executable>` with the path to figmaid. 

The code block is editable, and changes you made will be present if you click the copy button.

Change the `User` and `ExecStart` properties as described above.


```txt,editable
[Unit]
Description=figmaid — font daemon for figma
Documentation=https://figmaid.pages.dev
Wants=network-online.target
After=network-online.target

[Service]
User=<your username>

ExecReload=/bin/kill -HUP $MAINPID
ExecStart=<figmaid executable>
Restart=on-failure
RestartSec=2

[Install]
WantedBy=multi-user.target
```

Save the file to `/etc/systemd/system/figmaid.service`.

Then, to enable, start, and view the status of the service run the below commands:

```sh
sudo systemctl enable figmaid
sudo systemctl start figmaid
sudo systemctl status figmaid
```

### View logs

```sh
sudo journalctl -eu figmaid
```

Here `-e` tells journalctl that we want it to automatically scroll to the end. `-u` tells that we're asking for an unit.


### Update configuration

Currently figmaid doesn't reload the configuration when you reload the service. Instead, you must restart the service.

```sh
sudo systemctl restart figmaid
```

Note that *for already configured directories*, no restarting is necessary. Instead, you simply have to add the font to the configured directory and reload Figma.
