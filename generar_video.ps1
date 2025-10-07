# Script PowerShell para generar video del diorama
Write-Host "🎬 GENERADOR DE VIDEO DEL DIORAMA 🎬" -ForegroundColor Cyan
Write-Host ""

# Compilar el generador de video
Write-Host "Compilando generador de video..." -ForegroundColor Yellow
cargo build --release --bin generar_video

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error en la compilación" -ForegroundColor Red
    Read-Host "Presiona Enter para continuar"
    exit 1
}

Write-Host ""
Write-Host "🎞️ Generando frames del video..." -ForegroundColor Green
.\target\release\generar_video.exe

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error generando frames" -ForegroundColor Red
    Read-Host "Presiona Enter para continuar"
    exit 1
}

Write-Host ""
Write-Host "🎥 Verificando si ffmpeg está disponible..." -ForegroundColor Yellow

# Verificar si ffmpeg está disponible
try {
    $null = Get-Command ffmpeg -ErrorAction Stop
    $ffmpegAvailable = $true
} catch {
    $ffmpegAvailable = $false
}

if (-not $ffmpegAvailable) {
    Write-Host "⚠️ FFmpeg no encontrado. Por favor instala FFmpeg o ejecuta manualmente:" -ForegroundColor Yellow
    Write-Host "ffmpeg -framerate 30 -i frame_%04d.png -c:v libx264 -pix_fmt yuv420p diorama_video.mp4" -ForegroundColor White
    Write-Host ""
    Write-Host "📥 Puedes descargar FFmpeg desde: https://ffmpeg.org/download.html" -ForegroundColor Cyan
    Read-Host "Presiona Enter para continuar"
    exit 0
}

Write-Host ""
Write-Host "🎬 Creando video MP4..." -ForegroundColor Green
ffmpeg -y -framerate 30 -i frame_%04d.png -c:v libx264 -pix_fmt yuv420p -crf 18 diorama_video.mp4

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ ¡Video creado exitosamente como 'diorama_video.mp4'!" -ForegroundColor Green
    Write-Host "🧹 Limpiando frames temporales..." -ForegroundColor Yellow
    
    # Limpiar frames temporales
    for ($i = 0; $i -lt 120; $i++) {
        $frameName = "frame_{0:D4}.png" -f $i
        if (Test-Path $frameName) {
            Remove-Item $frameName
        }
    }
    
    Write-Host ""
    Write-Host "🎉 ¡Proceso completado!" -ForegroundColor Green
    Write-Host "📹 Video disponible: diorama_video.mp4" -ForegroundColor Cyan
    Write-Host "⏱️ Duración: 4 segundos a 30fps" -ForegroundColor Cyan
} else {
    Write-Host "❌ Error creando el video" -ForegroundColor Red
}

Write-Host ""
Read-Host "Presiona Enter para continuar"