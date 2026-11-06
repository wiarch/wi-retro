# Registro de decisiones — wi-retro

Este documento registra decisiones de producto y arquitectura. Las propuestas futuras no implican que ya estén implementadas.

## ADR-001 — Aplicación de escritorio con Tauri
- **Estado:** aceptada.
- **Decisión:** utilizar Tauri 2 con React/TypeScript y backend Rust.
- **Motivo:** interfaz web moderna con lógica nativa y menor sobrecarga que soluciones basadas en Electron.

## ADR-002 — Libretro como primera opción
- **Estado:** aceptada.
- **Decisión:** priorizar Libretro cuando exista un core compatible.
- **Motivo:** API común y ecosistema de cores.

## ADR-003 — Arquitectura híbrida
- **Estado:** aceptada.
- **Decisión:** permitir motores independientes mediante adaptadores cuando aporten compatibilidad o rendimiento.
- **Motivo:** no limitar el catálogo a la cobertura de Libretro.
- **Consecuencia:** evitar una abstracción genérica prematura; validar el contrato con un segundo motor real.

## ADR-004 — ROMs administradas por el usuario
- **Estado:** aceptada.
- **Decisión:** importar y organizar archivos locales; integrar fuentes de juegos únicamente cuando su licencia autorice distribución.
- **Motivo:** respetar derechos de autor y mantener la responsabilidad de contenido clara.

## ADR-005 — Persistencia local SQLite
- **Estado:** aceptada.
- **Decisión:** mantener metadatos, bibliotecas y preferencias en almacenamiento local.
- **Motivo:** funcionamiento offline y control del usuario.
- **Nota:** preservar el esquema y los datos existentes mediante migraciones.

## ADR-006 — GBA como primera plataforma
- **Estado:** vigente.
- **Decisión:** usar GBA/mGBA como primer flujo real de emulación.
- **Motivo:** reducir alcance inicial y validar el pipeline de Libretro.
