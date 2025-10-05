use nalgebra::{Vector3, Point3};
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

const ANCHO: u32 = 400;  // Resolución más baja para generar múltiples vistas rápidamente
const ALTO: u32 = 300;
const MUESTRAS_ANTIALIASING: u32 = 4;  // Menos muestras para mayor velocidad
const PROFUNDIDAD_MAXIMA: u32 = 8;

fn main() {
    println!("🎬 Generando secuencia de vistas del diorama...");
    
    let escena = crear_diorama();
    let num_frames = 8;  // Número de vistas diferentes
    
    for frame in 0..num_frames {
        println!("Renderizando vista {} de {}", frame + 1, num_frames);
        
        // Calcular posición de cámara orbital
        let angulo = (frame as f64 / num_frames as f64) * 2.0 * PI;
        let radio = 18.0;
        let altura = 8.0;
        
        let pos_x = angulo.cos() * radio;
        let pos_z = angulo.sin() * radio;
        
        let mut camara = Camara::nueva(
            Point3::new(pos_x, altura, pos_z),
            Point3::new(0.0, 2.0, 0.0),      // Mirar hacia el centro del castillo
            Vector3::new(0.0, 1.0, 0.0),
            45.0,
            ANCHO as f64 / ALTO as f64
        );
        
        // Renderizar la imagen
        let mut imagen = RgbImage::new(ANCHO, ALTO);
        
        for y in 0..ALTO {
            for x in 0..ANCHO {
                let mut color = Vector3::new(0.0, 0.0, 0.0);
                
                for _ in 0..MUESTRAS_ANTIALIASING {
                    let u = (x as f64 + rand::random::<f64>()) / ANCHO as f64;
                    let v = (y as f64 + rand::random::<f64>()) / ALTO as f64;
                    
                    let rayo = camara.obtener_rayo(u, v);
                    color += calcular_color(&rayo, &escena, PROFUNDIDAD_MAXIMA);
                }
                
                color /= MUESTRAS_ANTIALIASING as f64;
                color = Vector3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
                
                let r = (255.0 * color.x.clamp(0.0, 1.0)) as u8;
                let g = (255.0 * color.y.clamp(0.0, 1.0)) as u8;
                let b = (255.0 * color.z.clamp(0.0, 1.0)) as u8;
                
                imagen.put_pixel(x, ALTO - y - 1, Rgb([r, g, b]));
            }
        }
        
        // Guardar cada vista
        let nombre_archivo = format!("diorama_vista_{:02}.png", frame);
        match imagen.save(&nombre_archivo) {
            Ok(_) => println!("✅ Vista guardada como '{}'", nombre_archivo),
            Err(e) => println!("❌ Error al guardar {}: {}", nombre_archivo, e),
        }
    }
    
    println!("🎉 Secuencia completada! Se generaron {} vistas diferentes.", num_frames);
    println!("💡 Puedes usar estas imágenes para crear un GIF o video del diorama.");
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
            
            // Verificar sombras
            let rayo_sombra = Rayo::new(punto + normal * 0.001, direccion_luz);
            let distancia_luz = (luz.posicion - punto).magnitude();
            
            if !escena.hay_obstruccion(&rayo_sombra, distancia_luz) {
                color += luz.intensidad * intensidad * 0.7;
            }
        }
        
        // Reflexión
        if material.reflectividad > 0.0 {
            let direccion_reflejada = reflejar(&rayo.direccion, &normal);
            let rayo_reflejado = Rayo::new(punto + normal * 0.001, direccion_reflejada);
            let color_reflejado = calcular_color(&rayo_reflejado, escena, profundidad - 1);
            color = color * (1.0 - material.reflectividad) + color_reflejado * material.reflectividad;
        }
        
        // Refracción
        if material.transparencia > 0.0 {
            if let Some(direccion_refractada) = refractar(&rayo.direccion, &normal, material.indice_refraccion) {
                let rayo_refractado = Rayo::new(punto - normal * 0.001, direccion_refractada);
                let color_refractado = calcular_color(&rayo_refractado, escena, profundidad - 1);
                color = color * (1.0 - material.transparencia) + color_refractado * material.transparencia;
            }
        }
        
        color
    } else {
        // Skybox mejorado - gradiente más dramático
        let direccion_normalizada = rayo.direccion.normalize();
        let t = 0.5 * (direccion_normalizada.y + 1.0);
        
        // Colores del amanecer/atardecer
        let color_horizonte = Vector3::new(1.0, 0.7, 0.4);  // Naranja cálido
        let color_cenital = Vector3::new(0.3, 0.5, 0.9);    // Azul profundo
        
        color_horizonte * (1.0 - t) + color_cenital * t
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