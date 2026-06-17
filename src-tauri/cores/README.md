# Core Libretro mGBA

Archivo requerido:

```
src-tauri/cores/mgba_libretro.so
```

## Instalar (automático)

Desde la raíz del proyecto:

```bash
bash scripts/install-mgba-core.sh
```

## Manual

```bash
cd src-tauri/cores
curl -fL -o mgba.zip \
  "https://buildbot.libretro.com/nightly/linux/x86_64/latest/mgba_libretro.so.zip"
unzip -o mgba.zip
chmod +x mgba_libretro.so
rm mgba.zip
```

## Verificar

```bash
nm -D mgba_libretro.so | grep retro_init
```
