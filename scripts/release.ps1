if ( $args.count -lt 1 ) {
    echo "Range date is mandatory, e.g: `20240527_20240602` which whould be a directory inside the draft directory"
    exit(1)
}

$date = $args[0]

if (Test-Path -Path ./target/release) {
    ./target/release/generate.exe -d ./draft/$date -o ./release/$date $args[1]
} elseif (Test-Path -Path ./target/debug) {
    ./target/debug/generate.exe -d ./draft/$date -o ./release/$date $args[1]
} else {
    cargo run --release -- -d ./draft/$date -o ./release/$date $args[1]
}
