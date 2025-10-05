@echo off
echo 🗑️ Eliminando todas las imágenes anteriores...

if exist "diorama_preview.png" (
    del "diorama_preview.png"
    echo ✅ diorama_preview.png eliminado
)

if exist "diorama_renderizado.png" (
    del "diorama_renderizado.png"
    echo ✅ diorama_renderizado.png eliminado
)

for %%i in (diorama_vista_*.png) do (
    del "%%i"
    echo ✅ %%i eliminado
)

if exist "animacion_diorama.gif" (
    del "animacion_diorama.gif"
    echo ✅ animacion_diorama.gif eliminado
)

echo.
echo 🎉 Todas las imágenes anteriores han sido eliminadas
echo 💡 Ahora puedes ejecutar los renderizados para ver cambios frescos:
echo    - cargo run --bin preview_rapido
echo    - cargo run --bin diorama
echo    - cargo run --bin vistas_multiples
echo.
pause