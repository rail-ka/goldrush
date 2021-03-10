FROM alpine
MAINTAINER Rail Khusnutdinov <mail@rail-ka.ru>
EXPOSE 8000
VOLUME /app
WORKDIR /app
#RUN apk add --update alpine-sdk
#RUN mkdir /app/data
#COPY ./docker-entrypoint.sh /app/docker-entrypoint.sh
#RUN chmod 774 /app/docker-entrypoint.sh
#COPY ./target/x86_64-unknown-linux-musl/release/ /app/
COPY ./target/x86_64-unknown-linux-musl/release/goldrush ./goldrush
#CMD ["/bin/sh", "./docker-entrypoint.sh"]
ENV ADDRESS=localhost
CMD ["./goldrush"]
