# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.3](https://github.com/maciejpedzich/f1-game-packet-parser/compare/v1.0.2...v1.0.3) - 2025-03-16

### Other

- *(deps)* bump the patch-updates group with 3 updates

## [1.0.0](https://github.com/maciejpedzich/f1-game-packet-parser/releases/tag/v1.0.0) - 2025-02-28

### Added

- add missing 2023 spec red flag event
- add F2 2024 driver and team IDs
- derive Hash trait for all constant enums
- add 2024 driver and team IDs
- add 2024 spec parser fields and constants
- add 2023 spec parser fields and constants
- create a basic UDP client that reports parsing errors
- add 2022 spec session history packet parser
- add 2022 spec car damage packet parser
- add 2022 spec lobby packet parser
- add 2022 spec final classification packet parser
- add 2022 spec car status packet parser
- add 2022 spec car telemetry packet parser
- add 2022 spec car setups packet parser
- add 2022 spec participants packet parser
- add 2022 spec event event packet parser
- add 2022 spec lap data packet parser
- add 2022 spec session packet parser
- add 2022 spec motion packet parser

### Fixed

- add missing engine braking percentage field from 2024 spec
- replace incorrect constant for lap history entry's raw size with inline function
- remove incorrect assert for PENA event's other_vehicle_index value range
- add missing 2022 spec fields and constants
- add missing fields in session and event packets

### Other

- exclude files inside the .github and .idea directories
- add CI/CD GitHub workflows and issue templates
- add homepage link to package manifest
- add extra crate metadata and bump version to 1.0.0
- add installation instructions, complex example and FAQs
- convert parse function's arg type to generic implementing AsRef<[u8]>
- add num_tyre_stint doc links to tyre-related vectors
- fix or allow certain pedantic warning from clippy
- add integration tests for the parse function
- add missing 2023 stop-go and drive-through served event packet dumps
- add 2024 spec packet dumps
- add binrw asserts for all index fields in event packets
- increase the buffer size in example UDP client
- add missing 2022 spec lobby packet dump
- add 2023 spec packet dumps
- create tests directory and 2022 spec packet dumps
- repurpose udpclient to create raw packet dump files
- bump example UDP client's buffer size to 1464 bytes
- add cargo-deny config file
- add frontpage doc comments and tweak various existing ones
- use from_bits_retain instead of truncate when reading bitmap fields
- add MIT license field to Cargo.toml
- Create LICENSE
- add links to driver and team ID constants submodules in various struct fields
- replace SessionType enum with session_type submodule of constants
- convert inline code block field names to proper field links
- bump serde to 1.0.218
- print UDP client listening message after successful socket bind
- reorganise field asserts and use "num_" fields with pad_after for reading variable-sized collections
- set rustfmt print width to 90
- change u8_to_bool return type to Result with custom error
- use from_bits_truncate directly when reading bitmaps
- add missing non_exhaustive marker to LapData struct
- add binrw assertions for session-related structs
- add packet_format binrw import to CarMotionData struct
- remove redundant bitflags crate
- Cargo project init
