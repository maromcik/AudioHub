FROM rust:1.75

WORKDIR /usr/src/audiobooks

RUN apt-get update
RUN apt-get install -y postgresql-client libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev
RUN apt-get install -y gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad
RUN apt-get install -y gstreamer1.0-plugins-ugly gstreamer1.0-libav libgstrtspserver-1.0-dev libges-1.0-dev

COPY . .
RUN mkdir media

RUN cargo install --path .

CMD ["audiobooks"]