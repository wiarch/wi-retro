# Plan maestro de implementación — wi-retro

> Este documento es la guía de ejecución para Claude Code. No presupone que el proyecto esté vacío. Primero audita la implementación existente y continúa desde ella.

## Objetivo

Convertir wi-retro en un centro de biblioteca y emulación de videojuegos para escritorio, combinando una experiencia de catálogo visual tipo LaunchBox con la flexibilidad de RetroArch. La arquitectura final será híbrida: Libretro como primera opción y emuladores independientes mediante adaptadores.

## Contexto técnico existente

- Tauri 2.
- React 19, TypeScript y Vite.
- Backend Rust.
- Persistencia SQLite/local.
- Escaneo e indexación inicial de ROMs GBA.
- Integración Libretro/mGBA con carga dinámica.
- Ventana de emulación GBA independiente y manejo inicial de controles.
- Configuración de build y script de instalación del core mGBA.

Audita los archivos reales antes de confiar en esta lista. El código del repositorio es la fuente de verdad.

## Fase 0 — Auditoría y estabilización

1. Inspecciona frontend, backend, persistencia, comandos Tauri, capabilities y scripts.
2. Ejecuta build frontend, fmt, clippy y tests donde el entorno lo permita.
3. Identifica errores, deuda técnica, riesgos de seguridad y funcionalidades incompletas.
4. Documenta los flujos reales: escaneo, listado, lanzamiento, entrada, cierre y persistencia.
5. No hagas refactors amplios durante la auditoría.
6. Entrega un informe priorizado y corrige únicamente defectos críticos y de bajo riesgo.

**Aceptación:** mapa de arquitectura actualizado, build reproducible y lista de problemas con evidencia.

## Fase 1 — Biblioteca local robusta

- Diseñar un modelo de juego independiente del sistema emulador.
- Escanear carpetas seleccionadas de forma recursiva y cancelable.
- Soportar extensiones por sistema y archivos comprimidos de forma segura.
- Calcular hashes para detectar duplicados y reconocer revisiones.
- Evitar seguir enlaces simbólicos fuera de ubicaciones autorizadas.
- Persistir bibliotecas, ubicaciones, juegos y resultados de escaneo.
- Detectar archivos movidos, ausentes o unidades desconectadas.
- Permitir importación individual, por carpeta y reescaneo incremental.

**Aceptación:** importación repetible sin duplicados, persistencia tras reiniciar y errores recuperables.

## Fase 2 — Catálogo y metadatos

- Vista de biblioteca con carátulas, títulos, sistema, género y estado.
- Búsqueda rápida, filtros, ordenamiento, favoritos y colecciones.
- Pantalla de detalle con descripción, capturas y datos técnicos.
- Proveedores de metadatos desacoplados mediante adaptadores.
- Caché local y atribución de las fuentes.
- Edición manual cuando la identificación automática falle.
- No enviar ROMs completas a servicios externos; usar identificadores o hashes cuando el proveedor lo permita.

**Aceptación:** biblioteca usable sin conexión después de sincronizar metadatos.

## Fase 3 — Plataforma de emulación híbrida

- Definir un contrato de motor solo cuando se incorpore el segundo motor real.
- Mantener Libretro como opción preferente.
- Incorporar emuladores independientes mediante adaptadores y procesos controlados.
- Detectar instalaciones, versiones, capacidades y sistemas soportados.
- Selección automática por compatibilidad y preferencia manual por juego/sistema.
- Validar rutas de ejecutables y argumentos; no invocar shell.
- Gestionar sesiones, errores, cierre limpio y procesos huérfanos.
- Configuraciones globales, por sistema y por juego.

**Aceptación:** dos motores reales integrados mediante el mismo flujo de lanzamiento, con fallback y errores claros.

## Fase 4 — Experiencia de juego

- Perfiles de controles por sistema y juego.
- Configuración de pantalla, escalado, filtros y relación de aspecto.
- Guardados y save states cuando el motor los soporte.
- Capturas, tiempo jugado, último acceso y progreso.
- Ventana, pantalla completa y navegación por teclado/control.
- Pausa, reanudación y cierre consistente.

**Aceptación:** sesiones estables, estado recuperable y controles configurables.

## Fase 5 — Experiencia avanzada

- Temas y personalización del catálogo.
- Colecciones inteligentes y estadísticas.
- Gestión de múltiples bibliotecas y discos externos.
- Importación/exportación de configuración.
- Actualización segura de metadatos y componentes.
- Localización inicial español e inglés.

**Aceptación:** experiencia consistente, documentada y con pruebas de regresión.

## Requisitos de producto y legales

- El usuario administra ROMs que posee o está autorizado a utilizar.
- No incluir ROMs comerciales, BIOS propietarios ni material protegido en el repositorio.
- Facilitar fuentes legales de juegos gratuitos/de dominio público, sin scraping ni redistribución no autorizada.
- Los cores y emuladores se instalan desde fuentes oficiales o verificables, respetando sus licencias.
- Documentar licencias y avisos de terceros.

## Requisitos de calidad

- Mantener el código modular, tipado y testeable.
- No bloquear UI ni runtime async con tareas pesadas.
- Añadir migraciones SQLite versionadas.
- Manejar errores de IO, permisos, discos desconectados y archivos corruptos.
- Aplicar mínimo privilegio en Tauri.
- Evitar comandos shell construidos con datos del usuario.
- Mantener build reproducible en Linux y Windows; macOS se evaluará según disponibilidad.
- Actualizar README, docs y este roadmap al completar fases.

## Forma de trabajo de Claude Code

Trabaja una fase por vez. Antes de implementar, presenta el plan y los archivos afectados. No avances a la siguiente fase si hay fallos de build o aceptación. Al cerrar cada fase, entrega:
- Resumen de cambios.
- Archivos afectados.
- Pruebas ejecutadas y resultado real.
- Limitaciones conocidas.
- Siguiente fase recomendada.

**Primera tarea:** ejecutar la Fase 0. No empieces construyendo un sistema nuevo ni sustituyas el emulador GBA existente.
