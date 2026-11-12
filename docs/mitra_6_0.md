# Mitra 6.0 migration guide

## Preparation

If you're using Mitra 4.x, read the [Mitra 5.0 migration guide](./mitra_5_0.md). Updating from 4.x to 6.x is not supported.

## Dependencies

- Minimum supported version of Rust is 1.93.

## Configuration

- Removed `blocked_instances` and `allowed_instances` configuration options. Use `filter` commands to manage block/allowlists.

## CLI

- Removed `load-replies` command. Use `/api/v1/statuses/{status_id}/load_conversation` API endpoint or `ap import` command.
