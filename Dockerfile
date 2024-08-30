FROM ghcr.io/cross-rs/armv7-unknown-linux-gnueabihf:edge@sha256:3e1def581eb9c9f11cfff85745802f2de5cf9cdeeb5a8495048f393a0993b99b

# COPY .cargo/config.toml /root/.cargo/config.toml

RUN dpkg --add-architecture armhf

RUN apt-get update -y
RUN apt-get install -y libc6-dev-armhf-cross libudev-dev:armhf libssl-dev:armhf 
RUN apt-get install -y pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev