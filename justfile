default:
    just -l

cli *ARGS:
    cargo run --bin wcspr_cli {{ARGS}}
