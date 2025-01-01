# Compile
FROM    rust:1.83.0-alpine3.21 AS compiler

RUN     apk add -q --update-cache --no-cache build-base openssl-dev cmake
WORKDIR /
ENV     RUSTFLAGS="-C target-feature=-crt-static"
COPY    . .
RUN     set -eux; \
    cargo build --release


# Run
FROM    alpine:3.18

RUN     apk update --quiet \
    && apk add -q --no-cache libgcc tini curl openssl
# add system_test and meilitool to the `/bin` so you can run it from anywhere
# and it's easy to find.
COPY    --from=compiler /target/release/system_test /evas/system_test
# COPY    --from=compiler /src/configs/config.yaml /evas/src/configs/
# To stay compatible with the older version of the container (pre v0.27.0) we're
# going to symlink the system_test binary in the path to `/system_test`
EXPOSE  9000/tcp
EXPOSE  9001/tcp

ENTRYPOINT ["tini", "--"]
WORKDIR /evas
CMD   ["./system_test"]