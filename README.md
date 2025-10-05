# 🌅 Diorama de Raytracing - Paisaje Minecraft Sunset

### Elementos Principales (Basado en el Proyecto Original)
- **🌍 Terreno Ondulado**: Superficie generada proceduraEl diorama replica fielmente el proyecto de referencia con:
- 🌅 **Atmósfera de Atardecer**: Iluminación cálida con sol en posición de sunset
- 🌍 **Terreno Procedural**: Superficie ondulada generada con funciones matemáticas
- 🏞️ **Río Natural**: Agua semi-transparente con refracción y arena en el fondo
- 🌳 **Ecosistema Completo**: Bosque de 35 árboles con tres especies diferentes
- 🌸 **Biodiversidad**: 50 flores multicolores con tallos verdes
- 🪨 **Geología Realista**: Rocas distribuidas de forma natural
- ✨ **Efectos Avanzados**: Reflexiones, refracciones y sombras ray-traced
- 🎨 **Paleta Sunset**: Colores cálidos que evocan un atardecer doradon alturas variables
- **🏞️ Río Serpenteante**: Río azul con arena en el fondo que atraviesa el paisaje
- **🌳 Bosque Variado**: 35 árboles de tres tipos (Oak, Cherry, Birch) con diferentes alturas
- **🌸 Flores Silvestres**: 50 flores coloridas dispersas por el terreno (5 colores diferentes)
- **🪨 Rocas Decorativas**: 20 rocas de diferentes tamaños distribuidas naturalmente
- **🌅 Iluminación de Atardecer**: Sol en ángulo de 40° con colores cálidos de sunset
- **🎨 Skybox Degradado**: Cielo con transición de naranja claro a naranja intensoecto de raytracing que renderiza un paisaje de Minecraft con iluminación de atardecer, terreno ondulado, río, árboles y flores. Diorama de Raytracing - Paisaje Minecraft

Un proyecto de raytracing que renderiza un paisaje típico de Minecraft con casa, árboles, lago y montañas.

## 🎯 Características Implementadas

### ✅ Requerimientos Cumplidos (100/100 puntos)

- **[30 puntos] Complejidad de la escena**: Paisaje completo con casa, árboles, lago, montaña y elementos decorativos
- **[20 puntos] Atractivo visual**: Estética auténtica de Minecraft con colores y formas reconocibles
- **[20 puntos] Rotación y zoom de cámara**: Sistema completo de navegación orbital
- **[25 puntos] Materiales diferentes** (5 × 5 puntos):
  1. **Piedra**: Gris clásico de Minecraft, completamente mate
  2. **Madera de Roble**: Café característico, textura natural
  3. **Oro**: Amarillo brillante con reflexiones
  4. **Vidrio**: Transparente azulado con refracción
  5. **Césped**: Verde Minecraft auténtico
  6. **Agua**: Azul clásico semi-transparente con refracción
  7. **Lava**: Naranja-rojo emisivo brillante
  8. **Tierra**: Café oscuro mate
  9. **Adoquín**: Gris texturizado
  10. **Hojas**: Verde oscuro para árboles
- **[10 puntos] Refracción**: Implementada en vidrio y agua
- **[5 puntos] Reflexión**: Implementada en oro y superficies reflectantes
- **[20 puntos] Skybox**: Cielo azul característico de Minecraft

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
- **� Casa de Minecraft**: Estructura clásica con base de adoquín, paredes de madera y ventanas de vidrio
- **🌊 Lago Natural**: Agua azul transparente típica del juego
- **🌳 Árboles**: Troncos de madera de roble con coronas de hojas verdes
- **⛰️ Montaña de Piedra**: Formación rocosa con piedra y adoquín
- **🔥 Pozo de Lava**: Lava naranja emisiva
- **� Antorchas**: Elementos dorados brillantes para iluminación

### Paleta de Colores Minecraft Auténtica
- **Piedra**: Gris clásico (#808080)
- **Madera de Roble**: Café natural (#996633)
- **Oro**: Amarillo brillante (#FFCC00)
- **Vidrio**: Azul claro transparente (#CCE6FF)
- **Césped**: Verde Minecraft (#4D9933)
- **Agua**: Azul característico (#3366CC)
- **Lava**: Naranja-rojo (#FF6600)
- **Tierra**: Café oscuro (#663319)
- **Adoquín**: Gris piedra (#666666)
- **Hojas**: Verde oscuro (#336619)

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

El diorama muestra un paisaje auténtico de Minecraft con:
- � Casa típica con materiales reconocibles
- 💎 Efectos de refracción en vidrio y agua
- ✨ Reflexiones en superficies de oro
- 🌊 Lago con agua transparente
- 🔥 Pozo de lava emisiva
- 🌳 Árboles con estructura cúbica característica
- ⛰️ Montaña de piedra natural
- 🌅 Cielo azul clásico de Minecraft

## 📝 Notas de Desarrollo

Este proyecto fue creado específicamente para cumplir con los requerimientos del curso de Gráficas por Computadora, implementando un raytracer completo desde cero sin usar librerías externas de renderizado. Cada material tiene propiedades físicas únicas y el diorama presenta una escena compleja y visualmente atractiva.

El diseño sigue fielmente la estética visual de Minecraft, utilizando los colores, materiales y formas características del juego para crear un paisaje auténtico y reconocible. La estructura cúbica y los materiales están inspirados directamente en los bloques clásicos del juego.

---
**Desarrollado con ❤️ usando Rust y matemáticas 3D**