FROM ubuntu:16.04

RUN apt-get update && apt-get install -y \
    curl \
    libopenal-dev \
    libsndfile1-dev \
    libssl-dev \
    libgtk-3-dev

RUN set -xe \
    && cd /tmp \
    && curl https://sh.rustup.rs -o rustup.sh \
    && chmod +x rustup.sh \
    && ./rustup.sh -y

WORKDIR "/app"

CMD ["cargo", "build", "--release"]
