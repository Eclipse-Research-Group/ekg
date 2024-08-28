FROM ghcr.io/cross-rs/armv7-unknown-linux-gnueabihf:edge@sha256:3e1def581eb9c9f11cfff85745802f2de5cf9cdeeb5a8495048f393a0993b99b

# COPY .cargo/config.toml /root/.cargo/config.toml

RUN dpkg --add-architecture armhf

RUN apt-get update -y
RUN apt-get install --assume-yes --no-install-recommends \
    gcc-arm-linux-gnueabihf \
    g++-arm-linux-gnueabihf \
    libc6-dev-armhf-cross \
    libudev-dev:armhf \
    libssl-dev:armhf \
    build-essential \
    make \ 
    cmake

RUN apt-get install -y libasound2-dev mesa-common-dev libx11-dev libxrandr-dev libxi-dev xorg-dev libgl1-mesa-dev libglu1-mesa-dev -y
RUN apt-get install -y libglfw3-dev libglfw3 wayland-protocols libecm-dev

# ENV CMAKE_MODULE_PATH="/"