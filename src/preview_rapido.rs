use nalgebra::{Vector3, Point3};
use image::{RgbImage, Rgb};

mod geometria;
mod materiales;
mod camara;
mod iluminacion;
mod escena;

use geometria::*;
use materiales::*;
use camara::*;
use iluminacion::*;
use escena::*;

const ANCHO: u32 = 400;  // Resolución reducida para renderizado rápido
const ALTO: u32 = 300;
const MUESTRAS_ANTIALIASING: u32 = 4;  // Menos muestras para mayor velocidad
const PROFUNDIDAD_MAXIMA: u32 = 6;     // Menos profundidad para mayor velocidad

fn main() {
    println!("🎨 Iniciando renderizado RÁPIDO del diorama...");
    
    // Eliminar imagen anterior si existe
    if std::path::Path::new("diorama_preview.png").exists() {
        std::fs::remove_file("diorama_preview.png").ok();
        println!("🗑️ Preview anterior eliminado");
    }
    
    // Crear una versión simplificada de la escena
    let escena = crear_diorama_simplificado();
    
    // Configurar cámara con ZOOM GRANDÍSIMO para ver detalles
    let terrain_size = 25.0; // Terreno más pequeño para preview
    let camara = Camara::nueva(
        Point3::new(terrain_size/2.0 + 5.0, 8.0, terrain_size/2.0 + 5.0), // MUY CERCA con zoom extremo
        Point3::new(terrain_size/2.0, 2.0, terrain_size/2.0),              // Mirando al centro cerca
        Vector3::new(0.0, 1.0, 0.0),
        45.0,                                                               // Campo de visión normal para zoom
        ANCHO as f64 / ALTO as f64
    );
    
    // Crear imagen para el renderizado
    let mut imagen = RgbImage::new(ANCHO, ALTO);
    
    // Renderizar cada pixel
    for y in 0..ALTO {
        if y % 25 == 0 {
            println!("Procesando línea {} de {}", y, ALTO);
        }
        
        for x in 0..ANCHO {
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            
            // Anti-aliasing con múltiples muestras
            for _ in 0..MUESTRAS_ANTIALIASING {
                let u = (x as f64 + rand::random::<f64>()) / ANCHO as f64;
                let v = (y as f64 + rand::random::<f64>()) / ALTO as f64;
                
                let rayo = camara.obtener_rayo(u, v);
                color += calcular_color(&rayo, &escena, PROFUNDIDAD_MAXIMA);
            }
            
            // Promedio y corrección gamma
            color /= MUESTRAS_ANTIALIASING as f64;
            color = Vector3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
            
            // Convertir a RGB [0, 255]
            let r = (255.0 * color.x.clamp(0.0, 1.0)) as u8;
            let g = (255.0 * color.y.clamp(0.0, 1.0)) as u8;
            let b = (255.0 * color.z.clamp(0.0, 1.0)) as u8;
            
            imagen.put_pixel(x, ALTO - y - 1, Rgb([r, g, b]));
        }
    }
    
    // Guardar la imagen
    match imagen.save("diorama_preview.png") {
        Ok(_) => println!("✅ Preview guardado como 'diorama_preview.png'"),
        Err(e) => println!("❌ Error al guardar imagen: {}", e),
    }
    
    println!("🎉 Renderizado RÁPIDO completado!");
}

// Versión simplificada de la escena para renderizado rápido con ZOOM
fn crear_diorama_simplificado() -> Escena {
    let mut escena = Escena::nueva();
    
    let terrain_size = 15;  // Terreno MUY pequeño para zoom extremo
    let river_center_x = 6;
    let river_width = 2;
    
    // Terreno ondulado simplificado
    for x in (0..terrain_size).step_by(2) {  // Menos densidad
        for z in (0..terrain_size).step_by(2) {
            let height = ((x as f64 * 0.2).sin() + (z as f64 * 0.2).cos()) * 2.5
                + ((x as f64 * 0.5).sin() * (z as f64 * 0.5).cos()) * 1.5
                + 2.0;
            
            let height_int = height.round() as i32;
            
            for y in 0..height_int {
                let material = if y == height_int - 1 {
                    Material::cesped()
                } else if y >= height_int - 2 {
                    Material::tierra()
                } else {
                    Material::piedra()
                };
                
                escena.agregar_objeto(Box::new(Cubo::nuevo(
                    Point3::new(x as f64, y as f64 + 0.5, z as f64),
                    1.0,
                    material
                )));
            }
            
            // Río simplificado
            let dist_to_river = (x as i32 - river_center_x).abs();
            if dist_to_river < river_width {
                escena.agregar_objeto(Box::new(Cubo::nuevo(
                    Point3::new(x as f64, 0.0, z as f64),
                    1.0,
                    Material::agua()
                )));
            }
        }
    }
    
    // Algunos árboles grandes para zoom extremo
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    for _ in 0..6 {
        let x = rng.gen_range(1..(terrain_size-1)) as f64;
        let z = rng.gen_range(1..(terrain_size-1)) as f64;
        
        if (x as i32) < river_center_x - river_width || (x as i32) > river_center_x + river_width {
            let tree_height = 6.0; // Árboles más altos para zoom
            let terrain_height = ((x * 0.2).sin() + (z * 0.2).cos()) * 2.5
                + ((x * 0.5).sin() * (z * 0.5).cos()) * 1.5
                + 2.0;
            
            // Tronco más grueso
            for i in 0..(tree_height as i32) {
                escena.agregar_objeto(Box::new(Cubo::nuevo(
                    Point3::new(x, terrain_height + i as f64 + 0.5, z),
                    1.0, // Tronco más grueso
                    Material::madera_roble()
                )));
            }
            
            // Copa más grande para zoom
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(x, terrain_height + tree_height + 1.0, z),
                3.0, // Copa mucho más grande
                Material::hojas()
            )));
        }
    }
    
    // Flores grandes para zoom extremo
    let flower_colors = [
        Vector3::new(1.0, 0.0, 0.0),    // Rojo
        Vector3::new(1.0, 1.0, 0.0),    // Amarillo
        Vector3::new(0.5, 0.0, 0.5),    // Púrpura
        Vector3::new(1.0, 0.5, 0.0),    // Naranja
        Vector3::new(0.0, 0.8, 1.0),    // Azul claro
    ];
    
    for _ in 0..15 {
        let x = rng.gen_range(1..(terrain_size-1)) as f64;
        let z = rng.gen_range(1..(terrain_size-1)) as f64;
        
        let terrain_height = ((x * 0.3).sin() + (z * 0.3).cos()) * 2.5
            + ((x * 0.5).sin() * (z * 0.5).cos()) * 1.5
            + 2.0;
        
        let flower_color = flower_colors[rng.gen_range(0..flower_colors.len())];
        
        // Tallo más grueso
        let tallo = Material::nuevo(
            Vector3::new(0.2, 0.8, 0.2), // Verde brillante
            0.0, 0.0, 1.0, 0.9, 0.0
        );
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(x, terrain_height + 0.5, z),
            0.2, // Tallo más grueso
            tallo
        )));
        
        // Flor mucho más grande para zoom
        let flor = Material::nuevo(flower_color, 0.0, 0.0, 1.0, 0.8, 0.0);
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(x, terrain_height + 1.0, z),
            0.8, // Flores ENORMES para zoom
            flor
        )));
    }
    
    // Configurar iluminación de atardecer
    for luz in crear_iluminacion_minecraft() {
        escena.agregar_luz(luz);
    }
    
    escena
}

fn calcular_color(rayo: &Rayo, escena: &Escena, profundidad: u32) -> Vector3<f64> {
    if profundidad == 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    
    if let Some(interseccion) = escena.intersectar(rayo) {
        let material = &interseccion.material;
        let punto = interseccion.punto;
        let normal = interseccion.normal;
        
        let mut color = material.albedo * 0.3;  // Color base ambiental
        
        // Aplicar iluminación desde todas las fuentes
        for luz in &escena.luces {
            let direccion_luz = (luz.posicion - punto).normalize();
            let intensidad = normal.dot(&direccion_luz).max(0.0);
            
            // Verificar sombras (simplificado)
            let rayo_sombra = Rayo::new(punto + normal * 0.001, direccion_luz);
            let distancia_luz = (luz.posicion - punto).magnitude();
            
            if !escena.hay_obstruccion(&rayo_sombra, distancia_luz) {
                color += luz.intensidad * intensidad * 0.7;
            }
        }
        
        // Reflexión simplificada
        if material.reflectividad > 0.0 && profundidad > 1 {
            let direccion_reflejada = reflejar(&rayo.direccion, &normal);
            let rayo_reflejado = Rayo::new(punto + normal * 0.001, direccion_reflejada);
            let color_reflejado = calcular_color(&rayo_reflejado, escena, profundidad - 1);
            color = color * (1.0 - material.reflectividad) + color_reflejado * material.reflectividad;
        }
        
        // Refracción simplificada
        if material.transparencia > 0.0 && profundidad > 1 {
            if let Some(direccion_refractada) = refractar(&rayo.direccion, &normal, material.indice_refraccion) {
                let rayo_refractado = Rayo::new(punto - normal * 0.001, direccion_refractada);
                let color_refractado = calcular_color(&rayo_refractado, escena, profundidad - 1);
                color = color * (1.0 - material.transparencia) + color_refractado * material.transparencia;
            }
        }
        
        color
    } else {
        // Cielo del atardecer
        let direccion_normalizada = rayo.direccion.normalize();
        let t = 0.5 * (direccion_normalizada.y + 1.0);
        
        let color_superior = Vector3::new(1.0, 0.51, 0.27);
        let color_inferior = Vector3::new(1.0, 0.75, 0.51);
        
        color_inferior * (1.0 - t) + color_superior * t
    }
}

fn reflejar(incidente: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    incidente - 2.0 * incidente.dot(normal) * normal
}

fn refractar(incidente: &Vector3<f64>, normal: &Vector3<f64>, indice: f64) -> Option<Vector3<f64>> {
    let cos_theta = (-incidente).dot(normal).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    
    if indice * sin_theta > 1.0 {
        return None;
    }
    
    let perpendicular = indice * (incidente + cos_theta * normal);
    let paralelo = -(1.0 - perpendicular.magnitude_squared()).abs().sqrt() * normal;
    
    Some(perpendicular + paralelo)
}