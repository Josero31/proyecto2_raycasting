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
    vec![
        // Luz principal - sol artificial
        Luz::direccional(
            Vector3::new(-0.5, -1.0, -0.3).normalize(),
            Vector3::new(1.0, 0.95, 0.8),  // Luz cálida del sol
            0.8
        ),
        
        // Luz de relleno - azul suave
        Luz::puntual(
            Point3::new(5.0, 8.0, 10.0),
            Vector3::new(0.6, 0.7, 1.0),   // Azul cielo
            0.4
        ),
        
        // Luz ambiental - muy suave
        Luz::ambiental(
            Vector3::new(0.8, 0.9, 1.0),   // Azul muy claro
            0.1
        ),
        
        // Luz de acento - naranja cálida
        Luz::puntual(
            Point3::new(-3.0, 4.0, 8.0),
            Vector3::new(1.0, 0.6, 0.2),   // Naranja cálido
            0.3
        ),
    ]
}