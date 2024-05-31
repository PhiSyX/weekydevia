# weekydevia

## Usage, memory note:

```ps1
./scripts/release 20240527_20240602
# ./scripts/release 20240527_20240602 --delete
```

This command will generate a `release/20240527_20240602` directory containing
the markdown files from the `draft/20240527_20240602` directory, with the

contents of the default template located in
[draft/TEMPLATE.md](draft/TEMPLATE.md).

Markdown files beginning with `_` are not included in the release directory.

The `--delete` option will delete the `draft/20240527_20240602` after publish
a new release.

## Releases

Releases are published every Thursday, except in case of unavailability or lack
of links.

Last Release: [2024.06.03 - 2024.06.09](release/20240603_20240609/README.md)

- [2024.06.03 - 2024.06.09](release/20240603_20240609/README.md)
- [2024.05.27 - 2024.06.02](release/20240527_20240602/README.md)
