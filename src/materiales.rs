use nalgebra::Vector3;

#[derive(Clone)]
pub struct Material {
    pub albedo: Vector3<f64>,          // Color base del material
    pub reflectividad: f64,            // 0.0 = mate, 1.0 = espejo perfecto
    pub transparencia: f64,            // 0.0 = opaco, 1.0 = completamente transparente
    pub indice_refraccion: f64,        // Índice de refracción para la transparencia
    pub rugosidad: f64,                // Para efectos de especular
    pub brillo: f64,                   // Intensidad del brillo especular
    pub textura_nombre: Option<String>, // Nombre de la textura a usar
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
            textura_nombre: None,
        }
    }
    
    pub fn con_textura(albedo: Vector3<f64>, reflectividad: f64, transparencia: f64, 
                       indice_refraccion: f64, rugosidad: f64, brillo: f64, 
                       textura: String) -> Self {
        Self {
            albedo,
            reflectividad,
            transparencia,
            indice_refraccion,
            rugosidad,
            brillo,
            textura_nombre: Some(textura),
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
    
    // NUEVOS MATERIALES COLORIDOS ESTILO MINECRAFT
    
    /// Césped claro - verde brillante
    pub fn cesped_claro() -> Self {
        Self::nuevo(
            Vector3::new(0.5, 1.0, 0.3),  // Verde claro brillante
            0.0, 0.0, 1.0, 0.8, 0.0
        )
    }
    
    /// Césped oscuro - verde profundo
    pub fn cesped_oscuro() -> Self {
        Self::nuevo(
            Vector3::new(0.15, 0.5, 0.1), // Verde oscuro
            0.0, 0.0, 1.0, 0.8, 0.0
        )
    }
    
    /// Piedra oscura para montañas
    pub fn piedra_oscura() -> Self {
        Self::nuevo(
            Vector3::new(0.3, 0.3, 0.3), // Gris piedra oscura
            0.0, 0.0, 1.0, 0.9, 0.0
        )
    }
    
    /// Granito rojizo para montañas
    pub fn granito() -> Self {
        Self::nuevo(
            Vector3::new(0.7, 0.4, 0.3),  // Granito rojizo
            0.0, 0.0, 1.0, 0.8, 0.0
        )
    }
    
    /// Tierra rica - más oscura
    pub fn tierra_rica() -> Self {
        Self::nuevo(
            Vector3::new(0.3, 0.2, 0.1), // Café tierra fértil
            0.0, 0.0, 1.0, 1.0, 0.0
        )
    }
    
    /// Arena dorada
    pub fn arena() -> Self {
        Self::nuevo(
            Vector3::new(0.9, 0.8, 0.5), // Amarillo arena
            0.0, 0.0, 1.0, 0.7, 0.0
        )
    }
    
    /// Hojas claras
    pub fn hojas_claras() -> Self {
        Self::nuevo(
            Vector3::new(0.4, 0.9, 0.2),  // Verde hojas claro
            0.0, 0.0, 1.0, 0.9, 0.0
        )
    }
    
    /// Hojas oscuras
    pub fn hojas_oscuras() -> Self {
        Self::nuevo(
            Vector3::new(0.1, 0.4, 0.05),  // Verde muy oscuro
            0.0, 0.0, 1.0, 0.9, 0.0
        )
    }
    
    /// Tronco claro
    pub fn tronco_claro() -> Self {
        Self::nuevo(
            Vector3::new(0.8, 0.6, 0.4),  // Madera clara
            0.0, 0.0, 1.0, 0.8, 0.0
        )
    }
    
    /// Tronco oscuro
    pub fn tronco_oscuro() -> Self {
        Self::nuevo(
            Vector3::new(0.4, 0.2, 0.1),  // Madera oscura
            0.0, 0.0, 1.0, 0.8, 0.0
        )
    }
    
    // FLORES SÚPER COLORIDAS
    
    /// Flor roja vibrante
    pub fn flor_roja() -> Self {
        Self::nuevo(
            Vector3::new(1.0, 0.1, 0.1),  // Rojo vibrante
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    /// Flor amarilla pura
    pub fn flor_amarilla() -> Self {
        Self::nuevo(
            Vector3::new(1.0, 1.0, 0.0),  // Amarillo puro
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    /// Flor azul vibrante
    pub fn flor_azul() -> Self {
        Self::nuevo(
            Vector3::new(0.2, 0.4, 1.0),  // Azul vibrante
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    /// Flor rosa fucsia
    pub fn flor_rosa() -> Self {
        Self::nuevo(
            Vector3::new(1.0, 0.4, 0.8),  // Rosa fucsia
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    /// Flor púrpura brillante
    pub fn flor_purpura() -> Self {
        Self::nuevo(
            Vector3::new(0.7, 0.2, 1.0),  // Púrpura brillante
            0.0, 0.0, 1.0, 0.1, 0.0
        )
    }
    
    // === MATERIALES CON TEXTURAS MINECRAFT ===
    
    /// Pasto con textura de Minecraft
    pub fn pasto_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(1.0, 1.0, 1.0),  // Blanco para no alterar la textura
            0.0, 0.0, 1.0, 0.9, 0.1,
            "pasto".to_string()
        )
    }
    
    /// Agua con textura de Minecraft
    pub fn agua_texturizada() -> Self {
        Self::con_textura(
            Vector3::new(1.0, 1.0, 1.0),  // Blanco para no alterar la textura
            0.3, 0.1, 1.1, 0.0, 0.8,
            "agua".to_string()
        )
    }
    
    /// Tierra/montañas con textura de Minecraft
    pub fn dirt_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.6, 0.4, 0.2),  // Café tierra INTENSO
            0.05, 0.0, 1.0, 0.9, 0.1,
            "dirt".to_string()
        )
    }
    
    // =================== NUEVOS MATERIALES TEXTURIZADOS ===================
    
    /// Parte superior del pasto (grass_top)
    pub fn grass_top_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.2, 1.0, 0.3),  // Verde pasto BRILLANTE
            0.0, 0.0, 1.0, 0.8, 0.1,
            "grass_top".to_string()
        )
    }
    
    /// Lado del pasto (grass_side)
    pub fn grass_side_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.3, 0.9, 0.2),  // Verde pasto lateral
            0.0, 0.0, 1.0, 0.8, 0.1,
            "grass_side".to_string()
        )
    }
    
    /// Piedra texturizada
    pub fn stone_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.5, 0.5, 0.5),  // Gris piedra
            0.05, 0.0, 1.0, 0.9, 0.1,
            "stone".to_string()
        )
    }
    
    /// Adoquín texturizado
    pub fn cobblestone_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.4, 0.4, 0.4),  // Gris adoquín
            0.1, 0.0, 1.0, 0.9, 0.1,
            "cobblestone".to_string()
        )
    }
    
    /// Grava texturizada
    pub fn gravel_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.3, 0.3, 0.3),  // Gris grava oscuro
            0.05, 0.0, 1.0, 0.9, 0.1,
            "gravel".to_string()
        )
    }
    
    /// Agua quieta texturizada
    pub fn water_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(1.0, 1.0, 1.0),
            0.3, 0.1, 1.1, 0.0, 0.8,
            "water".to_string()
        )
    }
    
    // ÁRBOLES - ROBLE
    pub fn oak_log_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.5, 0.3, 0.1),  // Café tronco INTENSO
            0.0, 0.0, 1.0, 0.8, 0.1,
            "oak_log".to_string()
        )
    }
    
    pub fn oak_leaves_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.1, 0.9, 0.2),  // Verde hojas BRILLANTE
            0.0, 0.0, 1.0, 0.7, 0.2,
            "oak_leaves".to_string()
        )
    }
    
    // ÁRBOLES - ABEDUL
    pub fn birch_log_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.8, 0.7, 0.5),  // Café claro abedul
            0.1, 0.0, 1.0, 0.8, 0.1,
            "birch_log".to_string()
        )
    }
    
    pub fn birch_leaves_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.2, 0.8, 0.3),  // Verde hojas abedul
            0.0, 0.0, 1.0, 0.7, 0.2,
            "birch_leaves".to_string()
        )
    }
    
    // ÁRBOLES - JUNGLA
    pub fn jungle_log_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.4, 0.2, 0.1),  // Café oscuro jungla
            0.0, 0.0, 1.0, 0.8, 0.1,
            "jungle_log".to_string()
        )
    }
    
    pub fn jungle_leaves_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(0.0, 0.9, 0.1),  // Verde hojas jungla INTENSO
            0.0, 0.0, 1.0, 0.7, 0.2,
            "jungle_leaves".to_string()
        )
    }
    
    // MINERALES
    pub fn coal_ore_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(1.0, 1.0, 1.0),
            0.05, 0.0, 1.0, 0.9, 0.1,
            "coal_ore".to_string()
        )
    }
    
    pub fn iron_ore_texturizado() -> Self {
        Self::con_textura(
            Vector3::new(1.0, 1.0, 1.0),
            0.2, 0.0, 1.0, 0.8, 0.3,
            "iron_ore".to_string()
        )
    }
}