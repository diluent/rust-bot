FROM rustlang/rust:nightly

WORKDIR ~/test/rust/rust-bot
COPY . .

RUN cargo install --path . --verbose

CMD ["rust-bot"]
