# Running figmaid in the background

Figmaid doesn't run in the background by default. You can configure this yourself if you need it.

## Docker

Docker has a powerful feature called restart policies, which [allows containers to start automatically if they exit or if Docker restarts](https://docs.docker.com/config/containers/start-containers-automatically/), as such, running figmaid inside Docker is a great way of keeping figmaid alive. Read more in the dedicated Docker section: [figmaid in Docker](./docker-image.md).
