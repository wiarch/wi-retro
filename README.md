# wi-retro

Gestor de emulación multiplataforma (Windows / Linux) — MVP GBA con backend Rust y frontend React.

## Estructura

```
wi-retro/
├── src/                      # Frontend React + TypeScript
│   ├── lib/                  # Invocaciones Tauri
│   ├── styles/               # CSS oscuro global
│   ├── types/                # Tipos compartidos con backend
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/                # Backend Rust
│   ├── capabilities/         # Permisos Tauri 2
│   ├── icons/
│   ├── src/
│   │   ├── commands/         # Comandos expuestos al frontend
│   │   ├── gba.rs            # Detección ROM GBA
│   │   ├── libretro.rs       # Stub cores Libretro
│   │   ├── models.rs
│   │   ├── scanner.rs        # Indexación paralela (rayon + walkdir)
│   │   ├── state.rs          # Estado en memoria
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── index.html
├── package.json
├── tsconfig.json
└── vite.config.ts
```

## Inicializar entorno (ejecutar en orden)

### 1. Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup default stable
```

### 2. Dependencias del sistema

**Arch / CachyOS (Linux):**

```bash
sudo pacman -S --needed base-devel curl wget openssl pkg-config \
  webkit2gtk-4.1 gtk3 libappindicator-gtk3 librsvg
```

**Debian / Ubuntu:**

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

**Windows:**

- [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (incluido en Windows 11)

### 3. Node.js (LTS recomendado)

```bash
# Ejemplo con fnm
curl -fsSL https://fnm.vercel.app/install | bash
fnm install --lts
fnm use lts-latest
```

### 4. Dependencias del proyecto

```bash
cd /home/wydex/Code/WiArch/wi-retro
npm install
```

### 5. Desarrollo

```bash
npm run tauri dev
```

### 6. Build release

```bash
npm run tauri build
```

## Comandos Tauri expuestos

| Comando | Descripción |
|---------|-------------|
| `scan_library` | Escaneo paralelo de ROMs GBA |
| `get_indexed_roms` | Devuelve índice en memoria |
| `get_library_config` | Ruta raíz configurada |
| `set_library_root` | Define carpeta de biblioteca |

## Rendimiento

- Indexación en Rust con `walkdir` + `rayon` (paralelo por archivo).
- Estado en memoria con `parking_lot::RwLock` (sin DB en MVP).
- WebView nativo del SO vía Tauri 2 (bajo consumo RAM vs Electron).
- Release profile: LTO + `opt-level = "s"`.

## Próximos pasos

- Cargar cores Libretro dinámicamente (`libretro.rs`).
- Lanzar emulación embebida o externa.
- Persistencia SQLite del índice.
- Metadatos (carátulas, títulos) vía header GBA.
