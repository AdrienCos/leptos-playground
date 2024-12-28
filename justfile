format:
    leptosfmt .
    cargo fmt

build:
    trunk build

build-release:
    trunk build --release --minify

clean:
    trunk clean
    cargo clean

serve:
    trunk serve --open
