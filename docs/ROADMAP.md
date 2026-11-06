# Roadmap — wi-retro

Este roadmap expresa objetivos, no funcionalidades ya disponibles. La implementación actual está centrada en GBA.

## Fase 0 — Auditoría y estabilidad
- [ ] Documentar flujos actuales y dependencias.
- [ ] Verificar build frontend y Tauri.
- [ ] Verificar formato, clippy y pruebas Rust.
- [ ] Revisar seguridad de rutas, capabilities y ejecución de cores.
- [ ] Resolver defectos críticos.

**Salida:** estado reproducible y deuda técnica priorizada.

## Fase 1 — Biblioteca local
- [ ] Escaneo incremental y cancelable.
- [ ] Soporte de múltiples plataformas mediante perfiles de extensiones.
- [ ] Hashes y detección de duplicados.
- [ ] Importación de carpetas y unidades externas.
- [ ] Estados de archivos ausentes y reubicados.
- [ ] Migraciones SQLite versionadas.

**Salida:** biblioteca fiable y persistente.

## Fase 2 — Catálogo y metadatos
- [ ] Carátulas, capturas y detalles.
- [ ] Búsqueda, filtros y ordenamiento.
- [ ] Favoritos y colecciones.
- [ ] Adaptadores de proveedores de metadatos.
- [ ] Caché y edición manual.

**Salida:** catálogo visual útil offline.

## Fase 3 — Emulación híbrida
- [ ] Consolidar gestión de cores Libretro.
- [ ] Detección de cores y capacidades.
- [ ] Definir contrato con una segunda implementación real.
- [ ] Integrar primer emulador independiente.
- [ ] Selección automática y override manual.
- [ ] Manejo robusto de procesos y sesiones.

**Salida:** dos motores integrados con flujo coherente.

## Fase 4 — Experiencia de juego
- [ ] Perfiles de controles.
- [ ] Configuración por juego y sistema.
- [ ] Guardados y save states según motor.
- [ ] Capturas y estadísticas.
- [ ] Pantalla completa y navegación con mando.

**Salida:** experiencia de juego estable.

## Fase 5 — Pulido y distribución
- [ ] Temas y personalización.
- [ ] Español e inglés.
- [ ] Empaquetado y actualización segura.
- [ ] CI para build y pruebas.
- [ ] Documentación de instalación y licencias de terceros.

**Salida:** versión candidata distribuible.

## Criterios globales de finalización

Una tarea solo se marca como completada cuando:
- Existe implementación real.
- Pasa las pruebas pertinentes.
- No introduce regresiones conocidas.
- La documentación está actualizada.
- Las limitaciones restantes están registradas.
