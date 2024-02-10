# Changelog

## [v0.1.0] v0.1.0 (2023-02-09)
- Happy Chinese New Year
- Deprecated all original code
- Step by Step to rebuild a new tool set from v0.1.0

## [v0.2.4] v0.2.4 (2022-11-01)
- workaround to get rid of the annoying leading space caused by the xml prettify function. (unknown in which step it happens). It can be controlled via `clean_prettifed_code` in the command line. see `--help` for more details.

## [v0.2.3] v0.2.2 (2022-11-01)
- fix the typo "script"
- known issue: if the xml has been "prettified" (i.e. indented), the script will have redundant indents at the beginning of each line


## [v0.2.2] v0.2.2 (2022-11-01)
- fixed a bug which will remove some folders when there is a shape in the mashup.
- add log control support.

## [Unreleased]
## [v0.2.1] v0.2.1 (2022-04-07)

### Changed
- Fixed a bug the export will stop when the RemotePropertyBindings is empty.
- Fixed a bug on Windows that the letter ':' caused an error.

## [v0.2.0] v0.2.0 (2022-03-28)

### Changed

- [X] Support to export remote property bindings to the <export_root>/remote_property_bindings.csv file.

## [v0.1.5] v0.1.5 (2022-03-08)

### Changed

- [X] Don't export `Route` (json) and `Reflection` type services.

## [v0.1.4] v0.1.4 (2022-03-08)

### Changed
- [X] changed the leading comment string for sql script to '--'

## [v0.1.3] v0.1.3 (2022-03-08)

### Fixed
- [X] Fixed a bug where the `SQLQuery` and `SQLCommand` script were missed.

## [v0.1.2] v0.1.2 (2022-03-06)

### Changed

- Add change log

