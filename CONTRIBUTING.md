# Guía de contribución

Gracias por contribuir a wi-retro. Este proyecto prioriza estabilidad, seguridad y compatibilidad con bibliotecas locales.

## Antes de empezar

1. Lee `README.md`, `CLAUDE.md` y la documentación de `docs/`.
2. Revisa issues abiertos y evita duplicar trabajo.
3. Para cambios grandes, abre primero una propuesta de alcance.
4. No incluyas ROMs comerciales, BIOS propietarios, cores compilados ni datos personales.

## Entorno

Instala Node.js LTS, npm, Rust stable y las dependencias nativas de Tauri 2 para tu sistema.

```bash
npm install
npm run build
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
```

## Estándares

- Mantén TypeScript estricto y tipos compartidos claros.
- Sigue convenciones idiomáticas de Rust.
- Prefiere cambios pequeños y enfocados.
- Añade pruebas para errores, rutas, persistencia y flujos críticos.
- Evita dependencias innecesarias.
- Actualiza documentación y ejemplos cuando cambie el comportamiento.
- No ocultes advertencias o fallos de pruebas.

## Pull requests

Incluye:
- Problema que resuelve y contexto.
- Solución y decisiones relevantes.
- Pruebas ejecutadas con resultados.
- Capturas para cambios visuales.
- Migraciones, riesgos y pasos manuales si aplican.

## Seguridad y licencias

Reporta vulnerabilidades de forma privada según `SECURITY.md`. Verifica las licencias de dependencias, cores y recursos multimedia antes de integrarlos.
