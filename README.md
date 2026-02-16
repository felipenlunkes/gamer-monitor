# Gamer Monitor

<div align="justify">

## :uk: English

**Gamer Monitor** is a real-time performance monitor for Linux with a **GTK4** interface,
optimized for **ASUS** motherboards and **AMD Radeon RX**, **NVIDIA**, and **Intel Arc** GPUs.

### Features

- CPU monitoring (temperature and usage);
- GPU monitoring (temperatures and fan speed):
  - AMD Radeon RX: edge, hotspot and memory (via `amdgpu`);
  - NVIDIA: via `nvidia-smi`;
  - Intel Arc: sensors via kernel/`lm-sensors`;
- Multiple NVMe devices monitoring;
- RAM usage with progress bar;
- Modern GTK4 interface;
- Auto refresh every 5 seconds.

---

### Quick Start (recommended)

```bash
chmod +x build-install.sh
./build-install.sh
```

The script will:
- Detect your Linux distribution;
- Install dependencies;
- Install Rust (if needed);
- Build the project;
- Install the binary to `/usr/local/bin`;
- Create a desktop launcher;
- Configure sensors (optional).

---

### Dependencies (manual install)

#### Ubuntu / Debian

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libgtk-4-dev libglib2.0-dev libcairo2-dev libpango1.0-dev lm-sensors curl
```

#### Fedora

```bash
sudo dnf install -y @development-tools pkg-config gtk4-devel glib2-devel cairo-devel pango-devel lm-sensors curl
```

#### Arch Linux

```bash
sudo pacman -S --needed base-devel pkgconf gtk4 glib2 cairo pango lm_sensors curl
```

---

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

---

### Build

```bash
cd gamer-monitor
cargo build --release
```

Binary path:

```bash
target/release/gamer-monitor
```

---

### Run

```bash
./target/release/gamer-monitor
```

Install system-wide:

```bash
sudo cp target/release/gamer-monitor /usr/local/bin/
gamer-monitor
```

---

### Desktop launcher (.desktop)

Create `~/.local/share/applications/gamer-monitor.desktop`:

```desktop
[Desktop Entry]
Name=Gamer Monitor
Comment=Real-time system performance monitor
Exec=/usr/local/bin/gamer-monitor
Icon=utilities-system-monitor
Terminal=false
Type=Application
Categories=System;Monitor;
```

Update cache:

```bash
update-desktop-database ~/.local/share/applications
```

---

### Sensors setup

```bash
sudo sensors-detect --auto
```

#### ASUS motherboards

```bash
sudo modprobe nct6775
echo "nct6775" | sudo tee /etc/modules-load.d/sensors.conf
```

Verify:

```bash
sensors
```

---

### Troubleshooting

- **`sensors: command not found`**
  ```bash
  sudo apt install lm-sensors     # Ubuntu/Debian
  sudo dnf install lm-sensors     # Fedora
  sudo pacman -S lm_sensors       # Arch
  ```

- **“No sensors found”**
  ```bash
  sudo sensors-detect --auto
  sudo systemctl restart kmod
  ```

- **AMD GPU not detected**
  ```bash
  lspci -k | grep -A 3 VGA
  ```

---

### Uninstall

```bash
sudo rm /usr/local/bin/gamer-monitor
rm ~/.local/share/applications/gamer-monitor.desktop
update-desktop-database ~/.local/share/applications
```

---

### Contributing

Issues and pull requests are welcome!

### Tips

- Run `sensors` before launching the app;
- On ASUS boards, ensure `nct6775` is loaded;
- For NVIDIA, make sure `nvidia-smi` works;
- Multiple NVMe devices are detected automatically.
- 
## :brazil: Português

**Gamer Monitor** é um monitor de desempenho em tempo real para Linux com interface **GTK4**,
otimizado para placas-mãe **ASUS** e GPUs **AMD Radeon RX**, **NVIDIA** e **Intel Arc**.

### Recursos

- Monitoramento de CPU (temperatura e uso);
- Monitoramento de GPU (temperaturas e velocidade das ventoinhas):
  - AMD Radeon RX: edge, hotspot e memória (via `amdgpu`);
  - NVIDIA: via `nvidia-smi`;
  - Intel Arc: sensores via kernel/`lm-sensors`;
- Monitoramento de múltiplos dispositivos NVMe;
- Uso de RAM com barra de progresso;
- Interface moderna em GTK4;
- Atualização automática a cada 5 segundos.

---

### Guia de início rápido (recomendado)

```bash
chmod +x build-install.sh
./build-install.sh
```

O script irá:
- Detectar sua distribuição Linux;
- Instalar dependências;
- Instalar Rust (se necessário);
- Compilar o projeto;
- Instalar o binário em `/usr/local/bin`;
- Criar atalho no menu de aplicações;
- Configurar sensores (opcional).

---

### Dependências (instalação manual)

#### Ubuntu / Debian

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libgtk-4-dev libglib2.0-dev libcairo2-dev libpango1.0-dev lm-sensors curl
```

#### Fedora

```bash
sudo dnf install -y @development-tools pkg-config gtk4-devel glib2-devel cairo-devel pango-devel lm-sensors curl
```

#### Arch Linux

```bash
sudo pacman -S --needed base-devel pkgconf gtk4 glib2 cairo pango lm_sensors curl
```

---

### Instalação do Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

---

### Compilação

```bash
cd gamer-monitor
cargo build --release
```

Binário gerado em:

```bash
target/release/gamer-monitor
```

---

### Execução

```bash
./target/release/gamer-monitor
```

Instalar no sistema:

```bash
sudo cp target/release/gamer-monitor /usr/local/bin/
gamer-monitor
```

---

### Criar atalho no menu (.desktop)

Crie `~/.local/share/applications/gamer-monitor.desktop`:

```desktop
[Desktop Entry]
Name=Gamer Monitor
Comment=Monitor de desempenho em tempo real
Exec=/usr/local/bin/gamer-monitor
Icon=utilities-system-monitor
Terminal=false
Type=Application
Categories=System;Monitor;
```

Atualize o cache:

```bash
update-desktop-database ~/.local/share/applications
```

---

### Configuração dos Sensores

```bash
sudo sensors-detect --auto
```

#### Placas-mãe ASUS

```bash
sudo modprobe nct6775
echo "nct6775" | sudo tee /etc/modules-load.d/sensors.conf
```

Verifique:

```bash
sensors
```

---

### Troubleshooting

- **`sensors: command not found`**
  ```bash
  sudo apt install lm-sensors     # Ubuntu/Debian
  sudo dnf install lm-sensors     # Fedora
  sudo pacman -S lm_sensors       # Arch
  ```

- **“No sensors found”**
  ```bash
  sudo sensors-detect --auto
  sudo systemctl restart kmod
  ```

- **GPU AMD não aparece**
  ```bash
  lspci -k | grep -A 3 VGA
  ```

---

### Desinstalação

```bash
sudo rm /usr/local/bin/gamer-monitor
rm ~/.local/share/applications/gamer-monitor.desktop
update-desktop-database ~/.local/share/applications
```

---

### Contribuições

Contribuições são bem-vindas via issues e pull requests.

### Dicas

- Execute `sensors` antes de abrir o app;
- Em placas ASUS, confirme o módulo `nct6775`;
- Para NVIDIA, tenha o `nvidia-smi` funcionando;
- NVMe múltiplos são detectados automaticamente.

</div>