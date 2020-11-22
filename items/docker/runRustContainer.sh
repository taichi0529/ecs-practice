#!/bin/bash

SCRIPT_DIR=$(cd $(dirname $(readlink $0 || echo $0));pwd)
cd ${SCRIPT_DIR}
cd ..
SRC_DIR=$(pwd)
echo ${SRC_DIR}
cd ${SCRIPT_DIR}

docker run --name ecs-practice.items.rust --rm -d -v ${SRC_DIR}:/usr/src/app ecs-practice/rust sh -c "while :; do sleep 10; done"
