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

CONF_DIR="/etc/mitra"
TOML_CONF="$CONF_DIR/config.toml"
YAML_CONF="$CONF_DIR/config.yaml"

TOML_EXAMPLE="/usr/share/mitra/examples/config.example.toml"
YAML_EXAMPLE="/usr/share/mitra/examples/config.example.yaml"

# ---------------------------------------------------------------------------
# First-run config bootstrap
# ---------------------------------------------------------------------------
if [ -f "$TOML_CONF" ]; then
    echo "[entrypoint] Found existing TOML config at $TOML_CONF"
elif [ -f "$YAML_CONF" ]; then
    echo "[entrypoint] Found existing YAML config at $YAML_CONF"
else
    echo "[entrypoint] No configuration found. Initializing..."
    mkdir -p "$CONF_DIR"

    if [ -f "$TOML_EXAMPLE" ]; then
        echo "[entrypoint] Copying TOML example to $TOML_CONF"
        cp "$TOML_EXAMPLE" "$TOML_CONF"
    elif [ -f "$YAML_EXAMPLE" ]; then
        echo "[entrypoint] TOML example missing, copying YAML example to $YAML_CONF"
        cp "$YAML_EXAMPLE" "$YAML_CONF"
    else
        echo "[entrypoint] Error: No example configuration files found!"
        exit 1
    fi
fi

# ---------------------------------------------------------------------------
# Hand off to the application (or any command passed to `docker run`)
# exec replaces the shell process so the app receives OS signals directly.
# ---------------------------------------------------------------------------
exec "$@"
