# weekydevia

## Releases

Releases are published every Thursday, every 2 weeks, except in case of
unavailability or lack of links.

Last Release: [2024.06.10 - 2024.06.16](release/20240610_20240616/README.md)

- [2024.06.10 - 2024.06.16](release/20240610_20240616/README.md)
- [2024.06.03 - 2024.06.09](release/20240603_20240609/README.md)
- [2024.05.27 - 2024.06.02](release/20240527_20240602/README.md)


## Usage, memory note:

```ps1
./scripts/release 20240527_20240602
# ./scripts/release 20240527_20240602 --delete
```

This command will generate a `release/20240527_20240602` directory containing
the markdown files from the `draft/20240527_20240602` directory, with the
contents of the default template located in
[draft/TEMPLATE.md](draft/TEMPLATE.md). The entry point for the draft directory
is the `README.md` file. This file can include other markdown files with the
following syntax `#include <file.md>`.

Note that the markdown files beginning with `_` are not included in the release
directory.

The `--delete` option will delete the `draft/20240527_20240602` directory after
publish a new release.

### Example

$ `tree draft/`

```diff
draft/
├── 20240527_20240602/
│   ├── README.md
│   ├── _file1.md  (## Hello World)
│   ├── file2.md   (## Hello Friends)
│   └── _file3.md  (## Hello Dev's)
└── TEMPLATE.md
```

$ `cat draft/20240527_20240602/README.md`

```md
#include <_file1.md>

#include <file2.md>

**Example Title**  
https://example.org/

#include <_file3.md>

--- THE END ---
```

$ `./scripts/release 20240527_20240602`

$ `tree release/`

```diff
 release/
+├── 20240527_20240602/
+│   ├── README.md
+│   ├── file2.md
 └── feed.xml
```

$ `cat release/20240527_20240602/README.md`

```md
# 2024.05.27 - 2024.06.02

Hello and welcome to another issue of weekydevia. This is a summary of the
resources, readings and news I've read and found interesting to share this week.

Links may be out of date, this is not a newsletter about new resources only.

## Hello World

## Hello Friends

**Example Title**  
[https://example.org/](https://example.org/)

## Hello Dev's
```
