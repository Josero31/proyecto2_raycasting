# Diorama Minecraft - Proyecto de Gráficas por Computadora

**Video de demostración:** https://youtu.be/oX2VfSBshAU (referencia del repositorio original)

## Descripción

Este proyecto crea un diorama estilo Minecraft con efectos de sunset y sombras ray-traced en tiempo real usando Raylib.

### Características

- **Renderizado 3D en tiempo real** con Raylib
- **Shader personalizado** con efectos de atardecer (sunset)
- **Sombras ray-traced** calculadas en CPU
- **Control interactivo** con mouse
- **Terreno procedural** con colinas y valles
- **Vegetación variada**: Árboles (Oak, Cherry, Birch), flores, rocas
- **Río** con agua semitransparente
- **Efectos visuales**: Ambient Occlusion, Global Illumination, Rim Lighting

## Requisitos

- Rust (última versión estable)
- Cargo
- Windows/Linux/macOS

## Compilar y Ejecutar

```bash
cargo run --release
```

## Controles

- **Click Izquierdo + Arrastrar**: Rotar cámara
- **Rueda del Mouse**: Zoom in/out
- **ESC**: Salir

## Estructura del Proyecto

```
proyecto2_raycasting/
├── src/
│   └── main.rs           # Código principal con rendering y shaders
├── Cargo.toml            # Dependencias del proyecto
└── README.md             # Este archivo
```

## Créditos

Basado en el proyecto de referencia: https://github.com/pablouwunya2021/Graficas_Proyecto2
