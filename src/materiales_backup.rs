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

unsafe impl Send for Material {}
unsafe impl Sync for Material {}

impl Material {
            1.0,                           // Sin refracción
            0.8,                           // Rugoso
            0.0                            // Sin brillo
        )
    }3<f64>,          // Color base del material
    pub reflectividad: f64,            // 0.0 = mate, 1.0 = espejo perfecto
    pub transparencia: f64,            // 0.0 = opaco, 1.0 = completamente transparente
    pub indice_refraccion: f64,        // Índice de refracción para la transparencia
    pub rugosidad: f64,                // Para efectos de especular
    pub brillo: f64,                   // Intensidad del brillo especular
}

unsafe impl Send for Material {}
unsafe impl Sync for Material {}

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
    
    /// Agua - azul cristalino vibrante
    pub fn agua() -> Self {
        Self::nuevo(
            Vector3::new(0.1, 0.5, 1.0),  // Azul agua cristalina vibrante
            0.3,                           // Más reflectante
            0.8,                           // Muy transparente
            1.33,                          // Índice de refracción del agua
            0.0,                           // Suave
            0.8                            // Muy brillante
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
    
    /// Tierra - café rico
    pub fn tierra() -> Self {
        Self::nuevo(
            Vector3::new(0.6, 0.4, 0.2),  // Café tierra más claro
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
    
    /// Hojas - verde vibrante
    pub fn hojas() -> Self {
        Self::nuevo(
            Vector3::new(0.1, 0.8, 0.0),  // Verde hojas vibrante
            0.0,                           // No reflectante
            0.0,                           // Opaco
            1.0,                           // Sin refracción
            0.9,                           // Muy rugoso
            0.0                            // Sin brillo
        )
    }
    
    // NUEVOS MATERIALES COLORIDOS ESTILO MINECRAFT
    
    /// Tierra rica - más oscura y fértil
    pub fn tierra_rica() -> Self {
        Self::nuevo(
            Vector3::new(0.4, 0.25, 0.1), // Café tierra fértil
            0.0, 0.0, 1.0, 1.0, 0.0
        )
    }
    
    /// Arena - amarillo brillante
    pub fn arena() -> Self {
        Self::nuevo(
            Vector3::new(0.95, 0.9, 0.6), // Amarillo arena
            0.0, 0.0, 1.0, 0.7, 0.0
        )
    }
    
    /// Piedra oscura para montañas
    pub fn piedra_oscura() -> Self {
        Self::nuevo(
            Vector3::new(0.3, 0.3, 0.35), // Gris piedra oscura
            0.0, 0.0, 1.0, 0.9, 0.0
        )
    }
    
    /// Granito rojizo
    pub fn granito() -> Self {
        Self::nuevo(
            Vector3::new(0.7, 0.4, 0.3),  // Granito rojizo
            0.0, 0.0, 1.0, 0.8, 0.0
        )
    }
    
    /// Flores coloridas
    pub fn flor_roja() -> Self {
        Self::nuevo(
            Vector3::new(1.0, 0.1, 0.1),  // Rojo vibrante
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    pub fn flor_amarilla() -> Self {
        Self::nuevo(
            Vector3::new(1.0, 1.0, 0.0),  // Amarillo puro
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    pub fn flor_azul() -> Self {
        Self::nuevo(
            Vector3::new(0.2, 0.4, 1.0),  // Azul vibrante
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    pub fn flor_rosa() -> Self {
        Self::nuevo(
            Vector3::new(1.0, 0.4, 0.8),  // Rosa fucsia
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    pub fn flor_purpura() -> Self {
        Self::nuevo(
            Vector3::new(0.7, 0.2, 1.0),  // Púrpura brillante
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    /// Variaciones de césped
    pub fn cesped_claro() -> Self {
        Self::nuevo(
            Vector3::new(0.4, 1.0, 0.3),  // Verde claro brillante
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    pub fn cesped_oscuro() -> Self {
        Self::nuevo(
            Vector3::new(0.15, 0.6, 0.1), // Verde oscuro
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    /// Variaciones de hojas
    pub fn hojas_claras() -> Self {
        Self::nuevo(
            Vector3::new(0.3, 1.0, 0.2),  // Verde hojas claro
            0.0, 0.0, 1.0, 0.2, 0.0
        )
    }
    
    pub fn hojas_oscuras() -> Self {
        Self::nuevo(
            Vector3::new(0.0, 0.5, 0.0),  // Verde hojas muy oscuro
            0.0, 0.0, 1.0, 0.2, 0.0
        )
    }
    
    /// Tronco variado
    pub fn tronco_claro() -> Self {
        Self::nuevo(
            Vector3::new(0.8, 0.6, 0.4),  // Madera clara
            0.0, 0.0, 1.0, 0.4, 0.0
        )
    }
    
    pub fn tronco_oscuro() -> Self {
        Self::nuevo(
            Vector3::new(0.4, 0.2, 0.1),  // Madera oscura
            0.0, 0.0, 1.0, 0.4, 0.0
        )
    }
}