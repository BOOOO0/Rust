# Build stage
FROM rust:1.80 AS builder

# 작업 디렉토리 설정
WORKDIR /app

# 의존성 캐시를 위한 파일 복사
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# 소스 코드 복사 및 최종 빌드
COPY src ./src
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

# 필요한 라이브러리 설치
RUN apt-get update && \
    apt-get install -y libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# 빌드된 바이너리 복사
COPY --from=builder /app/target/release/crud /usr/local/bin/crud

ENTRYPOINT ["/usr/local/bin/crud"]

# 포트 노출 및 실행 명령어 설정
EXPOSE 3030
