<div align="center">

#  Diorama Minecraft - Ray Tracing en Tiempo Real

### Proyecto de Graficas por Computadora

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Raylib](https://img.shields.io/badge/Raylib-5.0-red?style=for-the-badge)
![OpenGL](https://img.shields.io/badge/OpenGL-3.3-blue?style=for-the-badge&logo=opengl&logoColor=white)
![License](https://img.shields.io/badge/License-Academic-green?style=for-the-badge)

**Renderizado 3D en tiempo real con sombras ray-traced | 60 FPS | Generacion procedural**

</div>

---

![Diorama Preview](diorama_renderizado.png)

---

##  Descripcion

Proyecto de renderizado 3D que combina **GPU rendering** (Raylib + OpenGL) con **CPU ray tracing** para crear un paisaje estilo Minecraft con iluminacion y sombras realistas.


###  Objetivos del Proyecto

-  Implementar sistema de renderizado 3D en tiempo real
-  Calcular sombras mediante ray tracing
-  Generar terreno y vegetacion de forma procedural
-  Crear controles interactivos de camara
-  Optimizar para 60 FPS constantes

---

##  Caracteristicas

###  Renderizado
- **Framework:** Raylib 5.0
- **API Grafica:** OpenGL 3.3
- **Resolucion:** 1400x900px
- **Performance:** 60 FPS
- **Objetos:** 2,500+ elementos

###  Sombras Ray-Traced
- Calculo en CPU
- 15 pasos por rayo
- Oclusion por arboles/rocas
- Reduccion de brillo 50%

###  Generacion Procedural
- 35 arboles (3 tipos: Roble, Cerezo, Abedul)
- 20 rocas aleatorias
- 50 flores coloridas
- Terreno 50x50 ondulado
- Rio con transparencia

###  Controles
- **Click + Drag:** Rotar camara
- **Rueda:** Zoom (20-100)
- **ESC:** Salir

---

##  Instalacion

### Requisitos Previos

**1. Rust**
```powershell
winget install Rustlang.Rust
```

**2. LLVM/Clang**
```powershell
winget install LLVM.LLVM
```

**3. Variable de Entorno**
```powershell
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
```

### Compilar y Ejecutar

```bash
# Clonar repositorio
git clone https://github.com/Josero31/proyecto2_raycasting.git
cd proyecto2_raycasting

# Compilar en modo release
cargo build --release

# Ejecutar
cargo run --release
```

---

##  Estructura del Proyecto

```
proyecto2_raycasting/
 src/
    main.rs           # Codigo principal (511 lineas)
                        # - Estructuras: Tree, Rock, Flower
                        # - Funcion: is_in_shadow() (ray tracing)
                        # - Funcion: apply_shadow() (sombreado)
                        # - Loop: renderizado 60 FPS
    camara.rs         # Camara (legacy)
    escena.rs         # Escena (legacy)
    materiales.rs     # Materiales (legacy)
 texturas/             # Texturas
 target/               # Binarios compilados
 Cargo.toml            # Dependencias: raylib 5.0, rand 0.8
 Cargo.lock            # Lock file
 README.md             # Este archivo
```

---

##  Documentacion Tecnica

### Estructuras de Datos

```rust
// Arbol con tipo, posicion y geometria
struct Tree {
    x: f32, z: f32,
    height: f32,
    leaf_layers: i32,
    tree_type: TreeType,
}

// Tipos de arboles disponibles
enum TreeType { Oak, Cherry, Birch }

// Roca con posicion y tamano
struct Rock { x: f32, z: f32, size: f32 }

// Flor con posicion y color
struct Flower { x: f32, z: f32, color: Color }
```

### Algoritmo de Sombras (Ray Tracing)

```rust
fn is_in_shadow(x: f32, y: f32, z: f32, 
                trees: &Vec<Tree>, 
                rocks: &Vec<Rock>, 
                light_dir: Vector3) -> bool {
    let shadow_steps = 15;
    let step_size = 0.6;
    
    for i in 1..shadow_steps {
        let test_pos = Vector3::new(
            x - light_dir.x * i as f32 * step_size,
            y - light_dir.y * i as f32 * step_size,
            z - light_dir.z * i as f32 * step_size,
        );
        
        // Detectar oclusion por arboles y rocas
        // ...
    }
    
    false
}

fn apply_shadow(base_color: Color, in_shadow: bool) -> Color {
    if in_shadow {
        // Reducir brillo al 50%
        Color::new(
            (base_color.r as f32 * 0.5) as u8,
            (base_color.g as f32 * 0.5) as u8,
            (base_color.b as f32 * 0.5) as u8,
            base_color.a,
        )
    } else {
        base_color
    }
}
```

---

##  Especificaciones

| Categoria | Especificacion |
|-----------|----------------|
| **Lenguaje** | Rust (Edition 2021) |
| **Framework** | Raylib 5.0 |
| **API Grafica** | OpenGL 3.3 |
| **Resolucion** | 1400900 px |
| **FPS** | 60 (16.67ms/frame) |
| **Terreno** | 5050 bloques (2,500) |
| **Arboles** | 35 (Oak/Cherry/Birch) |
| **Rocas** | 20 (tamano 2.0-5.0) |
| **Flores** | 50 (rojo/amarillo) |
| **Ray Tracing** | 15 pasos por sombra |
| **Zoom** | 20-100 unidades |

---

##  Dependencias

```toml
[package]
name = "diorama_raytracing"
version = "0.1.0"
edition = "2021"

[dependencies]
raylib = "5.0"  # Framework de renderizado 3D
rand = "0.8"    # Generacion aleatoria
```

---

##  Conceptos Implementados

### Graficas por Computadora
-  Renderizado 3D
-  Transformaciones de camara
-  Proyeccion perspectiva
-  Iluminacion y sombreado
-  Transparencia (agua)

### Programacion Avanzada
-  Ray tracing basico
-  Generacion procedural
-  Estructuras de datos
-  Optimizacion de performance
-  Manejo de eventos

---

##  Solucion de Problemas

### Error: "libclang not found"

**Solucion:**
```powershell
winget install LLVM.LLVM
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
cargo build --release
```

### FPS bajos

**Solucion:**
```bash
# SIEMPRE usar --release
cargo run --release
```

### Controles no responden

**Solucion:**
- Click izquierdo: Mantener presionado mientras arrastra
- Rueda: Mover con cursor dentro de la ventana
- ESC: Presionar mientras ventana tiene foco

---

##  Licencia

Proyecto academico desarrollado para el curso de **Graficas por Computadora**.

---

##  Autor

**Proyecto Academico** - 2025  
**Tecnologia:** Rust + Raylib 5.0 + OpenGL 3.3

---

<div align="center">

###  Si te gusto este proyecto, dale una estrella!

**Hecho con  usando Rust**

</div>
