# Gamer Monitor

<div align="justify">

<details title="English" align='left'>
<summary align='left'>:uk: English</summary>
<br>

**Gamer Monitor** is a real-time performance monitor for Linux built in **Rust** using **GTK4**.
The goal of this project is to provide a simple, lightweight, and intuitive tool that displays device resource usage,
such as workload and component temperatures. Gamer Monitor aggregates usage and telemetry data from the CPU, RAM,
storage (NVMe only), and GPU. It is optimized for AMD Radeon GPUs, with support for NVIDIA GPUs as well. In the future,
more detailed information for Intel Arc GPUs will also be available.

## Features

Below is a list of Gamer Monitor features and capabilities:

- CPU monitoring (temperature and usage);
- GPU monitoring (temperatures and fan speeds):
  - AMD Radeon RX: edge, hotspot, and memory (via amdgpu);
  - NVIDIA: via nvidia-smi;
  - Intel Arc: basic information via the kernel/lm-sensors. More advanced support is planned for the future;
- Monitoring of multiple NVMe devices;
- RAM usage with a progress bar;
- Modern GTK4 interface.

## Sensor setup

You need to configure `lm-sensors` to detect your hardware. To do so, run:

```bash
sudo sensors-detect --auto
```

## ASUS motherboards (optional)

> This configuration step is optional for users with ASUS motherboards. If your motherboard is from another vendor, you can skip this step.

ASUS motherboards usually use a specific module that reports various information, such as fan speeds, power states, and temperatures. If supported by your motherboard (check your board’s documentation), you can load the module below:

```bash
sudo modprobe nct6775
echo "nct6775" | sudo tee /etc/modules-load.d/sensors.conf
```

Then verify whether the motherboard data is being reported:

```bash
sensors
```

## Simplified installation (recommended)

You can install Gamer Monitor **without setting up a development environment**. A precompiled binary and an installation script are available, which also creates a shortcut in your application menu.

> Note that Gamer Monitor depends on the `lm-sensors` package being installed on your system. Follow the steps below to install lm-sensors:

On Debian/Ubuntu-based systems, run:

```bash
sudo apt install -y lm-sensors
```

On Fedora:

```bash
sudo dnf install -y lm-sensors
```

On Arch or derivatives:

```bash
sudo pacman -S --needed --noconfirm lm_sensors
```

> Done! Now you have installed the required dependency and can install and run Gamer Monitor.

To install Gamer Monitor using the simplified method, run:

```bash
sudo ./install.sh
```

> Done! Now run `gamer-monitor` from the terminal or use the shortcut created in your application menu.
> If you use the script, you don’t need to follow the dependency installation, build, and manual installation steps below.

## Quick start guide – build and install

You can also build and install Gamer Monitor yourself. A script is provided to set up the local environment,
compile the application, and install it, creating an application menu shortcut.

> This script will also install all dependencies required to build and run Gamer Monitor.

To install dependencies, configure the environment, build, and install, run:

```bash
chmod +x build-install.sh
./build-install.sh
```


The script will:
- Detect your Linux distribution;
- Install the required build and runtime dependencies;
- Install Rust (if needed);
- Build the project;
- Install the binary to /usr/local/bin;
- Create an application menu shortcut;
- Configure sensors (optional).

### Dependencies (manual installation)

You can also install the build dependencies manually, according to your distribution:

For Ubuntu/Debian:

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libgtk-4-dev libglib2.0-dev libcairo2-dev libpango1.0-dev lm-sensors curl
```

For Fedora:

```bash
sudo dnf install -y @development-tools pkg-config gtk4-devel glib2-devel cairo-devel pango-devel lm-sensors curl
```

For Arch Linux:

```bash
sudo pacman -S --needed base-devel pkgconf gtk4 glib2 cairo pango lm_sensors curl
```

### Rust installation (manual)

You need a local Rust development environment. The application has the following requirements:

* Rust edition 2021 or newer;
* rustc version 1.80.0 or newer.

To install Rust, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Build (manual)

To build the application and generate the binary, run in the project root:

```bash
cargo build --release
```

The final binary will be located at:

```bash
target/release/gamer-monitor
```

### Run (manual)

After building and before installing, you can run the application with:

```bash
./target/release/gamer-monitor
```

### Manual installation

To install the application system-wide, copy the generated binary to `/usr/local/bin`:

```bash
sudo cp target/release/gamer-monitor /usr/local/bin/
gamer-monitor
```

### Create application menu shortcut (.desktop)

Manually create an application entry at:
`~/.local/share/applications/gamer-monitor.desktop` with the following content:

```desktop
[Desktop Entry]
Name=Gamer Monitor
Comment=Real-time performance monitor
Exec=/usr/local/bin/gamer-monitor
Icon=utilities-system-monitor
Terminal=false
Type=Application
Categories=System;Monitor;
```

Then update the application database:

```bash
update-desktop-database ~/.local/share/applications
```

## Troubleshooting

Below is a list of reported issues and commands to fix them:

- **`sensors: command not found`**
  ```bash
  sudo apt install lm-sensors     # Ubuntu/Debian
  sudo dnf install lm-sensors     # Fedora
  sudo pacman -S lm_sensors       # Arch
  ```

- **`No sensors found`**
  ```bash
  sudo sensors-detect --auto
  sudo systemctl restart kmod
  ```

- **`AMD GPU not detected`**
  ```bash
  lspci -k | grep -A 3 VGA
  ```

## Uninstallation

You can uninstall the application manually or using the uninstall script.

Using the uninstall script:

```bash
sudo ./uninstall.sh
```

For manual uninstallation, use:

```bash
sudo rm /usr/local/bin/gamer-monitor
rm ~/.local/share/applications/gamer-monitor.desktop
update-desktop-database ~/.local/share/applications
```

## Contributing

Contributions are welcome via issues and pull requests!

## Tips

- On ASUS boards, confirm the nct6775 module is loaded;
- For NVIDIA GPUs, make sure nvidia-smi is working;
- Multiple NVMe devices are detected automatically.

<hr>

</details>

<details title="Português" align='left'>
<summary align='left'>:brazil: Português</summary>
<br>

**Gamer Monitor** é um monitor de desempenho em tempo real para Linux construído em **Rust** utilizando **GTK4**.
O objetivo deste projeto é oferecer uma ferramenta simples, leve e intuitiva que exibe a utilização dos recursos
do dispositivo, como carga de trabalho e temperatura dos componentes. O Gamer Monitor agrega informações e dados
de uso do processaor, memória RAM, armazenamento (apenas NVMe) e placa de vídeo. Ele é otimizado para placas AMD
Radeon, mas há suporte para placas da NVIDIA. Futuramente, informações mais detalhadas de placas Intel Arc também
estarão disponíveis.

## Recursos

Abaixo, uma lista dos recursos e funcionalidades do Gamer Monitor:

- Monitoramento de CPU (temperatura e uso);
- Monitoramento de GPU (temperaturas e velocidade das ventoinhas):
  - AMD Radeon RX: edge, hotspot e memória (via `amdgpu`);
  - NVIDIA: via `nvidia-smi`;
  - Intel Arc: informações básicas via kernel/`lm-sensors`. No futuro, haverá um suporte maior;
- Monitoramento de múltiplos dispositivos NVMe;
- Uso de memória RAM com barra de progresso;
- Interface moderna em GTK4.

## Configuração dos sensores

Você precisa configurar o `lm-sensors` para identificar o seu hardware. Para isso, utilize:

```bash
sudo sensors-detect --auto
```

## Placas-mãe ASUS (opcional)

> Essa etapa de configuração é opcional para usuários com placas da ASUS. Se você tem uma placa de outros fabricantes,
desconsidere essa etapa.

Placas-mãe da ASUS costumam utilizar um módulo específico que reporta diversas informações, como velocidade das fans, estado
energético e temperaturas. Caso seja compatível (leia a documentação da sua placa), você pode carregar o módulo à seguir:

```bash
sudo modprobe nct6775
echo "nct6775" | sudo tee /etc/modules-load.d/sensors.conf
```

Verifique depois se os dados da placa-mãe foram reportados.

```bash
sensors
```

## Instalação simplficiada (recomendado)

Você pode instalar o Gamer Monitor **sem precisar configurar um ambiente de desenvolvimento**. Para isso, existe
um binário pré-compilado e um script de instalação, que também cria um atalho na sua lista de aplicativos.

> Vale ressaltar que o Gamer Monitor depende do pacote `lm-sensors` instalado em seu computador. Siga os passos abaixo
para instalar o `lm-sensors`:

Em sistemas Debian/Ubuntu ou derivados, use, no terminal:

```bash
sudo apt install -t lm-sensors
```

Para Fedora, use:

```bash
sudo dnf install -y lm-sensors
```

Para Arch ou derivados, utilize:

```bash
sudo pacman -S --needed --noconfirm lm_sensors
```

> Pronto! Agora você instalou a dependência necessária e pode instalar e executar o Game Monitor.

Para instalar o Game Monitor de forma simplificada, use, no seu terminal:

```bash
sudo ./install.sh
```

> Pronto! Agora execute, pelo terminal, `game-monitor` ou utilize o atalho criado na lista de aplicativos.
> Se você utilizar o script, não precisa realizar as etapas de instalação de dependências, compilação e instalação
da aplicação descritas abaixo.

## Guia de início rápido - compilação e instalação

Voc6e também pode compilar e instalar o Game Monitor você mesmo. Para isso, existe um script que fará o setup
local do ambiente necessário, compilará a aplicação e a instalará, criando o atalho na sua lista de aplicativos.

> Este script também irá instalar todas as dependências necessárias para a compilação e execução do Game Monitor.

Para instalar as dependências, configurar o ambiente, compilar e instalar, utilize, no seu terminal:

```bash
chmod +x build-install.sh
./build-install.sh
```

O script irá:
- Detectar sua distribuição Linux;
- Instalar as dependências necessárias para a compilação e execução;
- Instalar Rust (se necessário);
- Compilar o projeto;
- Instalar o binário em `/usr/local/bin`;
- Criar atalho no menu de aplicações;
- Configurar sensores (opcional).

### Dependências (instalação manual)

Você também pode instalar as dependências para a compilação, de acordo com a sua distribuição:

Para Ubuntu/Debian, use:

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libgtk-4-dev libglib2.0-dev libcairo2-dev libpango1.0-dev lm-sensors curl
```

Para Fedora, use:

```bash
sudo dnf install -y @development-tools pkg-config gtk4-devel glib2-devel cairo-devel pango-devel lm-sensors curl
```

Para Arch Linux, use:

```bash
sudo pacman -S --needed base-devel pkgconf gtk4 glib2 cairo pango lm_sensors curl
```

### Instalação do Rust (instalação manual)

Você precisa ter o ambiente de desenvolvimento do Rust localmente. A aplicação tem os seguintes requisitos:

* Rust edition 2021 ou mais recente;
* rustc versão 1.80.0 ou mais recente.

Para instalar o ambiente, utilize, em seu terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Compilação (instalação manual)

Para compilar a aplicação e gerar o binário, utilize, na raiz do projeto:

```bash
cargo build --release
```

O binário final estará licalizado em:

```bash
target/release/gamer-monitor
```

### Execução (instalação manual)

Após a compilação e antes da instalação, você pode executar a aplicação, utilizando, no terminal:

```bash
./target/release/gamer-monitor
```

### Instalação manual

Para instalar a aplicação no seu sistema, você deve copiar o binário gerado para `/usr/local/bin`, utilizando:

```bash
sudo cp target/release/gamer-monitor /usr/local/bin/
gamer-monitor
```

### Criar atalho no menu (.desktop)

Você precisa criar, manualmente, uma entrada (atalho) na lista de aplicativos, no seguinte
caminho: `~/.local/share/applications/gamer-monitor.desktop`, com o conteúdo:

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

A seguir, você precisa atualizar o cache:

```bash
update-desktop-database ~/.local/share/applications
```

## Troubleshooting

Agora, uma lista de problemas relatados e comandos para resolver o problema.

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

## Desinstalação

Você pode desinstalar a aplicação (removendo o atalho) de forma manual ou com o script de desinstalação.

Se for utilizar o script de desinstalação, utilize, no terminal:

```bash
sudo ./uninstall.sh
```

Para desinstalação manual, utilize:

```bash
sudo rm /usr/local/bin/gamer-monitor
rm ~/.local/share/applications/gamer-monitor.desktop
update-desktop-database ~/.local/share/applications
```

## Contribuições

Contribuições são bem-vindas via issues e pull requests.

## Dicas

- Em placas ASUS, confirme o módulo `nct6775`;
- Para NVIDIA, tenha o `nvidia-smi` funcionando;
- NVMe múltiplos são detectados automaticamente.

</details>

</div>