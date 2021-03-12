#docker run --rm -it \
#-v $(pwd):/usr/src/runtime \
#-v $(pwd)/../models:/usr/src/models \
#-v $(pwd)/../models-derive:/usr/src/models-derive \
#-v $(pwd)/docker-target:/usr/src/runtime/docker-target \
#-v $(pwd)/docker-cargo/git:/usr/local/cargo/git \
#-v $(pwd)/docker-cargo/registry:/usr/local/cargo/registry \
#-w /usr/src/runtime \
#--cpus=0.000 \
#rust cargo build --target-dir /usr/src/runtime/docker-target --offline &&
#echo '/usr/src/runtime/docker-target/debug/runtime' | pbcopy &&
#docker run --rm -it \
#-v $(pwd):/usr/src/runtime \
#-w /usr/src/runtime \
#-p 8080:8080 \
#-e RUST_LOG=runtime \
#rust /bin/bash
##/usr/src/runtime/docker-target/debug/runtime

cargo build --release --target=x86_64-unknown-linux-musl && \
docker build -f docker/Dockerfile -t goldrush:0.1 . && \
docker rmi stor.highloadcup.ru/rally/puma_flyer
docker tag goldrush:0.1 stor.highloadcup.ru/rally/puma_flyer && \
docker push stor.highloadcup.ru/rally/puma_flyer
