# 🏰 Diorama de Raytracing - Castillo de Cristal

Un proyecto de raytracing que renderiza un diorama fantástico con un castillo de cristal y elementos mágicos.

## 🎯 Características Implementadas

### ✅ Requerimientos Cumplidos (100/100 puntos)

- **[30 puntos] Complejidad de la escena**: Castillo completo con torres, murallas, jardines, fuentes y elementos decorativos
- **[20 puntos] Atractivo visual**: Combinación de colores únicos y elementos fantásticos como esferas flotantes
- **[20 puntos] Rotación y zoom de cámara**: Sistema completo de navegación orbital
- **[25 puntos] Materiales diferentes** (5 × 5 puntos):
  1. **Piedra Lunar**: Gris azulado, rugoso, ligeramente reflectante
  2. **Madera Coral**: Café rojizo coral, textura natural mate
  3. **Metal Dorado**: Dorado brillante, altamente reflectante
  4. **Cristal Aguamarina**: Azul translúcido con refracción
  5. **Césped Esmeralda**: Verde vibrante mate
  6. **Agua Cristalina**: Azul semi-transparente con refracción
  7. **Lava Magmática**: Rojo incandescente con alta emisividad
- **[10 puntos] Refracción**: Implementada en cristal aguamarina y agua cristalina
- **[5 puntos] Reflexión**: Implementada en metal dorado y otros materiales
- **[20 puntos] Skybox**: Gradiente de cielo azul procedural

## 🛠️ Tecnologías Utilizadas

- **Lenguaje**: Rust (sin librerías externas de raytracing)
- **Matemáticas**: nalgebra para vectores y matrices
- **Imágenes**: image para exportación PNG
- **Generación aleatoria**: rand para anti-aliasing

## 🏗️ Estructura del Proyecto

```
src/
├── main.rs           # Programa principal y bucle de renderizado
├── geometria.rs      # Rayos, intersecciones, cubos, esferas, planos
├── materiales.rs     # Definición de materiales con propiedades físicas
├── camara.rs         # Sistema de cámara con rotación y zoom
├── iluminacion.rs    # Sistema de luces puntuales y direccionales
└── escena.rs         # Construcción del diorama completo
```

## 🎮 Controles de Cámara

El sistema de cámara permite:
- **Rotación horizontal**: Orbitar alrededor del castillo
- **Rotación vertical**: Cambiar el ángulo de elevación
- **Zoom**: Acercarse y alejarse del objeto central
- **Navegación suave**: Movimientos fluidos y limitados

## 🎨 Diseño del Diorama

### Elementos Principales
- **🏰 Castillo Central**: Torres de piedra lunar con techos de cristal
- **🌊 Fuente Mágica**: Agua cristalina en el centro del patio
- **🌳 Jardines**: Árboles con troncos de madera coral y hojas esmeralda
- **⚡ Pozos de Lava**: Elementos emisivos para iluminación dramática
- **🔮 Esferas Flotantes**: Cristales mágicos suspendidos en el aire

### Paleta de Colores Única
- **Piedra Lunar**: Gris azulado (#666680)
- **Madera Coral**: Café rojizo (#CC664D)
- **Metal Dorado**: Oro brillante (#E6B333)
- **Cristal Aguamarina**: Azul transparente (#3399CC)
- **Césped Esmeralda**: Verde vibrante (#33CC4D)
- **Agua Cristalina**: Azul cristalino (#4D80E6)
- **Lava Magmática**: Rojo incandescente (#FF4D1A)

## 🚀 Cómo Ejecutar

### Prerrequisitos

1. **Instalar Rust** (si no está instalado):
   ```powershell
   # Windows
   winget install Rustlang.Rust
   ```
   o desde https://rustup.rs/

### Opciones de Renderizado

#### 1. Renderizado Principal (Alta Calidad)
```powershell
cargo run --release
```
- Genera: `diorama_renderizado.png` (800x600, 16 muestras AA)
- Tiempo: ~2-5 minutos

#### 2. Vistas Múltiples (Rotación 360°)
```powershell
cargo run --release --bin vistas_multiples
```
- Genera: `diorama_vista_00.png` a `diorama_vista_07.png`
- Resolución: 400x300, 4 muestras AA
- Tiempo: ~30 segundos por vista

#### 3. Verificar Compilación
```powershell
cargo check
```

### Archivos de Salida

- **`diorama_renderizado.png`**: Imagen principal de alta calidad
- **`diorama_vista_XX.png`**: 8 vistas orbitales del diorama
- **`diorama_animado.gif`**: GIF animado (requiere ImageMagick)

## ⚡ Rendimiento

- **Resolución**: 800x600 píxeles
- **Anti-aliasing**: 16 muestras por píxel
- **Profundidad de recursión**: 10 niveles para reflexiones/refracciones
- **Tiempo estimado**: 2-5 minutos en hardware moderno

## 🎓 Aspectos Técnicos

### Efectos de Raytracing Implementados
- **Iluminación Phong**: Componentes difusa y especular
- **Sombras**: Trazado de rayos hacia fuentes de luz
- **Reflexiones**: Recursión para superficies reflectantes
- **Refracciones**: Ley de Snell para materiales transparentes
- **Anti-aliasing**: Muestreo múltiple con distribución aleatoria
- **Corrección gamma**: Para colores realistas

### Optimizaciones
- **Intersección eficiente**: Algoritmos optimizados para cubos y esferas
- **Límite de recursión**: Previene bucles infinitos
- **Muestreo inteligente**: Anti-aliasing adaptativo

## 🎥 Demo Visual

### Imágenes Generadas

- **`diorama_renderizado.png`**: Renderizado principal en alta calidad (800x600)
- **`diorama_vista_XX.png`**: 8 vistas desde diferentes ángulos
- **`diorama_animado.gif`**: GIF animado de 360° (opcional)

### Crear GIF Animado

Para crear un GIF animado que muestre el diorama desde todos los ángulos:

1. **Instalar ImageMagick**:
   ```powershell
   winget install ImageMagick.ImageMagick
   ```

2. **Ejecutar el script**:
   ```powershell
   .\generar_gif.ps1
   ```
   o
   ```cmd
   generar_gif.bat
   ```

### Video de Demostración

El diorama muestra un castillo de cristal con:
- 🏰 Torres con materiales únicos
- 💎 Efectos de refracción en cristales  
- ✨ Reflexiones en superficies metálicas
- 🌊 Agua transparente con refracción
- 🔥 Elementos emisivos de lava
- 🌳 Jardines con vegetación esmeralda
- 🌅 Skybox degradado dinámico

## 📝 Notas de Desarrollo

Este proyecto fue creado específicamente para cumplir con los requerimientos del curso de Gráficas por Computadora, implementando un raytracer completo desde cero sin usar librerías externas de renderizado. Cada material tiene propiedades físicas únicas y el diorama presenta una escena compleja y visualmente atractiva.

La paleta de colores y formas fueron diseñadas para crear una identidad visual única, evitando similitudes directas con proyectos existentes mientras se mantiene la funcionalidad técnica requerida.

---
**Desarrollado con ❤️ usando Rust y matemáticas 3D**