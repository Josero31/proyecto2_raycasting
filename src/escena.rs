use nalgebra::{Vector3, Point3};
use crate::geometria::*;
use crate::materiales::*;
use crate::iluminacion::*;
use rand::{thread_rng, Rng};

pub struct Escena {
    pub objetos: Vec<Box<dyn Figura>>,
    pub luces: Vec<Luz>,
}

unsafe impl Send for Escena {}
unsafe impl Sync for Escena {}

impl Escena {
    pub fn nueva() -> Self {
        Self {
            objetos: Vec::new(),
            luces: Vec::new(),
        }
    }
    
    pub fn agregar_objeto(&mut self, objeto: Box<dyn Figura>) {
        self.objetos.push(objeto);
    }
    
    pub fn agregar_luz(&mut self, luz: Luz) {
        self.luces.push(luz);
    }
    
    pub fn intersectar(&self, rayo: &Rayo) -> Option<Interseccion> {
        let mut interseccion_mas_cercana = None;
        let mut t_minimo = f64::INFINITY;
        
        for objeto in &self.objetos {
            if let Some(interseccion) = objeto.intersectar(rayo) {
                if interseccion.t < t_minimo {
                    t_minimo = interseccion.t;
                    interseccion_mas_cercana = Some(interseccion);
                }
            }
        }
        
        interseccion_mas_cercana
    }
    
    pub fn hay_obstruccion(&self, rayo: &Rayo, distancia_maxima: f64) -> bool {
        for objeto in &self.objetos {
            if let Some(interseccion) = objeto.intersectar(rayo) {
                if interseccion.t < distancia_maxima - 0.001 {
                    return true;
                }
            }
        }
        false
    }
}

// Función para crear un diorama EXACTAMENTE como el proyecto original
pub fn crear_diorama() -> Escena {
    let mut escena = Escena::nueva();
    let mut rng = thread_rng();
    
    let terrain_size = 20;  // Tamaño más pequeño para que todo sea visible
    
    // === TERRENO BASE CON TEXTURA DE PASTO MINECRAFT ===
    for x in 0..terrain_size {
        for z in 0..terrain_size {
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(x as f64, 0.0, z as f64),
                1.0,
                Material::pasto_texturizado()  // Pasto con textura real de Minecraft
            )));
        }
    }
    
    // === MONTAÑAS GRANDES Y VISIBLES ===
    // Crear montañas en lugares específicos donde la cámara las pueda ver
    let mountain_positions = vec![
        (2, 2, 8), (3, 2, 6), (4, 2, 7),   // Montañas frontales izquierda
        (15, 2, 9), (16, 2, 7), (17, 2, 8), // Montañas frontales derecha
        (2, 15, 6), (3, 16, 8), (4, 17, 7), // Montañas traseras izquierda
        (15, 15, 10), (16, 16, 8), (17, 17, 9), // Montañas traseras derecha
    ];
    
    for (mx, mz, height) in mountain_positions {
        // Crear cada montaña con múltiples bloques para hacerla visible
        for dx in -1..=1 {
            for dz in -1..=1 {
                for y in 1..=height {
                    let material = if y > height - 2 {
                        Material::nuevo(Vector3::new(0.9, 0.9, 0.9), 0.0, 0.0, 1.0, 0.9, 0.0) // Nieve
                    } else {
                        Material::dirt_texturizado() // Tierra con textura real de Minecraft
                    };
                    
                    let x_pos = (mx + dx).clamp(0, terrain_size-1);
                    let z_pos = (mz + dz).clamp(0, terrain_size-1);
                    
                    escena.agregar_objeto(Box::new(Cubo::nuevo(
                        Point3::new(x_pos as f64, y as f64, z_pos as f64),
                        1.0,
                        material
                    )));
                }
            }
        }
    }
    
    // === LAGO CON TEXTURA DE AGUA MINECRAFT ===
    for x in 8..14 {  // Centro del terreno
        for z in 8..14 {
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(x as f64, 0.5, z as f64),  // Ligeramente elevado
                1.0,
                Material::agua_texturizada()  // Agua con textura real de Minecraft
            )));
        }
    }
    
    // === ÁRBOLES GRANDES Y MUY VISIBLES ===
    let tree_positions = vec![
        (5, 5), (7, 16), (14, 4), (16, 12), (4, 10), (12, 15)
    ];
    
    for (x, z) in tree_positions {
        // Tronco del árbol - MÁS ALTO
        for y in 1..=6 {  // Troncos más altos
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(x as f64, y as f64, z as f64),
                1.0,
                Material::nuevo(Vector3::new(0.4, 0.2, 0.1), 0.0, 0.0, 1.0, 0.9, 0.0)  // Marrón oscuro
            )));
        }
        
        // Copa del árbol - MÁS GRANDE Y VISIBLE
        for dx in -2i32..=2i32 {
            for dz in -2i32..=2i32 {
                for dy in 0..=3 {  // Copa más alta
                    if (dx.abs() + dz.abs() + dy) <= 4 {  // Forma piramidal
                        escena.agregar_objeto(Box::new(Cubo::nuevo(
                            Point3::new((x as i32 + dx) as f64, (7 + dy) as f64, (z as i32 + dz) as f64),
                            1.0,
                            Material::nuevo(Vector3::new(0.1, 0.6, 0.1), 0.0, 0.0, 1.0, 0.9, 0.0)  // Verde oscuro
                        )));
                    }
                }
            }
        }
    }
    
    // === ALGUNAS FLORES COLORIDAS ===
    let flower_positions = vec![
        (3, 8), (9, 4), (13, 11), (17, 7), (20, 12), (4, 18), (8, 21), (15, 19)
    ];
    
    for (x, z) in flower_positions {
        let flower_material = match rng.gen_range(0..4) {
            0 => Material::flor_roja(),
            1 => Material::flor_amarilla(),
            2 => Material::flor_azul(),
            _ => Material::flor_rosa(),
        };
        
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(x as f64, 1.0, z as f64),
            0.8,
            flower_material
        )));
    }
    
    // Configurar iluminación
    for luz in crear_iluminacion_minecraft() {
        escena.agregar_luz(luz);
    }
    
    escena
}