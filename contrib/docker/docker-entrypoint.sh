#!/bin/sh
# docker-entrypoint.sh
#
# Bootstraps the Mitra container on first start:
#   - Copies the example config when no config.yaml is present yet.
#   - Hands off to the main process via exec so signals are forwarded
#     correctly and the application remains PID 1.
#
# Usage (set by Dockerfile):
#   ENTRYPOINT ["docker-entrypoint.sh"]
#   CMD        ["mitra", "server"]

set -e

CONFIG_FILE=/etc/mitra/config.yaml
EXAMPLE_CONFIG=/usr/share/mitra/examples/config.example.yaml

# ---------------------------------------------------------------------------
# First-run config bootstrap
# ---------------------------------------------------------------------------
if [ ! -f "$CONFIG_FILE" ]; then
    echo "[entrypoint] $CONFIG_FILE not found – copying example config."
    echo "[entrypoint] Edit $CONFIG_FILE before restarting the container."
    cp "$EXAMPLE_CONFIG" "$CONFIG_FILE"
fi

# ---------------------------------------------------------------------------
# Hand off to the application (or any command passed to `docker run`)
# exec replaces the shell process so the app receives OS signals directly.
# ---------------------------------------------------------------------------
exec "$@"
