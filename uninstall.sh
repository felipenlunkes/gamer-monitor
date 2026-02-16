#!/bin/bash

# Gamer Monitor - Installation Script
# Copyright (C) 2025-2026 Felipe Miguel Nery Lunkes

set -e

APP_NAME="gamer-monitor"
INSTALL_DIR="/usr/local/bin"
DESKTOP_FILE="/usr/share/applications/${APP_NAME}.desktop"

echo "Uninstalling Gamer Monitor..."
if [ -f "${INSTALL_DIR}/${APP_NAME}" ]; then
  sudo rm -f "${INSTALL_DIR}/${APP_NAME}"
else
  echo "ℹ Gamer Monitor not found!"
fi

echo "Removing Gamer Monitor entry in applications..."
if [ -f "${DESKTOP_FILE}" ]; then
  sudo rm -f "${DESKTOP_FILE}"
  sudo update-desktop-database >/dev/null 2>&1 || true
else
  echo "ℹ Entry not found"
fi

echo "Gamer Monitor uninstalled successfully!"
