# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2025-01-12

### Initial Release

- Forked from [opendeck-akp153](https://github.com/4ndv/opendeck-akp153) v0.9.4
- Adapted for VSD Inside M18 / Mirabox M18 device (5548:1000)
- 15 LCD keys (5 columns x 3 rows) + 3 bottom buttons (non-LCD)
- Protocol version 3 (1024-byte packets, supports press/release states)
- 64x64 JPEG images with 180° rotation and both-axis mirroring
- Vertical row flip for image output (device rows are inverted vs OpenDeck)
- Bottom buttons mapped to keys 16-18 in OpenDeck
- Updated DeviceNamespace from "99" to "18"
- Updated plugin identifiers for M18-specific plugin
