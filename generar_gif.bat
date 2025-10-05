@echo off
echo 🎬 Creando GIF animado del diorama...

REM Verificar si ImageMagick está instalado
magick -version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ ImageMagick no encontrado. Instala ImageMagick desde:
    echo    https://imagemagick.org/script/download.php#windows
    echo O usa: winget install ImageMagick.ImageMagick
    pause
    exit /b 1
)

echo ✅ ImageMagick encontrado

REM Generar todas las vistas si no existen
if not exist "diorama_vista_07.png" (
    echo 🔄 Generando vistas múltiples...
    cargo run --release --bin vistas_multiples
)

REM Crear GIF animado
echo 🎞️ Creando GIF animado...
magick -delay 80 diorama_vista_*.png -loop 0 diorama_animado.gif

if exist "diorama_animado.gif" (
    echo ✅ GIF animado creado: diorama_animado.gif
    echo 🎉 ¡Listo! Ahora tienes un video animado de tu diorama.
) else (
    echo ❌ Error al crear el GIF animado
)

echo.
echo 📁 Archivos generados:
dir /b diorama_*

pause