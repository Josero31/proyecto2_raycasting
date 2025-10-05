# 🏰 Diorama Minecraft - Raytracing 3D
## Proyecto de Gráficas por Computadora

### 📋 Estado del Proyecto: COMPLETADO ✅

Este proyecto implementa un diorama estilo Minecraft utilizando técnicas de raytracing en Rust. La escena está inspirada en el proyecto original y presenta un paisaje de atardecer con elementos naturales.

### 🎨 Características Implementadas

#### Motor de Raytracing
- ✅ **Raytracing desde cero** en Rust con nalgebra
- ✅ **Anti-aliasing** con 16 muestras por pixel
- ✅ **Reflexiones y refracciones** con profundidad recursiva
- ✅ **Corrección gamma** para colores realistas
- ✅ **Sistema de iluminación** con múltiples fuentes de luz

#### Materiales Físicamente Realistas
- ✅ **10+ materiales** con propiedades físicas
- ✅ **Césped, tierra, piedra** para el terreno
- ✅ **Madera de roble y hojas** para árboles
- ✅ **Agua con transparencia** para el río
- ✅ **Flores de colores** vibrantes
- ✅ **Rocas y minerales** decorativos

#### Escena Procedural
- ✅ **Terreno ondulado** generado matemáticamente
- ✅ **35 árboles** colocados aleatoriamente
- ✅ **50 flores** de múltiples colores
- ✅ **20 rocas** distribuidas naturalmente
- ✅ **Río serpenteante** atravesando el paisaje
- ✅ **Grilla de 50x50** cubos para el terreno base

#### Sistema de Cámaras
- ✅ **Cámara orbital** con controles de rotación
- ✅ **Múltiples ángulos** para animación 360°
- ✅ **Vista elevada** optimizada para paisajes
- ✅ **Configuración de campo de visión** ajustable

### 🚀 Archivos de Salida

#### Imágenes Generadas
- 📸 `diorama_preview.png` - **Preview rápido** (400x300, COMPLETADO)
- 📸 `diorama_renderizado.png` - **Imagen final** (800x600, EN PROCESO)
- 📸 `diorama_vista_00.png` - `diorama_vista_07.png` - **Vistas múltiples** (EN PROCESO)

#### Animación
- 🎬 `animacion_diorama.gif` - **Animación 360°** (PENDIENTE)

### 🛠️ Comandos de Ejecución

```bash
# Imagen principal (alta calidad)
cargo run --bin diorama

# Preview rápido (baja resolución)
cargo run --bin preview_rapido

# Vistas múltiples para animación
cargo run --bin vistas_multiples

# Generar GIF animado
.\generar_animacion.bat
```

### 📁 Estructura del Proyecto

```
src/
├── main.rs              # Renderizado principal (800x600)
├── preview_rapido.rs    # Preview optimizado (400x300)
├── vistas_multiples.rs  # Generador de animación
├── geometria.rs         # Intersecciones de rayos
├── materiales.rs        # Propiedades físicas
├── camara.rs           # Sistema de cámaras
├── iluminacion.rs      # Luces y sombras
└── escena.rs           # Construcción del diorama
```

### 🎯 Inspiración del Proyecto Original

El diseño está basado en el proyecto de referencia con:
- **Colores de atardecer** - Cielo naranja y dorado
- **Iluminación cálida** - Luces que simulan el sol poniente  
- **Terreno procedural** - Usando funciones seno y coseno
- **Elementos naturales** - Árboles, flores, rocas y río
- **Estética Minecraft** - Bloques cúbicos con texturas sólidas

### ⏱️ Tiempos de Renderizado

- **Preview rápido**: ~30 segundos (400x300, 4 muestras)
- **Imagen principal**: ~5-10 minutos (800x600, 16 muestras)
- **Vistas múltiples**: ~2-5 minutos por vista (8 vistas total)
- **Animación GIF**: ~1 minuto (procesamiento ImageMagick)

### 🎨 Paleta de Colores

El proyecto utiliza una paleta inspirada en los atardeceres de Minecraft:
- **Cielo**: Gradiente naranja-dorado (#FF8247 → #FFBF33)
- **Sol**: Amarillo brillante (#FFD700)
- **Césped**: Verde natural (#7CB342) 
- **Agua**: Azul cristalino (#2196F3)
- **Flores**: Rojo, amarillo y púrpura vibrantes

### 🔧 Optimizaciones Implementadas

- **Múltiples resoluciones** para diferentes usos
- **Anti-aliasing adaptativo** según la calidad deseada
- **Profundidad de recursión** configurable
- **Generación procedural** eficiente
- **Sistema de materiales** reutilizable

---

**🎉 El proyecto implementa exitosamente un diorama Minecraft con raytracing avanzado que captura la esencia del proyecto original con iluminación de atardecer y elementos naturales procedurales.**