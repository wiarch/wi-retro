# Arquitectura — wi-retro

## Estado actual

La aplicación está construida con Tauri 2, React 19, TypeScript, Vite y Rust. La capa nativa gestiona persistencia, escaneo y emulación. El frontend consume comandos Tauri y eventos para reflejar el estado.

### Componentes observados

- `src/`: interfaz React, estilos, tipos y wrappers de invocación Tauri.
- `src-tauri/src/commands/`: comandos expuestos al frontend.
- `src-tauri/src/libretro/`: FFI, carga del core, sesión, entrada y vídeo.
- `src-tauri/src/scanner.rs`: escaneo de archivos.
- `src-tauri/src/persistence.rs`: persistencia local.
- `src-tauri/src/gba_window.rs`: ventana dedicada a GBA.
- `src-tauri/capabilities/`: permisos por ventana.
- `src-tauri/cores/`: ubicación local de cores; no versionar binarios.

Verificar siempre el código antes de actualizar este mapa.

## Arquitectura objetivo

Separar cinco responsabilidades:

1. **Presentación:** biblioteca, detalles, ajustes y vista de emulación.
2. **Aplicación:** casos de uso, coordinación y validación.
3. **Dominio:** juegos, plataformas, ubicaciones, colecciones y contratos.
4. **Infraestructura:** SQLite, filesystem, metadatos y procesos.
5. **Motores:** Libretro y adaptadores de emuladores independientes.

## Flujo de biblioteca

1. El usuario autoriza una carpeta.
2. El backend canonicaliza y valida la ubicación.
3. El escáner identifica archivos soportados sin ejecutar su contenido.
4. Se calculan hashes y se extraen datos seguros.
5. La capa de persistencia guarda o actualiza registros.
6. El frontend recibe resultados y estados de progreso.

Los escaneos deben ser cancelables, tolerar errores parciales y no seguir enlaces fuera de ubicaciones autorizadas.

## Arquitectura híbrida de motores

Libretro es la primera opción. Un emulador independiente se integra cuando exista una necesidad demostrable o compatibilidad superior.

Contrato conceptual futuro:
- Identificar sistemas y formatos compatibles.
- Consultar disponibilidad y versión del motor.
- Validar configuración.
- Lanzar un juego con argumentos estructurados.
- Consultar estado y capacidades.
- Cerrar la sesión y limpiar recursos.

No crear una abstracción universal sin un segundo motor implementado. La primera integración independiente debe validar el contrato.

## Persistencia

SQLite es el almacén local previsto y ya existe código de persistencia. Las futuras modificaciones deben:
- Usar migraciones versionadas.
- Mantener claves estables para juegos y ubicaciones.
- No almacenar ROMs dentro de la base de datos.
- Soportar rutas que desaparecen o cambian.
- Preservar datos de usuario durante actualizaciones.

## Seguridad

- Canonicalización y validación de rutas.
- Protección contra traversal y enlaces simbólicos inesperados.
- No ejecutar ROMs ni archivos importados como código.
- Procesos externos con argumentos separados, sin shell.
- Capabilities Tauri de mínimo privilegio.
- CSP explícita y revisada.
- Límites de tamaño y manejo de archivos corruptos.
- Sanitización de metadatos externos.
- No enviar ROMs completas a proveedores de metadatos.

## Decisiones de rendimiento

- Escaneo e indexación fuera del hilo de UI.
- El bucle de emulación corre en un hilo dedicado.
- Transferencia de frames mediante mecanismos acotados para evitar crecimiento ilimitado de memoria.
- Caché local de imágenes y metadatos.
- Consultas paginadas para bibliotecas grandes.

## Plataformas

Linux y Windows son los objetivos iniciales documentados. La compatibilidad real debe verificarse en CI y hardware antes de anunciar soporte estable. macOS queda sujeto a evaluación.
