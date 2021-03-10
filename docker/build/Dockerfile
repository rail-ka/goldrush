FROM ubuntu
MAINTAINER Rail Khusnutdinov <mail@rail-ka.ru>
RUN sudo apt-get update -y && \
    sudo apt-get upgrade -y
RUN sudo apt-get install curl gcc software-properties-common musl-tools
#RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -- --profile minimal
RUN /root/.cargo/bin/rustup target add x86_64-unknown-linux-musl
VOLUME /app
WORKDIR /app
#CMD /root/.cargo/bin/cargo clean && \
#    /root/.cargo/bin/cargo build --release --target=x86_64-unknown-linux-musl
CMD /root/.cargo/bin/cargo build --release --target=x86_64-unknown-linux-musl --target-dir docker-target
