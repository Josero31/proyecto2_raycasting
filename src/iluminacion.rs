use nalgebra::{Vector3, Point3};

#[derive(Clone)]
pub struct Luz {
    pub posicion: Point3<f64>,
    pub intensidad: Vector3<f64>,
    pub tipo_luz: TipoLuz,
}

#[derive(Clone)]
pub enum TipoLuz {
    Puntual,      // Luz que emite en todas las direcciones
    Direccional,  // Luz que viene de una dirección específica (como el sol)
    Ambiental,    // Luz uniforme en toda la escena
}

impl Luz {
    pub fn puntual(posicion: Point3<f64>, color: Vector3<f64>, intensidad: f64) -> Self {
        Self {
            posicion,
            intensidad: color * intensidad,
            tipo_luz: TipoLuz::Puntual,
        }
    }
    
    pub fn direccional(direccion: Vector3<f64>, color: Vector3<f64>, intensidad: f64) -> Self {
        // Para luz direccional, usamos la posición como dirección
        Self {
            posicion: Point3::new(direccion.x, direccion.y, direccion.z),
            intensidad: color * intensidad,
            tipo_luz: TipoLuz::Direccional,
        }
    }
    
    pub fn ambiental(color: Vector3<f64>, intensidad: f64) -> Self {
        Self {
            posicion: Point3::new(0.0, 0.0, 0.0),
            intensidad: color * intensidad,
            tipo_luz: TipoLuz::Ambiental,
        }
    }
}

// Configuración de iluminación para nuestro diorama
pub fn crear_iluminacion_diorama() -> Vec<Luz> {
    crear_iluminacion_minecraft()
}

// Iluminación estilo atardecer como el proyecto original
pub fn crear_iluminacion_minecraft() -> Vec<Luz> {
    // Replicar la iluminación de sunset del original
    
    // Sol en ángulo de 40 grados (sunset) como el original
    let sun_angle = 40.0f64.to_radians();
    let sun_azimuth = 220.0f64.to_radians(); // Dirección suroeste
    
    // Posición del sol (igual que el original)
    let light_pos = Point3::new(
        25.0 + 80.0 * sun_azimuth.cos() * sun_angle.cos(),
        80.0 * sun_angle.sin(),
        25.0 + 80.0 * sun_azimuth.sin() * sun_angle.cos(),
    );
    
    vec![
        // Sol principal de atardecer (igual colores que el original)
        Luz::puntual(
            light_pos,
            Vector3::new(1.0, 0.65, 0.35),  // Colores cálidos de sunset
            1.2
        ),
        
        // Luz ambiental de atardecer
        Luz::ambiental(
            Vector3::new(1.0, 0.7, 0.4),    // Ambiente cálido
            0.4
        ),
        
        // Luz de relleno para suavizar sombras
        Luz::puntual(
            Point3::new(0.0, 60.0, 0.0),
            Vector3::new(0.8, 0.6, 0.4),    // Luz cenital suave
            0.3
        ),
    ]
}