# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Types of changes: Added, Changed, Deprecated, Removed, Fixed, Security.

## [0.1.3] - 2026-07-09

### Fixed
- report.rs:
    - counting of used_space uses saturating_sub method in fn report_disks
    - fn count_percent handles zero division (when full number is zero)
        - also added tests: fn test_count_percent_zero

## [0.1.2] - 2026-07-08

### Changed
- pub(crate) fn save_report returns std::io::Result<()>; save.rs
- pub fn run() can logging error after failed report saving; lib.rs

- report.rs:
    - fn b_to_gb & fn count_percent f32 to f64

### Removed
- unused argument mutation from partial report functions; report.rs

### Fixed
- solved clippy warning

## [0.1.1] - 2026-07-07

### Added
- implemented logging (env_logger, log)

## [0.1.0] - 2026-06-29

### Added
- first public version