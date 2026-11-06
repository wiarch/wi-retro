# Política de seguridad

## Alcance

La seguridad de wi-retro incluye la aplicación de escritorio, el backend Rust, la interfaz Tauri, la importación de archivos, los cores y los procesos externos.

## Reportar vulnerabilidades

No publiques detalles de vulnerabilidades explotables en issues públicos. Utiliza los canales privados de contacto de la organización/repositorio en GitHub. Incluye:
- Versión o commit afectado.
- Sistema operativo y entorno.
- Pasos mínimos para reproducir.
- Impacto observado y evidencia no destructiva.

No se establece un SLA de respuesta hasta que el proyecto publique uno.

## Modelo de amenazas prioritario

- ROMs y archivos comprimidos malformados.
- Path traversal y enlaces simbólicos.
- Cores o ejecutables externos manipulados.
- Argument injection y ejecución de comandos.
- Permisos excesivos de Tauri.
- Metadatos remotos maliciosos.
- Archivos locales sensibles expuestos en logs.

## Requisitos

- Validar rutas mediante canonicalización y límites de directorio.
- Aplicar límites razonables de tamaño y recursos.
- No ejecutar contenido importado como código.
- No construir comandos shell con datos del usuario.
- Verificar procedencia e integridad de cores cuando sea posible.
- Mantener binarios descargados fuera del control de versiones.
- Restringir capabilities y revisar CSP.
- Sanitizar texto y URLs de proveedores externos.
- Evitar transmitir ROMs completas a servicios externos.
- No almacenar secretos en el repositorio.

## Dependencias

Revisa mantenimiento, advisories, licencias y procedencia antes de añadir o actualizar dependencias críticas. Documenta excepciones.
