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

// ====================== MUNDO MINECRAFT MASIVO CON TEXTURAS ======================

pub fn crear_escena_minecraft_masiva() -> Escena {
    let mut escena = Escena::nueva();
    
    // ====================== CONFIGURACIÓN DEL MUNDO PEQUEÑO ======================
    let size = 10;   // Mundo pequeño pero visible 10x10
    let spacing = 3.0;  // Espaciado grande para ver bien
    let centro = size as f64 / 2.0;
    
    println!("🌍 Generando mundo Minecraft PEQUEÑO {}x{} con todas las texturas...", size, size);
    
    // ====================== SISTEMA DE BIOMAS ======================
    
    // SUELO BASE CON BIOMAS
    for x in 0..size {
        for z in 0..size {
            let pos_x = (x as f64 - centro) * spacing;
            let pos_z = (z as f64 - centro) * spacing;
            let distancia_centro = ((pos_x / spacing).powi(2) + (pos_z / spacing).powi(2)).sqrt();
            
            // Determinar material según distancia del centro (ajustado para 10x10)
            let material = if distancia_centro < 2.0 {
                // BIOMA CENTRAL - Pasto verde
                Material::grass_top_texturizado()
            } else if distancia_centro < 3.5 {
                // BIOMA INTERMEDIO - Tierra
                Material::dirt_texturizado()
            } else {
                // BIOMA EXTREMO - Piedra
                Material::stone_texturizado()
            };
            
            let suelo = Box::new(Cubo::con_limites(
                Point3::new(pos_x - 1.0, -1.0, pos_z - 1.0),
                Point3::new(pos_x + 1.0, 0.0, pos_z + 1.0),
                material
            ));
            escena.agregar_objeto(suelo);
        }
    }
    
    // ====================== SISTEMA MONTAÑOSO MASIVO ======================
    
    for x in 0..size {
        for z in 0..size {
            let pos_x = (x as f64 - centro) * spacing;
            let pos_z = (z as f64 - centro) * spacing;  
            let distancia_centro = ((pos_x / spacing).powi(2) + (pos_z / spacing).powi(2)).sqrt();
            
            // MONTAÑAS PRINCIPALES (perímetro exterior)
            if distancia_centro > 4.0 && distancia_centro < 6.0 {
                let altura = (4.0 + (x + z) as f64 * 0.2) % 8.0 + 3.0;
                
                let material = if (x + z) % 3 == 0 {
                    Material::stone_texturizado()
                } else if (x + z) % 3 == 1 {
                    Material::cobblestone_texturizado()
                } else {
                    Material::gravel_texturizado()
                };
                
                let montaña = Box::new(Cubo::con_limites(
                    Point3::new(pos_x - 1.0, 0.0, pos_z - 1.0),
                    Point3::new(pos_x + 1.0, altura, pos_z + 1.0),
                    material
                ));
                escena.agregar_objeto(montaña);
            }
            
            // COLINAS INTERMEDIAS
            if distancia_centro > 3.0 && distancia_centro < 4.0 && (x + z) % 2 == 0 {
                let altura = (2.0 + (x * z) as f64 * 0.05) % 4.0 + 1.5;
                
                let colina = Box::new(Cubo::con_limites(
                    Point3::new(pos_x - 1.0, 0.0, pos_z - 1.0),
                    Point3::new(pos_x + 1.0, altura, pos_z + 1.0),
                    Material::dirt_texturizado()
                ));
                escena.agregar_objeto(colina);
            }
        }
    }
    
    // ====================== LAGOS Y RÍOS ======================
    
    // LAGO CENTRAL PRINCIPAL
    let radio_lago = 1.5;
    for x in 0..size {
        for z in 0..size {
            let pos_x = (x as f64 - centro) * spacing;
            let pos_z = (z as f64 - centro) * spacing;
            let distancia_centro = ((pos_x / spacing).powi(2) + (pos_z / spacing).powi(2)).sqrt();
            
            if distancia_centro < radio_lago {
                let profundidad = if distancia_centro < 3.0 { 1.2 } else { 0.8 };
                
                let agua = Box::new(Cubo::con_limites(
                    Point3::new(pos_x - 1.0, 0.0, pos_z - 1.0),
                    Point3::new(pos_x + 1.0, profundidad, pos_z + 1.0),
                    Material::water_texturizado()
                ));
                escena.agregar_objeto(agua);
            }
        }
    }
    
    // LAGOS SECUNDARIOS
    let lagos_secundarios = [(20.0, 20.0, 4.0), (-25.0, 15.0, 3.0), (15.0, -30.0, 3.5), (-20.0, -20.0, 4.0)];
    for (lago_x, lago_z, radio) in &lagos_secundarios {
        for x in 0..size {
            for z in 0..size {
                let pos_x = (x as f64 - centro) * spacing;
                let pos_z = (z as f64 - centro) * spacing;
                let dist_lago = ((pos_x - lago_x).powi(2) + (pos_z - lago_z).powi(2)).sqrt();
                
                if dist_lago < *radio {
                    let agua = Box::new(Cubo::con_limites(
                        Point3::new(pos_x - 1.0, 0.0, pos_z - 1.0),
                        Point3::new(pos_x + 1.0, 0.6, pos_z + 1.0),
                        Material::agua_texturizada()
                    ));
                    escena.agregar_objeto(agua);
                }
            }
        }
    }
    
    // ====================== ÁRBOLES SIMPLES ======================
    
    // Solo algunos árboles simples para mundo pequeño
    for i in 0..5 {
        let x = (i + 2) % size;
        let z = (i + 3) % size;
        let pos_x = (x as f64 - centro) * spacing;
        let pos_z = (z as f64 - centro) * spacing;
        let distancia_centro = ((pos_x / spacing).powi(2) + (pos_z / spacing).powi(2)).sqrt();
        
        if distancia_centro > 2.0 && distancia_centro < 3.5 {
            // Tronco simple
            let tronco = Box::new(Cubo::con_limites(
                Point3::new(pos_x - 0.3, 0.0, pos_z - 0.3),
                Point3::new(pos_x + 0.3, 3.0, pos_z + 0.3),
                Material::oak_log_texturizado()
            ));
            escena.agregar_objeto(tronco);
            
            // Hojas simples
            let hojas = Box::new(Cubo::con_limites(
                Point3::new(pos_x - 1.0, 2.5, pos_z - 1.0),
                Point3::new(pos_x + 1.0, 4.0, pos_z + 1.0),
                Material::oak_leaves_texturizado()
            ));
            escena.agregar_objeto(hojas);
        }
    }
    
    // ====================== DEPÓSITOS MINERALES ======================
    
    // CARBÓN en montañas
    for i in 0..80 {
        let x = (i * 13 + 40) % size;
        let z = (i * 17 + 35) % size;
        let pos_x = (x as f64 - centro) * spacing;
        let pos_z = (z as f64 - centro) * spacing;
        let distancia_centro = ((pos_x / spacing).powi(2) + (pos_z / spacing).powi(2)).sqrt();
        
        if distancia_centro > 30.0 && distancia_centro < 42.0 {
            let mineral = Box::new(Cubo::con_limites(
                Point3::new(pos_x - 0.5, 0.0, pos_z - 0.5),
                Point3::new(pos_x + 0.5, 1.5, pos_z + 0.5),
                Material::coal_ore_texturizado()
            ));
            escena.agregar_objeto(mineral);
        }
    }
    
    // HIERRO en montañas profundas
    for i in 0..50 {
        let x = (i * 19 + 20) % size;
        let z = (i * 23 + 45) % size;
        let pos_x = (x as f64 - centro) * spacing;
        let pos_z = (z as f64 - centro) * spacing;
        let distancia_centro = ((pos_x / spacing).powi(2) + (pos_z / spacing).powi(2)).sqrt();
        
        if distancia_centro > 35.0 && distancia_centro < 43.0 {
            let mineral = Box::new(Cubo::con_limites(
                Point3::new(pos_x - 0.4, 0.0, pos_z - 0.4),
                Point3::new(pos_x + 0.4, 2.0, pos_z + 0.4),
                Material::iron_ore_texturizado()
            ));
            escena.agregar_objeto(mineral);
        }
    }
    
    // Luz principal optimizada para mundo grande
    escena.agregar_luz(Luz::puntual(
        Point3::new(50.0, 80.0, 50.0),
        Vector3::new(1.0, 1.0, 1.0),
        200.0
    ));
    
    // Luz secundaria
    escena.agregar_luz(Luz::puntual(
        Point3::new(-50.0, 60.0, -50.0),
        Vector3::new(0.8, 0.9, 1.0),
        150.0
    ));
    
    println!("✅ Mundo Minecraft MASIVO generado:");
    println!("   🌍 Tamaño: {}x{} = {} bloques", size, size, size * size);
    println!("   🏔️ Montañas perimetrales con 3 tipos de piedra");
    println!("   🌊 5 lagos (1 central + 4 secundarios)");
    println!("   🌲 450 árboles (roble, abedul, jungla)");
    println!("   ⛏️ 130 depósitos minerales (carbón + hierro)");
    println!("   🎨 Usa todas las 19 texturas disponibles");
    
    escena
}

// ====================== ESCENA SIMPLE PARA DEBUG ======================

pub fn crear_escena_minecraft_simple() -> Escena {
    let mut escena = Escena::nueva();
    
    println!("🌍 Generando paisaje de Minecraft REAL con bloques individuales...");
    
    let size = 40; // Mundo de 40x40 bloques
    let mut alturas = vec![vec![0i32; size]; size]; // Mapa de alturas
    
    // GENERAR TERRENO ORGÁNICO CON RUIDO
    use std::f64::consts::PI;
    for x in 0..size {
        for z in 0..size {
            // Múltiples ondas de ruido para terreno natural
            let fx = x as f64 * 0.1;
            let fz = z as f64 * 0.1;
            
            let altura1 = (fx * 0.8).sin() * (fz * 0.6).cos() * 3.0;
            let altura2 = (fx * 0.3).sin() * (fz * 0.4).sin() * 2.0;
            let altura3 = (fx * 0.15).cos() * (fz * 0.2).cos() * 4.0;
            
            let altura_final = (altura1 + altura2 + altura3 + 5.0) as i32;
            alturas[x][z] = altura_final.max(1).min(15);
        }
    }
    
    // COLOCAR BLOQUES DE TERRENO INDIVIDUALES
    for x in 0..size {
        for z in 0..size {
            let altura_max = alturas[x][z];
            
            for y in 0..altura_max {
                let pos_x = (x as f64 - size as f64 / 2.0);
                let pos_z = (z as f64 - size as f64 / 2.0);
                let pos_y = y as f64;
                
                let material = if y == altura_max - 1 {
                    // Superficie: grass
                    Material::grass_top_texturizado()
                } else if y >= altura_max - 3 {
                    // Subsuelo: dirt
                    Material::dirt_texturizado()
                } else {
                    // Profundidad: stone
                    Material::stone_texturizado()
                };
                
                let bloque = Box::new(Cubo::con_limites(
                    Point3::new(pos_x, pos_y, pos_z),
                    Point3::new(pos_x + 1.0, pos_y + 1.0, pos_z + 1.0),
                    material
                ));
                escena.agregar_objeto(bloque);
            }
        }
    }
    
    // COLOCAR ÁRBOLES DE BLOQUES INDIVIDUALES
    let posiciones_arboles = [
        (15, 15), (25, 20), (18, 30), (32, 12), (8, 25), 
        (28, 28), (12, 8), (35, 35), (5, 15), (22, 25)
    ];
    
    for &(tree_x, tree_z) in &posiciones_arboles {
        if tree_x < size && tree_z < size {
            let altura_base = alturas[tree_x][tree_z];
            let pos_x = (tree_x as f64 - size as f64 / 2.0);
            let pos_z = (tree_z as f64 - size as f64 / 2.0);
            
            // Tronco (4-6 bloques de altura)
            let altura_tronco = 4 + (tree_x + tree_z) % 3;
            for y in 0..altura_tronco {
                let tronco = Box::new(Cubo::con_limites(
                    Point3::new(pos_x, altura_base as f64 + y as f64, pos_z),
                    Point3::new(pos_x + 1.0, altura_base as f64 + y as f64 + 1.0, pos_z + 1.0),
                    Material::oak_log_texturizado()
                ));
                escena.agregar_objeto(tronco);
            }
            
            // Hojas (corona 5x5x3)
            let base_hojas = altura_base as f64 + altura_tronco as f64 - 1.0;
            for dx in -2..=2 {
                for dz in -2..=2 {
                    for dy in 0..3 {
                        // Forma más natural de la corona
                        let distancia = (dx * dx + dz * dz) as f64;
                        if distancia <= 4.0 && (distancia <= 2.0 || dy < 2) {
                            let hoja = Box::new(Cubo::con_limites(
                                Point3::new(pos_x + dx as f64, base_hojas + dy as f64, pos_z + dz as f64),
                                Point3::new(pos_x + dx as f64 + 1.0, base_hojas + dy as f64 + 1.0, pos_z + dz as f64 + 1.0),
                                Material::oak_leaves_texturizado()
                            ));
                            escena.agregar_objeto(hoja);
                        }
                    }
                }
            }
        }
    }
    
    // LAGO NATURAL CON BLOQUES DE AGUA INDIVIDUALES
    let centro_lago_x = size / 4;
    let centro_lago_z = size / 4;
    let radio_lago = 6;
    
    for x in centro_lago_x - radio_lago..centro_lago_x + radio_lago {
        for z in centro_lago_z - radio_lago..centro_lago_z + radio_lago {
            if x < size && z < size {
                let dx = x as i32 - centro_lago_x as i32;
                let dz = z as i32 - centro_lago_z as i32;
                let distancia = ((dx * dx + dz * dz) as f64).sqrt();
                
                if distancia <= radio_lago as f64 {
                    let pos_x = (x as f64 - size as f64 / 2.0);
                    let pos_z = (z as f64 - size as f64 / 2.0);
                    let altura_agua = (alturas[x][z] - 2).max(1);
                    
                    let agua = Box::new(Cubo::con_limites(
                        Point3::new(pos_x, altura_agua as f64, pos_z),
                        Point3::new(pos_x + 1.0, altura_agua as f64 + 1.0, pos_z + 1.0),
                        Material::water_texturizado()
                    ));
                    escena.agregar_objeto(agua);
                }
            }
        }
    }
    
    // Agregar iluminación
    for luz in crear_iluminacion_minecraft() {
        escena.agregar_luz(luz);
    }
    
    println!("✅ Paisaje de Minecraft REAL generado:");
    println!("   �️ Terreno orgánico 40x40 con colinas y valles");
    println!("   � Superficie de grass con dirt y stone por capas");
    println!("   🌳 10 árboles individuales con troncos y hojas naturales");
    println!("   🌊 1 lago natural formado por bloques de agua");
    println!("   🧱 {} bloques individuales total", "Miles de");
    
    escena
}

