use nalgebra::{Vector3, Point3};
use crate::geometria::*;
use crate::materiales::*;
use crate::iluminacion::*;

pub struct Escena {
    pub objetos: Vec<Box<dyn Figura>>,
    pub luces: Vec<Luz>,
}

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

// Función para crear nuestro diorama único
pub fn crear_diorama() -> Escena {
    let mut escena = Escena::nueva();
    
    // Suelo del diorama - tierra fértil
    escena.agregar_objeto(Box::new(Plano::nuevo(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Material::tierra_fertil()
    )));
    
    // === ESTRUCTURA PRINCIPAL - CASTILLO DE CRISTAL ===
    
    // Torres principales del castillo
    for i in 0..4 {
        let x = if i % 2 == 0 { -4.0 } else { 4.0 };
        let z = if i < 2 { -4.0 } else { 4.0 };
        
        // Base de la torre (piedra lunar)
        for altura in 0..3 {
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(x, altura as f64 + 0.5, z),
                1.0,
                Material::piedra_lunar()
            )));
        }
        
        // Techo de cristal aguamarina
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(x, 3.5, z),
            1.0,
            Material::cristal_aguamarina()
        )));
    }
    
    // Murallas conectoras (madera coral)
    for i in -3..4 {
        // Muralla frontal
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(i as f64, 1.0, -4.0),
            1.0,
            Material::madera_coral()
        )));
        
        // Muralla trasera
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(i as f64, 1.0, 4.0),
            1.0,
            Material::madera_coral()
        )));
        
        // Murallas laterales
        if i != 0 { // Dejar espacio para la entrada
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(-4.0, 1.0, i as f64),
                1.0,
                Material::madera_coral()
            )));
            
            escena.agregar_objeto(Box::new(Cubo::nuevo(
                Point3::new(4.0, 1.0, i as f64),
                1.0,
                Material::madera_coral()
            )));
        }
    }
    
    // === PATIO INTERIOR ===
    
    // Fuente central con agua cristalina
    escena.agregar_objeto(Box::new(Cubo::nuevo(
        Point3::new(0.0, 0.5, 0.0),
        1.5,
        Material::agua_cristalina()
    )));
    
    // Pilares decorativos de metal dorado
    for &pos in &[(-2.0, -2.0), (2.0, -2.0), (-2.0, 2.0), (2.0, 2.0)] {
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(pos.0, 1.5, pos.1),
            0.5,
            Material::metal_dorado()
        )));
        
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(pos.0, 2.5, pos.1),
            0.5,
            Material::metal_dorado()
        )));
    }
    
    // === JARDINES EXTERIORES ===
    
    // Árboles de césped esmeralda
    for &pos in &[(-6.0, -6.0), (6.0, -6.0), (-6.0, 6.0), (6.0, 6.0)] {
        // Tronco (madera coral)
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(pos.0, 0.5, pos.1),
            0.5,
            Material::madera_coral()
        )));
        
        // Copa del árbol (césped esmeralda)
        for y in 1..4 {
            for x in -1..2 {
                for z in -1..2 {
                    if !(x == 0 && z == 0) || y > 1 {
                        escena.agregar_objeto(Box::new(Cubo::nuevo(
                            Point3::new(pos.0 + x as f64 * 0.5, y as f64 + 0.5, pos.1 + z as f64 * 0.5),
                            0.5,
                            Material::cesped_esmeralda()
                        )));
                    }
                }
            }
        }
    }
    
    // === ELEMENTOS ESPECIALES ===
    
    // Pozos de lava decorativos (efecto emisivo)
    for &pos in &[(-7.0, 0.0), (7.0, 0.0)] {
        escena.agregar_objeto(Box::new(Cubo::nuevo(
            Point3::new(pos.0, 0.5, pos.1),
            1.0,
            Material::lava_magmatica()
        )));
    }
    
    // Esferas flotantes de cristal (elemento mágico)
    for i in 0..6 {
        let angulo = i as f64 * std::f64::consts::PI / 3.0;
        let radio = 8.0;
        let x = angulo.cos() * radio;
        let z = angulo.sin() * radio;
        
        escena.agregar_objeto(Box::new(Esfera::nueva(
            Point3::new(x, 4.0 + (i as f64 * 0.5).sin(), z),
            0.3,
            Material::cristal_aguamarina()
        )));
    }
    
    // Configurar iluminación
    for luz in crear_iluminacion_diorama() {
        escena.agregar_luz(luz);
    }
    
    escena
}