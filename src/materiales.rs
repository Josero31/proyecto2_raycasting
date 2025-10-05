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
    
    // Materiales predefinidos tipo Minecraft pero con nuestros propios colores
    
    /// Material tipo piedra - gris oscuro con textura rugosa
    pub fn piedra_lunar() -> Self {
        Self::nuevo(
            Vector3::new(0.4, 0.4, 0.5),  // Gris azulado
            0.1,                           // Ligeramente reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.8,                           // Muy rugoso
            0.2                            // Poco brillo
        )
    }
    
    /// Material tipo madera - café cálido
    pub fn madera_coral() -> Self {
        Self::nuevo(
            Vector3::new(0.8, 0.4, 0.3),  // Café rojizo coral
            0.05,                          // Muy poco reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.9,                           // Muy rugoso
            0.1                            // Muy poco brillo
        )
    }
    
    /// Material tipo metal - dorado brillante
    pub fn metal_dorado() -> Self {
        Self::nuevo(
            Vector3::new(0.9, 0.7, 0.2),  // Dorado brillante
            0.8,                           // Muy reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.1,                           // Muy suave
            0.9                            // Muy brillante
        )
    }
    
    /// Material tipo cristal - azul transparente
    pub fn cristal_aguamarina() -> Self {
        Self::nuevo(
            Vector3::new(0.2, 0.6, 0.8),  // Azul aguamarina
            0.2,                           // Algo reflectante
            0.7,                           // Muy transparente
            1.5,                           // Índice de refracción del vidrio
            0.0,                           // Muy suave
            0.8                            // Muy brillante
        )
    }
    
    /// Material tipo césped - verde vibrante
    pub fn cesped_esmeralda() -> Self {
        Self::nuevo(
            Vector3::new(0.2, 0.8, 0.3),  // Verde esmeralda
            0.0,                           // No reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.7,                           // Rugoso
            0.1                            // Poco brillo
        )
    }
    
    /// Material tipo agua - azul translúcido
    pub fn agua_cristalina() -> Self {
        Self::nuevo(
            Vector3::new(0.3, 0.5, 0.9),  // Azul cristalino
            0.3,                           // Reflectante como el agua
            0.6,                           // Semi-transparente
            1.33,                          // Índice de refracción del agua
            0.0,                           // Muy suave
            0.7                            // Brillante
        )
    }
    
    /// Material tipo lava - rojo brillante con emisión
    pub fn lava_magmatica() -> Self {
        Self::nuevo(
            Vector3::new(1.0, 0.3, 0.1),  // Rojo incandescente
            0.1,                           // Poco reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.6,                           // Rugoso
            0.8                            // Muy brillante (simula emisión)
        )
    }
    
    /// Material para el suelo - tierra café
    pub fn tierra_fertil() -> Self {
        Self::nuevo(
            Vector3::new(0.3, 0.2, 0.1),  // Café tierra
            0.0,                           // No reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            1.0,                           // Muy rugoso
            0.0                            // Sin brillo
        )
    }
}