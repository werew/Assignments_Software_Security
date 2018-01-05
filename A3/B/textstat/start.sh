cargo afl build && \
cargo afl fuzz -i ../fuzzing/input/ -o output target/debug/textstat

