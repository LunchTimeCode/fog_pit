set dotenv-load := true

alias v := verify
alias r := run

bt := '0'
log := "warn"

@_list:
    just --list --unsorted

run *args:
    cargo run -q -- {{args}}

install:
    cargo install --path .

# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    echo ------------ verify done! ------------

# Run the tests
test:
    cargo test

# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo clippy

# Format the code
fmt:
    cargo fmt


dist:
    cargo dist init
    cargo dist generate