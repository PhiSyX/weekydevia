if ( $args.count -lt 1 ) {
    echo "Range date is mandatory, e.g: `20240529_20240602` which whould be a directory inside the draft directory"
    exit(1)
}

$date = $args[0]

cargo run -- -d ./draft/$date -o ./release/$date $args[1]