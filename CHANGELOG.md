## v1.0.1 (2023-02-06)

### Fix

- problematic filenames

## v1.0.0 (2022-06-22)

### Fix

- replace `:` with `-` when generating filename

### BREAKING CHANGE

- Files using a `:` cannot be synced to Android using Syncthing. The filename replaces `:` with ` -` (space + minus). In order to use this version, rename your files before.
You can use `rename` on Linux and install it on Mac `brew install rename`.
```
rename 's/:/ \-/g' kindle-notes/*.md
```

## v0.2.3 (2022-03-09)

### Fix

- remove non-ascci chars

## v0.2.2 (2022-01-31)

### Fix

- remove duplicates when cleaning notes

## v0.2.1 (2021-12-10)

### Refactor

- remove unused code

## v0.2.0 (2021-12-10)

### Feat

- remove duplicated entries in the notes
- add some feedback
- add output folder button
- connect core to gui
- add parallelism
- add output folder and path
- write books to its files
- add book separator
- initial draft

### Fix

- create file or open if exists
