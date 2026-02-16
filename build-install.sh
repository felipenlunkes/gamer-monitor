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

# Detect distro
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
else
    echo -e "${RED}Cannot detect Linux distribution${NC}"
    exit 1
fi

echo -e "${YELLOW}Detected distribution: $DISTRO${NC}\n"

# Install dependencies
echo -e "${CYAN}[1/5] Installing system dependencies...${NC}"

case $DISTRO in
    ubuntu|debian|pop|linuxmint)
        sudo apt update
        sudo apt install -y build-essential pkg-config libgtk-4-dev libglib2.0-dev libcairo2-dev libpango1.0-dev lm-sensors curl
        ;;
    fedora)
        sudo dnf install -y @development-tools pkg-config gtk4-devel glib2-devel cairo-devel pango-devel lm-sensors curl
        ;;
    arch|manjaro)
        sudo pacman -S --needed --noconfirm base-devel pkgconf gtk4 glib2 cairo pango lm_sensors curl

        ;;
    *)
        echo -e "${YELLOW}Unknown distribution. Please install manually:${NC}"
        echo "  - build-essential / gcc / base-devel"
        echo "  - gtk4-dev / gtk4-devel / gtk4"
        echo "  - lm-sensors"
        echo "  - curl"
        read -p "Press Enter to continue if you have installed these packages..."
        ;;
esac

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo -e "${CYAN}[2/5] Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo -e "${GREEN}[2/5] Rust is already installed${NC}"
fi

# Build project
echo -e "${CYAN}[3/5] Building project...${NC}"
cargo build --release

# Install binary
echo -e "${CYAN}[4/5] Installing binary...${NC}"
sudo cp target/release/gamer-monitor /usr/local/bin/
sudo chmod +x /usr/local/bin/gamer-monitor

# Create desktop entry
echo -e "${CYAN}[5/5] Creating desktop entry...${NC}"
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

update-desktop-database ~/.local/share/applications 2>/dev/null || true

# Configure sensors
echo -e "\n${CYAN}========================================${NC}"
echo -e "${YELLOW}Sensor configuration${NC}"
echo -e "${CYAN}========================================${NC}\n"

if [ ! -f /etc/modules-load.d/sensors.conf ]; then
    echo -e "${YELLOW}Do you want to configure lm-sensors now? (recommended)${NC}"
    read -p "Run sensors-detect? [Y/n] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
        sudo sensors-detect --auto
        echo -e "${GREEN}Sensors configured!${NC}"
    fi
fi

# For ASUS motherboards
echo -e "\n${YELLOW}Do you have an ASUS motherboard?${NC}"
read -p "Load nct6775 module? [y/N] " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    sudo modprobe nct6775
    echo "nct6775" | sudo tee -a /etc/modules-load.d/sensors.conf > /dev/null
    echo -e "${GREEN}nct6775 module loaded and configured!${NC}"
fi

echo -e "\n${GREEN}========================================${NC}"
echo -e "${GREEN}  Installation completed successfully!${NC}"
echo -e "${GREEN}========================================${NC}\n"
echo -e "${CYAN}You can now run the application:${NC}"
echo -e "  ${YELLOW}gamer-monitor${NC}"
echo -e "\nOr find it in your application menu: ${YELLOW}Gamer Monitor${NC}\n"
