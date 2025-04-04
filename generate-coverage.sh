#!/bin/bash
export LLVM_PROFILE_FILE="target/coverage/%p-%m.profraw"
export CARGO_TERM_COLOR=always
export RUSTFLAGS="-Cinstrument-coverage"
export SONAR_SCANNER_OPTS="-Dsonar.coverageReportPaths=target/coverage/cobertura.xml"
export SONAR_TOKEN=$SONAR_TOKEN

rustup component add llvm-tools-preview
cargo install grcov
cargo build
mkdir -p target/coverage
cargo test --verbose --no-fail-fast
grcov target/coverage \
        --binary-path target/debug \
        -s . \
        --branch \
        -o target/coverage \
        --ignore-not-existing \
        --keep-only 'src/*' \
        --output-types html,cobertura
xmllint --xpath "concat('Coverage: ', 100 * string(//coverage/@line-rate), '%')" target/coverage/cobertura.xml
cat target/coverage/cobertura.xml
sonar-scanner --debug > sonar.log