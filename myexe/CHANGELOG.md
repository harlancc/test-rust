# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

/usr/lib/x86_64-linux-gnu/pkgconfig/openssl.pc

## [Unreleased]

## [0.1.20] - 2024-02-24
  - test 0.1.19

## [0.1.19] - 2024-02-24
  - test 0.1.19

## [0.1.18] - 2024-02-24
  - test 0.1.18

## [0.1.17] - 2024-02-24
  - test 0.1.17

## [0.1.16] - 2024-02-24
  - test 0.1.16

## [0.1.15] - 2024-02-24
  - test 0.1.15

## [0.1.14] - 2024-02-24
  - test 0.1.14

## [0.1.13] - 2024-02-24
  - test 0.1.13

## [0.1.12] - 2024-02-24
  - test 0.1.12

## [0.1.11] - 2024-02-24
  - test 0.1.11

## [0.1.10] - 2024-02-24
  - test 0.1.10

## [0.1.9] - 2024-02-24
  - test 0.1.9

## [0.1.8] - 2024-02-24
  - test 0.1.8

## [0.1.7] - 2024-02-24
  - test 0.1.7

## [0.1.6] - 2024-02-24
  - test 0.1.6

## [0.1.5] - 2024-02-24
  - test 0.1.5

## [0.1.4] - 2024-02-24
  - test 0.1.4

## [0.1.3] - 2024-02-24
  - test 0.1.3

## [0.1.2] - 2024-02-24
  - test

- test v0.1.2

## [0.1.1] - 2024-02-24

- test v0.1.1

## [0.1.0] - 2024-02-23

- test v0.1.0


docker run -d --restart=always --ulimit core=-1 --ulimit nofile=200000 --privileged=true --ipc=host --shm-size=1g --name live-stream-mixer_8001 --net="host" --expose=8001 -w /data1/live-stream-mixer/ -e "LANG=" -e "DOCKER_IMAGE=127.0.0.1:65001/live-stream-mixer/live-stream-mixer" -e "DOCKER_VERSION=202401261120" -e "DISPATCHER_HOST=sz.live.stream.api.weibo.com" -e "DISPATCHER_PORT=80" -e "AGENT_HOST=127.0.0.1" -e "AGENT_PORT=8090" -e "MIXER_INNER_IP=10.94.2.228" -e "MIXER_OUTER_IP=47.107.95.212" -e "MIXER_PORT=8001" -e "RTC_UDP_PORT=8501" -e "PKG_CONFIG_PATH=/usr/local/lib/pkgconfig/" -e "LD_LIBRARY_PATH=/usr/local/lib64:/usr/local/lib:/usr/lib64:/usr/lib:/lib64:/lib" -v /data1/live-stream-mixer/download:/data1/live-stream-mixer/download -v /data1/live-stream-mixer_8001/logs:/data1/live-stream-mixer/logs -v /data1/live-stream-mixer_8001/coreDump:/data1/live-stream-mixer/coreDump 127.0.0.1:65001/live-stream-mixer/live-stream-mixer:202401261120 /data1/live-stream-mixer/start_mixer.sh 8001


