use nalgebra::{Vector3, Point3};
use image::{RgbImage, Rgb};
use rayon::prelude::*;
use std::sync::Arc;

mod geometria;
mod materiales;
mod camara;
mod iluminacion;
mod escena;
mod texturas;

use geometria::*;
use camara::*;
use escena::*;
use texturas::*;

const ANCHO: u32 = 600;  // Resolución reducida para velocidad
const ALTO: u32 = 450;   // Mantiene aspecto 4:3
const MUESTRAS_ANTIALIASING: u32 = 2;  // Ultra-optimizado: 2x más rápido
const PROFUNDIDAD_MAXIMA: u32 = 3;    // Ultra-optimizado: mucho más rápido

fn main() {
    println!("🎨 Iniciando renderizado del diorama...");
    
    // Cargar texturas de Minecraft
    let _gestor_texturas = match GestorTexturas::cargar_texturas_minecraft() {
        Ok(gestor) => {
            println!("🖼️ Texturas cargadas exitosamente!");
            gestor
        },
        Err(e) => {
            println!("⚠️ Error cargando texturas: {}", e);
            println!("🔄 Continuando con colores sólidos...");
            GestorTexturas::nuevo()
        }
    };
    
    // Eliminar imagen anterior si existe
    if std::path::Path::new("diorama_renderizado.png").exists() {
        std::fs::remove_file("diorama_renderizado.png").ok();
        println!("🗑️ Imagen anterior eliminada");
    }
    
    // Configurar cámara PANORÁMICA para mostrar todo el paisaje
    let camera_distance = 80.0;   // Muy alejada para mostrar paisaje completo
    let camera_height = 50.0;     // Altura elevada para vista aérea
    let camera_angle = 35.0f64.to_radians(); // Ángulo para ver más paisaje
    
    let camera_x = camera_distance * camera_angle.cos();
    let camera_z = camera_distance * camera_angle.sin();
    
    let camara = Camara::nueva(
        Point3::new(camera_x, camera_height, camera_z),   // Posición panorámica lejana
        Point3::new(0.0, 5.0, 0.0),                      // Mirando un poco arriba del centro
        Vector3::new(0.0, 1.0, 0.0),                     // arriba
        60.0,                                             // Campo de visión amplio para panorámica
        ANCHO as f64 / ALTO as f64                        // aspecto
    );
    
    // Crear la escena que SÍ existe
    let escena = Arc::new(crear_escena_minecraft_simple());
    let camara = Arc::new(camara);
    let gestor_texturas = Arc::new(_gestor_texturas);
    
    // Crear imagen para el renderizado
    let mut imagen = RgbImage::new(ANCHO, ALTO);
    
    // Renderizado paralelo ULTRA-OPTIMIZADO por chunks grandes
    let pixels: Vec<_> = (0..ALTO).into_par_iter().flat_map(|y| {
        let escena_ref = Arc::clone(&escena);
        let camara_ref = Arc::clone(&camara);
        let gestor_texturas_ref = Arc::clone(&gestor_texturas);
        
        if y % 50 == 0 {
            println!("Procesando línea {} de {} (ULTRA-RÁPIDO)", y, ALTO);
        }
        
        (0..ANCHO).into_par_iter().map(move |x| {
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            
            // Anti-aliasing OPTIMIZADO con menos muestras
            for _ in 0..MUESTRAS_ANTIALIASING {
                let u = (x as f64 + rand::random::<f64>()) / ANCHO as f64;
                let v = (y as f64 + rand::random::<f64>()) / ALTO as f64;
                
                let rayo = camara_ref.obtener_rayo(u, v);
                color += calcular_color(&rayo, &escena_ref, &gestor_texturas_ref, PROFUNDIDAD_MAXIMA);
            }
            
            // Promedio y corrección gamma
            color /= MUESTRAS_ANTIALIASING as f64;
            color = Vector3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
            
            
            // Convertir a RGB [0, 255]
            let r = (255.0 * color.x.clamp(0.0, 1.0)) as u8;
            let g = (255.0 * color.y.clamp(0.0, 1.0)) as u8;
            let b = (255.0 * color.z.clamp(0.0, 1.0)) as u8;
            
            (x, y, [r, g, b])
        }).collect::<Vec<_>>()
    }).collect();
    
    // Aplicar pixels a la imagen
    for (x, y, color) in pixels {
        imagen.put_pixel(x, ALTO - y - 1, Rgb(color));
    }
    
    // Guardar la imagen
    match imagen.save("diorama_renderizado.png") {
        Ok(_) => println!("✅ Imagen guardada como 'diorama_renderizado.png'"),
        Err(e) => println!("❌ Error al guardar imagen: {}", e),
    }
    
    println!("🎉 Renderizado completado!");
}

fn calcular_color(rayo: &Rayo, escena: &Escena, gestor_texturas: &GestorTexturas, profundidad: u32) -> Vector3<f64> {
    if profundidad == 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    
    if let Some(interseccion) = escena.intersectar(rayo) {
        let material = &interseccion.material;
        let punto = interseccion.punto;
        let normal = interseccion.normal;
        
        // Color del material OPTIMIZADO - menos cálculos
        let mut color = if let Some(ref textura_nombre) = material.textura_nombre {
            if let Some(textura) = gestor_texturas.obtener_textura(textura_nombre) {
                // UV simplificado para velocidad
                let u = (punto.x * 0.05).fract().abs(); 
                let v = (punto.z * 0.05).fract().abs(); 
                
                // Usar más albedo para colores verdes brillantes
                let color_textura = textura.sample(u, v);
                color_textura * 0.3 + material.albedo * 0.7
            } else {
                material.albedo
            }
        } else {
            material.albedo
        };
        
        // Aplicar iluminación OPTIMIZADA (solo luz principal)
        if let Some(luz) = escena.luces.first() {
            let direccion_luz = (luz.posicion - punto).normalize();
            let intensidad = normal.dot(&direccion_luz).max(0.0);
            
            // Sin verificación de sombras para máxima velocidad
            color += luz.intensidad * intensidad * 0.5; // Reducir intensidad para compensar
        }
        
        // Reflexión
        if material.reflectividad > 0.0 {
            let direccion_reflejada = reflejar(&rayo.direccion, &normal);
            let rayo_reflejado = Rayo::new(punto + normal * 0.001, direccion_reflejada);
            let color_reflejado = calcular_color(&rayo_reflejado, escena, gestor_texturas, profundidad - 1);
            color = color * (1.0 - material.reflectividad) + color_reflejado * material.reflectividad;
        }
        
        // Refracción (para materiales transparentes)
        if material.transparencia > 0.0 {
            if let Some(direccion_refractada) = refractar(&rayo.direccion, &normal, material.indice_refraccion) {
                let rayo_refractado = Rayo::new(punto - normal * 0.001, direccion_refractada);
                let color_refractado = calcular_color(&rayo_refractado, escena, gestor_texturas, profundidad - 1);
                color = color * (1.0 - material.transparencia) + color_refractado * material.transparencia;
            }
        }
        
        color
    } else {
        // Cielo azul brillante estilo Minecraft
        let direccion_normalizada = rayo.direccion.normalize();
        let t = 0.5 * (direccion_normalizada.y + 1.0);
        
        // Cielo azul Minecraft - azul brillante arriba, más claro hacia el horizonte
        let color_superior = Vector3::new(0.4, 0.7, 1.0);    // Azul cielo brillante
        let color_inferior = Vector3::new(0.7, 0.9, 1.0);    // Azul muy claro cerca del horizonte
        
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