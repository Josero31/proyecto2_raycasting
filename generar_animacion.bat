@echo off
echo 🎬 Generando animacion GIF del diorama Minecraft...

REM Verificar si existe ImageMagick
where magick >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ ImageMagick no encontrado. Instalando...
    winget install ImageMagick.ImageMagick
    if %errorlevel% neq 0 (
        echo ❌ Error instalando ImageMagick
        echo 💡 Instala manualmente desde: https://imagemagick.org/script/download.php#windows
        pause
        exit /b 1
    )
)

REM Esperar a que todas las vistas se generen
echo ⏳ Esperando a que se generen todas las vistas...
:wait_loop
set count=0
for %%f in (diorama_vista_*.png) do set /a count+=1
echo Vistas encontradas: %count%/8

if %count% lss 8 (
    timeout /t 2 /nobreak >nul
    goto wait_loop
)

echo ✅ Todas las vistas generadas!

REM Generar GIF con ImageMagick
echo 🎨 Creando animacion GIF...
magick convert -delay 100 diorama_vista_*.png -loop 0 animacion_diorama.gif

if %errorlevel% equ 0 (
    echo ✅ Animacion generada: animacion_diorama.gif
    echo 📁 Abriendo carpeta...
    explorer .
) else (
    echo ❌ Error generando animacion
)

echo.
echo 🎉 Proceso completado!
echo 📋 Archivos generados:
echo    - diorama_preview.png (preview rapido)
echo    - diorama_renderizado.png (imagen completa)
echo    - diorama_vista_00.png - diorama_vista_07.png (vistas multiples)
echo    - animacion_diorama.gif (animacion)
echo.
pause