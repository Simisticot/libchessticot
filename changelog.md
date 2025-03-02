# Changelog
Notable changes to this project might be documented in this file.

## [0.2.1] - 2025-03-02

### Added

- "Planner" engine that searches a few moves ahead using negamax with alpha/beta pruning.
- Method to serialize a position to FEN.
- Benchmarking for search.

### Changed

- Reimplemented attack detection for improved performance

### Fixed

- Castleing is no longer allowed while in check

