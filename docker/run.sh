#!/bin/bash

docker build -t timbre docker/
docker run -it --rm \
    -v $(pwd):/app \
    timbre
