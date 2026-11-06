# Instrucciones para Claude Code — wi-retro

## Misión

Desarrollar wi-retro: una aplicación de escritorio multiplataforma para organizar bibliotecas de videojuegos y ejecutar juegos mediante una arquitectura de emulación híbrida. La experiencia de producto combina la organización visual de LaunchBox con la flexibilidad de RetroArch.

## Antes de modificar

1. Lee este archivo, `README.md`, `PROMPT.md` y los documentos de `docs/`.
2. Inspecciona el código real y el historial relevante antes de proponer cambios.
3. Identifica qué funcionalidades ya existen, cuáles están incompletas y cuáles son futuras.
4. No asumas que el roadmap equivale a funcionalidad implementada.
5. Presenta un plan breve para cambios amplios y divide el trabajo en fases verificables.

## Reglas no negociables

- **No reescribir por comodidad:** conserva la arquitectura y las funciones existentes salvo que exista una razón técnica demostrable.
- No elimines módulos, comandos, modelos, rutas, estilos ni flujos sin comprobar sus dependencias.
- No reemplaces una implementación real por mocks, simulaciones o datos ficticios.
- No declares que una característica funciona sin ejecutarla o probarla.
- Evita cambios masivos no relacionados con la tarea.
- Mantén compatibilidad con los archivos de guardado existentes; cualquier migración debe ser versionada y reversible.
- No agregues dependencias sin justificar su necesidad, mantenimiento, licencia y coste.
- No descargues, incluyas ni distribuyas ROMs comerciales.
- No incluyas binarios de cores o emuladores en Git.
- No ejecutes archivos importados por el usuario como scripts.
- No almacenes rutas privadas, tokens, claves ni datos personales en el repositorio.

## Arquitectura vigente

- Tauri 2 como shell de escritorio.
- React 19 + TypeScript + Vite en el frontend.
- Rust en el backend.
- SQLite para persistencia local.
- Libretro con carga dinámica de cores; mGBA es la integración inicial.
- El flujo actual de emulación GBA utiliza una ventana independiente.
- La biblioteca y el escáner deben permanecer desacoplados del motor de emulación.

La visión futura es híbrida:
1. Preferir Libretro cuando exista un core apropiado y mantenido.
2. Permitir emuladores independientes cuando aporten mejor compatibilidad, rendimiento o funciones.
3. Integrarlos mediante adaptadores con contratos comunes de detección, configuración, lanzamiento, estado y cierre.
4. Permitir selección automática y override manual por juego/sistema.

No implementes una abstracción genérica prematura: introduce interfaces cuando exista un segundo motor real que valide el contrato.

## Seguridad

- Canonicaliza rutas y valida que los archivos existan y sean regulares.
- Protege contra path traversal, enlaces simbólicos inesperados y rutas fuera de las carpetas autorizadas.
- Valida extensiones, tamaño, formato y hashes; no confíes en el nombre de archivo.
- Ejecuta procesos externos con argumentos estructurados, nunca concatenando comandos de shell con datos del usuario.
- Limita permisos Tauri al mínimo necesario por ventana.
- Revisa CSP y capabilities antes de habilitar APIs nuevas.
- Evita bloquear el runtime asíncrono con tareas de emulación o escaneo pesado.
- No registres rutas sensibles ni contenido de ROMs en logs.
- Trata metadatos externos como datos no confiables y evita HTML sin sanitizar.

## Calidad

- Rust: formato con `cargo fmt`, análisis con `cargo clippy` y pruebas con `cargo test`.
- Frontend: `npm run build` y pruebas disponibles en el proyecto.
- Añade pruebas para validación de rutas, importación, persistencia, selección de motor y errores.
- Maneja errores con mensajes accionables y sin filtrar información sensible.
- Actualiza documentación cuando cambien comandos, configuración, arquitectura o comportamiento.
- Revisa `git diff` y `git status` antes de finalizar.

## Flujo de trabajo

1. Resume el estado actual y el objetivo.
2. Define alcance y criterios de aceptación.
3. Implementa el cambio mínimo completo.
4. Ejecuta verificaciones pertinentes.
5. Corrige regresiones.
6. Actualiza documentación.
7. Informa archivos modificados, pruebas ejecutadas, resultados y limitaciones.

## Comandos útiles

```bash
npm install
npm run build
npm run tauri dev
npm run tauri build
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
```

Ejecuta los comandos Rust desde la raíz del repositorio. Si una dependencia del sistema impide una prueba, indícalo claramente en vez de reportarla como superada.
