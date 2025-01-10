FROM --platform=linux/amd64 rust:slim-bookworm
RUN apt update -y && apt upgrade -y
RUN apt-get -y  install protobuf-compiler build-essential libclang-dev clang
WORKDIR /app
#COPY ../../homework-1/1606/polkadot-sdk-solo-template-dev-courses /app
#RUN cd polkadot-sdk-solo-template-dev-courses &&  cargo build --release
#RUN ./target/release/node-template --dev --tmp

