@echo off
echo 🔄 REGENERANDO TODAS LAS IMÁGENES CON MEJORAS
echo ===============================================

echo.
echo 🗑️ Paso 1: Eliminando imágenes anteriores...
call limpiar_imagenes.bat

echo.
echo 🎨 Paso 2: Generando preview mejorado...
cargo run --bin preview_rapido

echo.
echo 🖼️ Paso 3: Generando imagen principal mejorada...
timeout /t 2 /nobreak >nul
cargo run --bin diorama

echo.
echo 🎬 Paso 4: Generando vistas múltiples mejoradas...
timeout /t 2 /nobreak >nul  
cargo run --bin vistas_multiples

echo.
echo 🎉 TODAS LAS IMÁGENES REGENERADAS CON MEJORAS
echo =============================================
echo 📁 Abriendo carpeta para ver resultados...
explorer .

echo.
echo ✨ Ahora puedes ver todas las mejoras implementadas:
echo    - Cámara optimizada para vista panorámica
echo    - 45 árboles mejor distribuidos
echo    - 60 flores más visibles
echo    - Composición visual mejorada
echo.
pause