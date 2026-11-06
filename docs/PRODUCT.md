# Definición del producto — wi-retro

## Resumen

wi-retro es una aplicación de escritorio para organizar, explorar y ejecutar bibliotecas personales de videojuegos. Combina un catálogo visual y herramientas de gestión inspiradas en LaunchBox con un sistema de emulación flexible inspirado en RetroArch.

## Usuarios objetivo

- Personas que mantienen bibliotecas de juegos retro.
- Usuarios que utilizan varias consolas y emuladores.
- Usuarios que desean una experiencia unificada, visual y configurable.
- Usuarios que administran sus propios archivos de juego.

## Propuesta de valor

Una sola aplicación para importar juegos, reconocerlos, organizarlos, consultar información y ejecutarlos con el motor más adecuado disponible.

## Alcance funcional

### Biblioteca
- Bibliotecas en carpetas locales y unidades externas.
- Escaneo recursivo e incremental.
- Identificación por sistema, extensión, hash y metadatos.
- Detección de duplicados, revisiones y archivos ausentes.
- Favoritos, colecciones, filtros y búsqueda.

### Catálogo
- Carátulas, capturas y fondos.
- Descripciones, géneros, año, desarrollador y plataforma.
- Edición manual de metadatos.
- Proveedores externos desacoplados y caché local.

### Emulación
- Libretro como motor preferente cuando exista un core compatible.
- Emuladores independientes mediante adaptadores.
- Selección automática y override manual.
- Configuración global, por plataforma y por juego.
- Sesiones, controles, guardados y estadísticas según capacidades del motor.

### Experiencia
- Interfaz de escritorio adaptable.
- Modo ventana y pantalla completa.
- Navegación con teclado y mandos.
- Temas configurables.
- Español e inglés como idiomas iniciales previstos.

## Fuera de alcance inicial

- Distribución de ROMs comerciales.
- Descarga desde repositorios no autorizados.
- Emulación de todas las plataformas desde el primer lanzamiento.
- Sincronización cloud obligatoria.
- Tienda comercial de videojuegos.

## Modelo legal y de contenido

La aplicación organiza archivos proporcionados por el usuario. No incluirá ROMs comerciales ni BIOS protegidos. Las integraciones de metadatos deben respetar términos, límites y atribución de cada proveedor. Los juegos gratuitos o de dominio público solo podrán enlazarse o descargarse cuando la fuente y la licencia permitan expresamente ese uso.

## Estado del producto

La base actual se concentra en GBA y Libretro/mGBA. El resto de capacidades son objetivos de evolución y no deben presentarse como disponibles hasta que estén implementadas y verificadas.

## Principios de experiencia

- La biblioteca debe seguir siendo útil aunque no haya conexión.
- El usuario mantiene el control de sus archivos y ubicaciones.
- La configuración avanzada no debe complicar el uso básico.
- Los errores deben explicar qué ocurrió y cómo recuperarse.
- No ocultar incompatibilidades ni inventar soporte de plataformas.
