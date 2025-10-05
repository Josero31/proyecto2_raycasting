use nalgebra::{Vector3, Point3, Unit};
use image::{RgbImage, Rgb};
use std::f64::consts::PI;

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

const ANCHO: u32 = 800;
const ALTO: u32 = 600;
const MUESTRAS_ANTIALIASING: u32 = 16;
const PROFUNDIDAD_MAXIMA: u32 = 10;

fn main() {
    println!("🎨 Iniciando renderizado del diorama...");
    
    // Eliminar imagen anterior si existe
    if std::path::Path::new("diorama_renderizado.png").exists() {
        std::fs::remove_file("diorama_renderizado.png").ok();
        println!("🗑️ Imagen anterior eliminada");
    }
    
    // Configurar cámara con ZOOM GRANDÍSIMO para ver detalles
    let terrain_size = 50.0;
    let camara = Camara::nueva(
        Point3::new(terrain_size/2.0 + 8.0, 12.0, terrain_size/2.0 + 8.0), // MUY CERCA con zoom extremo
        Point3::new(terrain_size/2.0, 3.0, terrain_size/2.0),               // Mirando al centro cerca
        Vector3::new(0.0, 1.0, 0.0),                                        // arriba
        45.0,                                                                // Campo de visión normal para zoom
        ANCHO as f64 / ALTO as f64                                           // aspecto
    );
    
    // Crear la escena del diorama
    let escena = crear_diorama();
    
    // Crear imagen para el renderizado
    let mut imagen = RgbImage::new(ANCHO, ALTO);
    
    // Renderizar cada pixel
    for y in 0..ALTO {
        if y % 50 == 0 {
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
    match imagen.save("diorama_renderizado.png") {
        Ok(_) => println!("✅ Imagen guardada como 'diorama_renderizado.png'"),
        Err(e) => println!("❌ Error al guardar imagen: {}", e),
    }
    
    println!("🎉 Renderizado completado!");
}

fn calcular_color(rayo: &Rayo, escena: &Escena, profundidad: u32) -> Vector3<f64> {
    if profundidad == 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    
    if let Some(interseccion) = escena.intersectar(rayo) {
        let material = &interseccion.material;
        let punto = interseccion.punto;
        let normal = interseccion.normal;
        
        // Color del material
        let mut color = material.albedo;
        
        // Aplicar iluminación
        for luz in &escena.luces {
            let direccion_luz = (luz.posicion - punto).normalize();
            let intensidad = normal.dot(&direccion_luz).max(0.0);
            
            // Verificar sombras
            let rayo_sombra = Rayo::new(punto + normal * 0.001, direccion_luz);
            let distancia_luz = (luz.posicion - punto).magnitude();
            
            if !escena.hay_obstruccion(&rayo_sombra, distancia_luz) {
                color += luz.intensidad * intensidad;
            }
        }
        
        // Reflexión
        if material.reflectividad > 0.0 {
            let direccion_reflejada = reflejar(&rayo.direccion, &normal);
            let rayo_reflejado = Rayo::new(punto + normal * 0.001, direccion_reflejada);
            let color_reflejado = calcular_color(&rayo_reflejado, escena, profundidad - 1);
            color = color * (1.0 - material.reflectividad) + color_reflejado * material.reflectividad;
        }
        
        // Refracción (para materiales transparentes)
        if material.transparencia > 0.0 {
            if let Some(direccion_refractada) = refractar(&rayo.direccion, &normal, material.indice_refraccion) {
                let rayo_refractado = Rayo::new(punto - normal * 0.001, direccion_refractada);
                let color_refractado = calcular_color(&rayo_refractado, escena, profundidad - 1);
                color = color * (1.0 - material.transparencia) + color_refractado * material.transparencia;
            }
        }
        
        color
    } else {
        // Cielo del atardecer con degradado (igual que el original)
        let direccion_normalizada = rayo.direccion.normalize();
        let t = 0.5 * (direccion_normalizada.y + 1.0);
        
        // Colores exactos del atardecer del proyecto original
        let color_superior = Vector3::new(1.0, 0.51, 0.27);    // Naranja intenso arriba
        let color_inferior = Vector3::new(1.0, 0.75, 0.51);    // Naranja claro abajo
        
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
        return None; // Reflexión total interna
    }
    
    let perpendicular = indice * (incidente + cos_theta * normal);
    let paralelo = -(1.0 - perpendicular.magnitude_squared()).abs().sqrt() * normal;
    
    Some(perpendicular + paralelo)
}