FROM ubuntu:latest
COPY --chmod=755 myexe /app/
COPY --chmod=755 lib.rs /app/
ENTRYPOINT ["/app/myexe"]