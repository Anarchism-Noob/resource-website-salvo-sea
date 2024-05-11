# 使用Rust官方的Rust镜像作为基础
FROM rust:latest as builder

# 设置工作目录
WORKDIR /usr/src/myapp

# 将依赖项缓存到一个单独的层中
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release

# 复制配置文件和静态资源文件夹到工作目录中
COPY config ./config
COPY assets ./assets

# 将源代码复制到工作目录中
COPY . .

# 构建应用程序
RUN cargo install --path .

# 第二个阶段，创建一个尽可能小的镜像
FROM debian:buster-slim

# 设置工作目录
WORKDIR /usr/src/myapp

# 复制二进制文件从builder阶段到当前阶段
COPY --from=builder /usr/local/cargo/bin/resource-website /usr/local/bin/resource-website

# 复制配置文件和静态资源文件夹到当前阶段
COPY --from=builder /usr/src/resource-website/config ./config
COPY --from=builder /usr/src/resource-website/assets ./assets

# 运行应用程序
CMD ["resource-website"]

# 构建镜像：docker build -t my-rust-web-app .
#运行：docker run my-rust-web-app