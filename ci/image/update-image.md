To update and publish a newer version of the docker image for Circle CI:

Change to the folder containing `Dockerfile`:
```
$ cd cubeb-rs/ci/image
```

Build new version of `cubeb-rust` image (*note*: `.`):
```
$ docker build -t cubeb-rust .
```

Get the `IMAGE ID` for the image from `docker images` (Eg: `38e978bb303d`):
```
$ docker images
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
cubeb-rust          latest              38e978bb303d        7 minutes ago       4.22GB
...
```

Tag the image with `mozmedia/cubeb-rust:latest`:
```
$ docker tag 38e978bb303e mozmedia/cubeb-rust:latest
```

Log into the mozmedia account:
```
$ docker login --username=mozmedia
Password:
...
Login Succeeded
```

Push the new image to docker hub:
```
$ docker push mozmedia/cubeb-rust
```

Once the `push` has completed, details can be found at [mozmedia docker hub](https://cloud.docker.com/u/mozmedia/repository/docker/mozmedia/cubeb-rust)
