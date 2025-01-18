export BROWSERSLIST_IGNORE_OLD_DATA := "1"

format:
    leptosfmt .
    cargo fmt
    ./node_modules/.bin/biome format --write .

lint:
    leptosfmt --check --quiet .
    cargo fmt --check
    cargo clippy
    ./node_modules/.bin/biome check .

test:
    earthly --push +test

build:
    trunk build

build-release:
    trunk build --release --minify

clean:
    trunk clean
    cargo clean
    rm -rf ./playwright-report
    rm -rf ./test-results

serve:
    trunk serve --open
