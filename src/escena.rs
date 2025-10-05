use nalgebra::{Vector3, Point3};
use crate::geometria::*;
use crate::materiales::*;
use crate::iluminacion::*;

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
    
    let terrain_size = 50;
    let river_center_x = 20; // Río centrado como original
    let river_width = 4;     // Río del tamaño original
    
    // === TERRENO ONDULADO COMO EL ORIGINAL ===
    
    for x in 0..terrain_size {
        for z in 0..terrain_size {
            // Usar la misma fórmula de altura que el original
            let height = ((x as f64 * 0.2).sin() + (z as f64 * 0.2).cos()) * 2.5
                + ((x as f64 * 0.5).sin() * (z as f64 * 0.5).cos()) * 1.5
                + 2.0;
            
            let height_int = height.round() as i32;
            
            for y in 0..height_int {
                let material = if y == height_int - 1 {
                    Material::cesped()  // Capa superior de césped
                } else if y >= height_int - 2 {
                    Material::tierra()  // Tierra debajo del césped
                } else {
                    Material::piedra()  // Piedra en el fondo
                };
                
                escena.agregar_objeto(Box::new(Cubo::nuevo(
                    Point3::new(x as f64, y as f64 + 0.5, z as f64),
                    1.0,
                    material
                )));
            }
            
            // === RÍO (igual que el original) ===
            let dist_to_river = (x as i32 - river_center_x).abs();
            if dist_to_river < river_width {
                let water_depth = if dist_to_river == 0 { 1.5 } else { 1.0 };
                
                // Agua azul como el original
                escena.agregar_objeto(Box::new(Cubo::nuevo(
                    Point3::new(x as f64, -water_depth / 2.0, z as f64),
                    1.0,
                    Material::agua()
                )));
                
                // Arena en el fondo
                let arena = Material::nuevo(
                    Vector3::new(0.93, 0.84, 0.69), // Color arena
                    0.0, 0.0, 1.0, 0.9, 0.0
                );
                escena.agregar_objeto(Box::new(Cubo::nuevo(
                    Point3::new(x as f64, -water_depth - 0.3, z as f64),
                    1.0,
                    arena
                )));
            }
        }
    }
    
    // === ÁRBOLES MEJOR DISTRIBUIDOS ===
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Generar árboles con mejor distribución espacial
    let mut tree_positions: Vec<(f64, f64)> = Vec::new();
    let mut attempts = 0;
    
    while tree_positions.len() < 35 && attempts < 200 { // 35 árboles como original
        let x = rng.gen_range(2..(terrain_size-2)) as f64;
        let z = rng.gen_range(2..(terrain_size-2)) as f64;
        
        // Solo poner árboles fuera del río y con separación mínima
        if (x as i32) < river_center_x - river_width || (x as i32) > river_center_x + river_width {
            // Verificar separación mínima con otros árboles  
            let mut too_close = false;
            for &(tx, tz) in &tree_positions {
                let dist: f64 = ((x - tx).powi(2) + (z - tz).powi(2)).sqrt();
                if dist < 3.0 { // Separación mínima de 3 unidades
                    too_close = true;
                    break;
                }
            }
            
            if !too_close {
                tree_positions.push((x, z));
            }
        }
        attempts += 1;
    }
    
    // Crear los árboles en las posiciones calculadas
    for (x, z) in tree_positions {
        let tree_height = rng.gen_range(3..6) as f64; // Alturas como original
        let leaf_layers = rng.gen_range(3..5); // Capas como original
        
        // Calcular altura del terreno para este árbol
        let terrain_height = ((x * 0.2).sin() + (z * 0.2).cos()) * 2.5
            + ((x * 0.5).sin() * (z * 0.5).cos()) * 1.5
            + 2.0;
        
        // Tronco del árbol
        for i in 0..(tree_height as i32) {
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(x, terrain_height + i as f64 + 0.5, z),
                0.8,
                Material::madera_roble()
            )));
        }
        
        // Copa del árbol con mejor forma
        for y in 0..leaf_layers {
            let size = (leaf_layers - y) as f64 * 0.8 + 1.5; // Tamaño original
            let layer_y = terrain_height + tree_height + y as f64 * 0.8;
            
            // Decidir tipo de hoja aleatoriamente
            let leaf_material = match rng.gen_range(0..3) {
                0 => Material::hojas(),     // Oak - verde normal
                1 => Material::nuevo(      // Cherry - rosa
                    Vector3::new(1.0, 0.71, 0.76),
                    0.0, 0.0, 1.0, 0.9, 0.0
                ),
                _ => Material::nuevo(      // Birch - verde claro
                    Vector3::new(0.2, 0.8, 0.2),
                    0.0, 0.0, 1.0, 0.9, 0.0
                ),
            };
            
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(x, layer_y + 0.5, z),
                size,
                leaf_material
            )));
        }
        
        // Punta del árbol
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(x, terrain_height + tree_height + leaf_layers as f64 * 0.8 + 0.5, z),
            1.0,
            Material::hojas()
        )));
    }
    
    // === ROCAS Y ELEMENTOS DECORATIVOS GRANDES ===
    for _ in 0..30 {
        let x = rng.gen_range(0..terrain_size) as f64;
        let z = rng.gen_range(0..terrain_size) as f64;
        let size = rng.gen_range(8..20) as f64 / 10.0; // Rocas más grandes para vista panorámica
        
        let terrain_height = ((x * 0.3).sin() + (z * 0.3).cos()) * 2.5
            + ((x * 0.5).sin() * (z * 0.5).cos()) * 1.5
            + 2.0;
        
        let roca = Material::nuevo(
            Vector3::new(0.5, 0.5, 0.5), // Gris roca
            0.0, 0.0, 1.0, 0.9, 0.0
        );
        
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(x, terrain_height + size/2.0, z),
            size,
            roca
        )));
    }
    
    // === FLORES COLORIDAS ===
    let flower_colors = [
        Vector3::new(1.0, 0.0, 0.0),    // Rojo
        Vector3::new(1.0, 1.0, 0.0),    // Amarillo
        Vector3::new(0.5, 0.0, 0.5),    // Púrpura
        Vector3::new(1.0, 0.65, 0.0),   // Naranja
        Vector3::new(1.0, 0.0, 1.0),    // Magenta
    ];
    
    for _ in 0..100 { // MUCHAS más flores para llenar el paisaje
        let x = rng.gen_range(1..(terrain_size-1)) as f64 + 0.3;
        let z = rng.gen_range(1..(terrain_size-1)) as f64 + 0.3;
        
        // Evitar flores en el río
        if (x as i32) < river_center_x - river_width || (x as i32) > river_center_x + river_width {
            let terrain_height = ((x * 0.3).sin() + (z * 0.3).cos()) * 2.5
                + ((x * 0.5).sin() * (z * 0.5).cos()) * 1.5
                + 2.0;
            
            let flower_color = flower_colors[rng.gen_range(0..flower_colors.len())];
            
            // Tallo verde más visible
            let tallo = Material::nuevo(
                Vector3::new(0.2, 0.8, 0.2), // Verde más brillante
                0.0, 0.0, 1.0, 0.9, 0.0
            );
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(x, terrain_height + 0.4, z),
                0.15, // Tallo más grueso
                tallo
            )));
            
            // Flor colorida MUY VISIBLE desde lejos
            let flor = Material::nuevo(flower_color, 0.0, 0.0, 1.0, 0.8, 0.0);
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(x, terrain_height + 1.2, z),
                0.6, // Flores MUY GRANDES para vista panorámica
                flor
            )));
        }
    }
    
    // === MONTAÑAS DE FONDO PARA PAISAJE COMPLETO ===
    for x in 0..terrain_size {
        for z in 0..terrain_size {
            // Crear montañas en los bordes
            if x == 0 || x == terrain_size-1 || z == 0 || z == terrain_size-1 {
                let mountain_height = rng.gen_range(8..15) as f64;
                
                for y in 0..(mountain_height as i32) {
                    let mountain_material = if y > mountain_height as i32 - 3 {
                        Material::nuevo(Vector3::new(0.9, 0.9, 0.9), 0.0, 0.0, 1.0, 0.9, 0.0) // Nieve
                    } else {
                        Material::piedra() // Roca
                    };
                    
                    escena.agregar_objeto(Box::new(Cubo::nuevo(
                        Point3::new(x as f64, y as f64 + 0.5, z as f64),
                        1.0,
                        mountain_material
                    )));
                }
            }
        }
    }
    
    // === LAGO EN UNA ESQUINA ===
    for x in 40..terrain_size {
        for z in 40..terrain_size {
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(x as f64, -0.5, z as f64),
                1.0,
                Material::agua()
            )));
        }
    }
    
    // Configurar iluminación de atardecer (sunset)
    for luz in crear_iluminacion_minecraft() {
        escena.agregar_luz(luz);
    }
    
    escena
}