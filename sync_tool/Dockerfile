FROM rust:latest AS builder


RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    pkg-config \
    libssl-dev \
    libz-dev \
    && rm -rf /var/lib/apt/lists/*

# 创建一个用户 rustuser   确保和宿主机的用户id一致
RUN useradd -m -u 1000 rustuser

COPY . /sync_tool
WORKDIR /sync_tool

# 更改文件的所有权
RUN chown -R rustuser:rustuser /sync_tool

# 切换到 rustuser 用户
USER rustuser

RUN cargo build --release

FROM debian:latest


RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    libz-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# 创建一个用户 rustuser 确保和宿主机的用户id一致
RUN useradd -m -u 1000 rustuser

COPY --from=builder /sync_tool /workspace

# 更改文件的所有权
RUN chown -R rustuser:rustuser /workspace
# 切换到 rustuser 用户
USER rustuser

WORKDIR /workspace/target/release

CMD ["/bin/bash"]

