#!/bin/bash

# Gamer Monitor - Installation Script
# Copyright (C) 2025-2026 Felipe Miguel Nery Lunkes

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}  Gamer Monitor - Installer${NC}"
echo -e "${CYAN}========================================${NC}\n"

echo "Installing Gamer Monitor on your system..."
echo
echo -e "${CYAN}[1/2] Installing binary...${NC}"
sudo cp gamer-monitor /usr/local/bin/
sudo chmod +x /usr/local/bin/gamer-monitor

# Create desktop entry
echo -e "${CYAN}[2/2] Creating desktop entry...${NC}"
mkdir -p ~/.local/share/applications

cat > ~/.local/share/applications/gamer-monitor.desktop << 'EOF'
[Desktop Entry]
Name=Gamer Monitor
Comment=Real-time system performance monitor
Exec=/usr/local/bin/gamer-monitor
Icon=utilities-system-monitor
Terminal=false
Type=Application
Categories=System;Monitor;
EOF