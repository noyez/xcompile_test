
FROM dockcross/linux-armv7

ENV DEFAULT_DOCKCROSS_IMAGE noyez/dockcore-armv7


ENV RUSTUP_HOME=/opt/rust
ENV CARGO_HOME=/opt/rust 
ENV PATH=$PATH:/opt/rust/bin
ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV PKG_CONFIG_ALLOW_CROSS=1

ENV ARCH=armv7
 
ENV CROSS_COMPILE=arm-linux-gnueabihf
#ENV RANLIB=arm-linux-gnueabihf-ranlib
#ENV NM=arm-linux-gnueabihf-nm
#ENV STRIP=arm-linux-gnueabihf-strip
#ENV CHOST=arm-linux-gnueabihf
ENV TARGET=arm-linux-gnueabihf

#RUN curl  https://sh.rustup.rs -sSf | sh -s --  -y --default-host armv7-unknown-linux-gnueabihf --default-toolchain nightly
#RUN curl  https://sh.rustup.rs -sSf | sh -s --  -y --default-toolchain nightly
RUN curl  https://sh.rustup.rs -sSf | sh -s --  -y
#RUN curl  https://sh.rustup.rs -sSf | sh -s --  -y --default-host armv7-unknown-linux-gnueabihf --default-toolchain nightly-2017-10-15
#RUN /opt/rust/bin/rustup target add armv7-unknown-linux-gnueabihf 
#RUN rustup install nightly
#RUN rustup override set nightly
RUN rustup target add armv7-unknown-linux-gnueabihf 

