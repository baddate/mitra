#!/bin/sh
set -e

CONF_DIR="/etc/mitra"

TOML_CONF="$CONF_DIR/config.toml"
YAML_CONF="$CONF_DIR/config.yaml"

TOML_EXAMPLE="/usr/share/mitra/examples/config.example.toml"
YAML_EXAMPLE="/usr/share/mitra/examples/config.example.yaml"

# ---------------------------------------------------------------------------
# 1. Resolve config file (priority: ENV > existing > bootstrap)
# ---------------------------------------------------------------------------

if [ -n "$CONFIG_PATH" ]; then
    CONFIG_FILE="$CONFIG_PATH"
    echo "[entrypoint] Using config from ENV: $CONFIG_FILE"

elif [ -f "$TOML_CONF" ]; then
    CONFIG_FILE="$TOML_CONF"
    echo "[entrypoint] Found TOML config"

elif [ -f "$YAML_CONF" ]; then
    CONFIG_FILE="$YAML_CONF"
    echo "[entrypoint] Found YAML config"

else
    echo "[entrypoint] No config found, bootstrapping..."
    mkdir -p "$CONF_DIR"

    if [ -f "$TOML_EXAMPLE" ]; then
        cp "$TOML_EXAMPLE" "$TOML_CONF"
        CONFIG_FILE="$TOML_CONF"
        echo "[entrypoint] Initialized TOML config"

    elif [ -f "$YAML_EXAMPLE" ]; then
        cp "$YAML_EXAMPLE" "$YAML_CONF"
        CONFIG_FILE="$YAML_CONF"
        echo "[entrypoint] Initialized YAML config"

    else
        echo "[entrypoint] ERROR: no example config found"
        exit 1
    fi
fi

if [ ! -f "$CONFIG_FILE" ]; then
    echo "[entrypoint] $CONFIG_FILE not found - copying example config."
    echo "[entrypoint] Edit $CONFIG_FILE before restarting the container."
    cp "$EXAMPLE_CONFIG" "$CONFIG_FILE"
fi

echo "[entrypoint] Final config: $CONFIG_FILE"

# ---------------------------------------------------------------------------
# Hand off to the application (or any command passed to `docker run`)
# exec replaces the shell process so the app receives OS signals directly.
# ---------------------------------------------------------------------------
exec "$@"
