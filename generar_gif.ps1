# Script para generar GIF animado del diorama
# Requiere ImageMagick instalado

Write-Host "🎬 Creando GIF animado del diorama..."

# Verificar si ImageMagick está instalado
try {
    $null = Get-Command magick -ErrorAction Stop
    Write-Host "✅ ImageMagick encontrado"
} catch {
    Write-Host "❌ ImageMagick no encontrado. Instala ImageMagick desde: https://imagemagick.org/script/download.php#windows"
    Write-Host "O usa la línea de comandos: winget install ImageMagick.ImageMagick"
    exit 1
}

# Generar todas las vistas si no existen
if (-not (Test-Path "diorama_vista_07.png")) {
    Write-Host "🔄 Generando vistas múltiples..."
    cargo run --release --bin vistas_multiples
}

# Crear GIF animado
Write-Host "🎞️ Creando GIF animado..."
magick -delay 80 diorama_vista_*.png -loop 0 diorama_animado.gif

if (Test-Path "diorama_animado.gif") {
    Write-Host "✅ GIF animado creado: diorama_animado.gif"
    Write-Host "🎉 ¡Listo! Ahora tienes un video animado de tu diorama."
} else {
    Write-Host "❌ Error al crear el GIF animado"
}

Write-Host ""
Write-Host "📁 Archivos generados:"
Get-ChildItem -Name "diorama_*"