# wi-retro

**Biblioteca y centro de emulación de videojuegos para escritorio.**

wi-retro busca combinar la experiencia de organización de bibliotecas de LaunchBox con la flexibilidad de emulación de RetroArch. El objetivo es ofrecer una aplicación de escritorio moderna para descubrir, organizar y ejecutar juegos que el usuario posee o está autorizado a utilizar.

> **Estado:** desarrollo temprano. La implementación actual está centrada en Game Boy Advance (GBA) mediante Libretro/mGBA. Las funciones descritas como futuras no deben interpretarse como ya disponibles.

## Visión del producto

- Biblioteca unificada de juegos, sistemas y colecciones.
- Importación y escaneo de ROMs desde carpetas locales y unidades externas.
- Identificación de juegos mediante metadatos, hashes y material gráfico.
- Emulación híbrida: Libretro como primera opción y emuladores independientes mediante adaptadores.
- Perfiles de controles, configuración por sistema y por juego.
- Guardados, estados, capturas, progreso y estadísticas.
- Interfaz personalizable, modo ventana y pantalla completa.
- Integración con fuentes legales de metadatos y juegos de libre distribución.

wi-retro no distribuirá ROMs comerciales ni facilitará fuentes no autorizadas. El usuario es responsable de contar con los derechos necesarios sobre los archivos que importe.

## Estado actual

| Área | Estado |
|---|---|
| Aplicación de escritorio | Base Tauri 2 + React |
| Backend nativo | Rust |
| Biblioteca GBA | Escaneo e indexación inicial |
| Persistencia | SQLite/local implementada en el código actual |
| Emulación Libretro | Integración inicial con core mGBA |
| Ventana independiente GBA | Implementada |
| Otros sistemas y cores | Pendiente |
| Emuladores independientes | Arquitectura futura |
| Metadatos y artwork | Pendiente |
| Colecciones y perfiles avanzados | Pendiente |

Consulta [ROADMAP.md](docs/ROADMAP.md) para conocer las fases previstas y sus criterios de finalización.

## Stack tecnológico

- **Desktop:** Tauri 2
- **Frontend:** React 19, TypeScript, Vite
- **Backend:** Rust
- **Emulación:** Libretro mediante carga dinámica de cores
- **Persistencia:** SQLite
- **Estilos:** CSS
- **Herramientas:** Node.js LTS, npm y Rust stable

## Estructura del proyecto

```text
wi-retro/
├── .github/                 # Plantillas y automatización GitHub
├── docs/                    # Producto, arquitectura, decisiones y roadmap
├── scripts/                 # Scripts auxiliares
├── src/                     # Interfaz React + TypeScript
│   ├── components/
│   ├── lib/
│   ├── styles/
│   └── types/
├── src-tauri/               # Backend Rust y configuración Tauri
│   ├── capabilities/
│   ├── cores/               # Cores locales (no versionar binarios)
│   └── src/
│       ├── commands/
│       └── libretro/
├── CLAUDE.md                # Instrucciones para Claude Code
├── PROMPT.md                # Plan maestro de implementación
├── package.json
└── vite.config.ts
```

## Requisitos

- Node.js LTS y npm.
- Rust stable y Cargo.
- Dependencias nativas de Tauri 2 para el sistema operativo.
- Un core Libretro compatible para ejecutar juegos GBA.

### Linux: Arch Linux / CachyOS

```bash
sudo pacman -S --needed base-devel curl wget openssl pkg-config \
  webkit2gtk-4.1 gtk3 libappindicator-gtk3 librsvg
```

En otras distribuciones, instala los paquetes de desarrollo requeridos por Tauri 2 y WebKitGTK.

### Windows

Instala Microsoft C++ Build Tools y Microsoft Edge WebView2 Runtime, además de Node.js LTS y Rust stable.

## Instalación y ejecución

```bash
git clone https://github.com/wiarch/wi-retro.git
cd wi-retro
npm install
npm run tauri dev
```

Para compilar:

```bash
npm run build
npm run tauri build
```

Para instalar el core mGBA en Linux, revisa primero las instrucciones y el origen del binario en [src-tauri/cores/README.md](src-tauri/cores/README.md). El script disponible es:

```bash
bash scripts/install-mgba-core.sh
```

No ejecutes scripts de descarga sin revisar su contenido y verificar que el binario provenga de una fuente confiable.

## Comandos de desarrollo

| Comando | Descripción |
|---|---|
| `npm run dev` | Inicia Vite |
| `npm run build` | Comprueba TypeScript y genera el frontend |
| `npm run preview` | Previsualiza el build web |
| `npm run tauri dev` | Ejecuta la aplicación de escritorio |
| `npm run tauri build` | Genera los instaladores |
| `npm run tauri:dev` | Ejecución con ajustes de compatibilidad Linux |
| `npm run install:core` | Ejecuta el instalador del core mGBA |

## Documentación para desarrollo asistido

- [CLAUDE.md](CLAUDE.md): reglas permanentes para Claude Code.
- [PROMPT.md](PROMPT.md): plan maestro para continuar el producto.
- [Producto y alcance](docs/PRODUCT.md).
- [Arquitectura](docs/ARCHITECTURE.md).
- [Roadmap](docs/ROADMAP.md).
- [Decisiones técnicas](docs/DECISIONS.md).
- [Contribución](CONTRIBUTING.md).
- [Seguridad](SECURITY.md).

## Principios del proyecto

1. Preservar el código y las funciones existentes.
2. Separar la interfaz, la lógica de biblioteca y los motores de emulación.
3. Mantener Libretro como primera opción sin bloquear integraciones independientes.
4. No ejecutar ROMs, cores ni emuladores sin validar rutas y entradas.
5. Priorizar estabilidad, rendimiento, trazabilidad y experiencia de usuario.
6. No declarar una función terminada hasta que esté implementada y probada.

## Licencia

No se ha establecido todavía una licencia de distribución para el proyecto. El código y los recursos permanecen sujetos a los derechos de sus respectivos titulares mientras se define una licencia explícita.

---

Parte del ecosistema **[WI Arch](https://github.com/wiarch)**.
