FROM alpine
COPY ./target/x86_64-unknown-linux-musl/release/heshab /root
COPY public /root/public
CMD ["/root/heshab"]