# Chapter 2

Get help of any command in docker

```sh
docker help
```

## Project 1 - Website monitor

3 Dockers:

- nginx
- mailer
- watcher

Creating a new container running as daemon with the name web using the last image of nginx:

```sh
docker run --detach --name web nginx:latest
```

Creating a new container running as daemon with the name web using the last image of nginx:

```sh
docker run -d --name mailer dockerinaction/ch_mailer
```

Creating a new interactive container with a tty and giving access to the webserver (just to test the connection):

```sh
docker run -d --interactive --tty --link web:web --name web_test busybox:latest /bin/sh
```

Creating the watcher to watch the server:

```sh
docker run -it --link web:insideweb --link mailer:insidemailer dockerinaction:ch2_agent
```

### List, stop, restart and view output of containers

To check which containers are currently running:

```sh
docker ps
```

To check all containers:

```sh
docker ps -a
```

To restart some container:

```sh
docker restart <conainter-name>
```

Examining logs of the container:

```sh
docker log <conainter-name>
```

### PID namespace

Example of creating a docker without PID

```sh
docker run --pid host busybox:latest ps
```

### Flexible container identification

To rename a docker, the following command can be used:

```sh
docker rename <current name> <new name>
```

To execute an command into the container the following command can be used:

```sh
docker exec <contained ID> <command>
```

The container ID can be also used in `docker exec` and `docker stop`

To evade misunderstanding in scripts we use the entirely ID of a container

Some examples of containers in scripts:

- [script 1](./scripts/script_1) - Using environment variables
- [script 2](./scripts/script_2) - Using CID files




