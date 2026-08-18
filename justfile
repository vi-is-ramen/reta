default:
    @just --list

check:
    @cargo fmt --all
    @cargo clippy --all-targets --all-features -- -D warnings

test:
    @echo '-- NDF AT'
    @cargo test --no-default-features --all-targets
    @echo '-- AF AT'
    @cargo test --all-features --all-targets
    @echo '-- D AF'
    @cargo test --doc --all-features
    @python scripts/test.py core

chlog *a:
    @python scripts/chlog.py {{a}}

clean:
    @cargo clean

pre-commit: check test

docs:
    @RUSTDOCFLAGS="--cfg docsrs" cargo doc --no-deps --all-features --open
