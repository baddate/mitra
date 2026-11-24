#!/bin/sh
set -e

if [ -n "$VITE_BACKEND_URL" ]; then
    if grep -rq "__PLACEHOLDER_MITRA_BACKEND_URL__" /usr/share/mitra/www 2>/dev/null; then
        echo "Runtime env detected. Replacing frontend API URL with: $VITE_BACKEND_URL"
        find /usr/share/mitra/www -name "*.js" -exec sed -i "s|__PLACEHOLDER_MITRA_BACKEND_URL__|${VITE_BACKEND_URL}|g" {} +
    else
        echo "WARNING: placeholder not found — build baked in a different VITE_BACKEND_URL (check .env default: http://localhost:8383)" >&2
    fi
else
    echo "No VITE_BACKEND_URL provided at runtime, keeping build-time configuration."
fi

exec "$@"
