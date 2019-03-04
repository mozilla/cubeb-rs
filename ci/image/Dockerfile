FROM fedora:latest

RUN dnf update -y && \
    dnf install -y curl git gcc gcc-c++ cmake make pulseaudio-libs-devel && \
    dnf install -y glibc-devel.i686 pulseaudio-libs-devel.i686

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH "$PATH:/root/.cargo/bin"
ENV CFG_RELEASE_CHANNEL "nightly"

RUN rustup update && \
    rustup install nightly && \
    rustup toolchain install stable-i686-unknown-linux-gnu && \
    rustup toolchain install nightly-i686-unknown-linux-gnu && \
    rustup component add rustfmt-preview --toolchain nightly

RUN bash -l -c 'echo $(rustc --print sysroot)/lib >> /etc/ld.so.conf'
RUN bash -l -c 'echo /usr/local/lib >> /etc/ld.so.conf'
RUN ldconfig
