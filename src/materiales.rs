use nalgebra::Vector3;

#[derive(Clone)]
pub struct Material {
    pub albedo: Vector3<f64>,          // Color base del material
    pub reflectividad: f64,            // 0.0 = mate, 1.0 = espejo perfecto
    pub transparencia: f64,            // 0.0 = opaco, 1.0 = completamente transparente
    pub indice_refraccion: f64,        // Índice de refracción para la transparencia
    pub rugosidad: f64,                // Para efectos de especular
    pub brillo: f64,                   // Intensidad del brillo especular
}

impl Material {
    pub fn nuevo(albedo: Vector3<f64>, reflectividad: f64, transparencia: f64, 
                 indice_refraccion: f64, rugosidad: f64, brillo: f64) -> Self {
        Self {
            albedo,
            reflectividad,
            transparencia,
            indice_refraccion,
            rugosidad,
            brillo,
        }
    }
    
    // Materiales estilo Minecraft con colores reconocibles
    
    /// Bloque de piedra - gris clásico de Minecraft
    pub fn piedra() -> Self {
        Self::nuevo(
            Vector3::new(0.5, 0.5, 0.5),  // Gris piedra clásico
            0.0,                           // No reflectante (mate)
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.9,                           // Muy rugoso
            0.0                            // Sin brillo
        )
    }
    
    /// Madera de roble - café característico
    pub fn madera_roble() -> Self {
        Self::nuevo(
            Vector3::new(0.6, 0.4, 0.2),  // Café madera roble
            0.0,                           // No reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.8,                           // Rugoso
            0.0                            // Sin brillo
        )
    }
    
    /// Bloque de oro - amarillo brillante
    pub fn oro() -> Self {
        Self::nuevo(
            Vector3::new(1.0, 0.8, 0.0),  // Amarillo oro intenso
            0.6,                           // Reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.2,                           // Suave
            0.7                            // Brillante
        )
    }
    
    /// Vidrio - transparente azulado
    pub fn vidrio() -> Self {
        Self::nuevo(
            Vector3::new(0.8, 0.9, 1.0),  // Azul muy claro
            0.1,                           // Poco reflectante
            0.8,                           // Muy transparente
            1.5,                           // Índice de refracción del vidrio
            0.0,                           // Muy suave
            0.9                            // Muy brillante
        )
    }
    
    /// Césped - verde Minecraft característico
    pub fn cesped() -> Self {
        Self::nuevo(
            Vector3::new(0.3, 0.7, 0.2),  // Verde césped Minecraft
            0.0,                           // No reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.8,                           // Rugoso
            0.0                            // Sin brillo
        )
    }
    
    /// Agua - azul clásico de Minecraft
    pub fn agua() -> Self {
        Self::nuevo(
            Vector3::new(0.2, 0.4, 0.8),  // Azul agua Minecraft
            0.2,                           // Algo reflectante
            0.7,                           // Transparente
            1.33,                          // Índice de refracción del agua
            0.0,                           // Suave
            0.5                            // Algo brillante
        )
    }
    
    /// Lava - naranja-rojo intenso
    pub fn lava() -> Self {
        Self::nuevo(
            Vector3::new(1.0, 0.4, 0.0),  // Naranja-rojo lava
            0.0,                           // No reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.8,                           // Rugoso
            1.0                            // Muy brillante (emisivo)
        )
    }
    
    /// Tierra - café oscuro
    pub fn tierra() -> Self {
        Self::nuevo(
            Vector3::new(0.4, 0.3, 0.2),  // Café tierra
            0.0,                           // No reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            1.0,                           // Muy rugoso
            0.0                            // Sin brillo
        )
    }
    
    /// Adoquín - gris oscuro texturizado
    pub fn adoquin() -> Self {
        Self::nuevo(
            Vector3::new(0.4, 0.4, 0.4),  // Gris adoquín
            0.0,                           // No reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.9,                           // Muy rugoso
            0.0                            // Sin brillo
        )
    }
    
    /// Hojas - verde más oscuro que el césped
    pub fn hojas() -> Self {
        Self::nuevo(
            Vector3::new(0.2, 0.6, 0.1),  // Verde hojas más oscuro
            0.0,                           // No reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.9,                           // Muy rugoso
            0.0                            // Sin brillo
        )
    }
}