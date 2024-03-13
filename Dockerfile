# build
FROM amazonlinux:2023 AS build-env
WORKDIR /app
COPY src ./src
COPY Cargo.toml Cargo.lock ./
RUN yum localinstall -y https://dev.mysql.com/get/mysql80-community-release-el9-5.noarch.rpm
RUN yum install -y git gcc
RUN yum install -y --enablerepo=mysql80-community mysql-community-client mysql-community-devel
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN $HOME/.cargo/bin/rustup show
RUN $HOME/.cargo/bin/cargo build --release

# deploy
FROM amazonlinux:2023
WORKDIR /app
RUN yum localinstall -y https://dev.mysql.com/get/mysql80-community-release-el9-5.noarch.rpm
RUN yum install -y git gcc
RUN yum install -y --enablerepo=mysql80-community mysql-community-client mysql-community-devel
COPY --from=build-env /app/target/release/onsen_tabi ./onsen_tabi
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["./onsen_tabi"]
