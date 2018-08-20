# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2018-08-20

### Added
- Updated Flags to be public (and plural) so that it's accessible in the effects.
- Added data effects to the resources module. These are undocumented in the stellar resource online but appear in the all effects endpoint.
- Added missing memo field to the transaction resource.

### Fixed
- Removed source_amount from payment operations as it's not in use.
- Increased timeout to 60 to counter the long find-path endpoint
- Fixed some minor documentation error
- Restricted aggregation resolution to the defined set.

## [0.1.0] - 2018-04-20

### Added
- Synchronous client can be initialized and used.
- All resources can be deserialized from the horizon API.
- All endpoints are implemented.
