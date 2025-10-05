use nalgebra::Vector3;
use image::{RgbImage, open};
use std::collections::HashMap;

pub struct Textura {
    pub imagen: RgbImage,
    pub ancho: u32,
    pub alto: u32,
}

impl Textura {
    pub fn cargar(ruta: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let img = open(ruta)?.to_rgb8();
        let (ancho, alto) = img.dimensions();
        
        Ok(Textura {
            imagen: img,
            ancho,
            alto,
        })
    }
    
    pub fn sample(&self, u: f64, v: f64) -> Vector3<f64> {
        // Coordenadas UV normalizadas [0,1] a píxeles
        let x = ((u.fract() * self.ancho as f64) as u32).min(self.ancho - 1);
        let y = ((v.fract() * self.alto as f64) as u32).min(self.alto - 1);
        
        let pixel = self.imagen.get_pixel(x, y);
        
        // Convertir RGB [0,255] a Vector3 [0,1]
        Vector3::new(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
        )
    }
}

pub struct GestorTexturas {
    texturas: HashMap<String, Textura>,
}

impl GestorTexturas {
    pub fn nuevo() -> Self {
        GestorTexturas {
            texturas: HashMap::new(),
        }
    }
    
    pub fn cargar_textura(&mut self, nombre: &str, ruta: &str) -> Result<(), Box<dyn std::error::Error>> {
        let textura = Textura::cargar(ruta)?;
        self.texturas.insert(nombre.to_string(), textura);
        Ok(())
    }
    
    pub fn obtener_textura(&self, nombre: &str) -> Option<&Textura> {
        self.texturas.get(nombre)
    }
    
    pub fn cargar_texturas_minecraft() -> Result<Self, Box<dyn std::error::Error>> {
        let mut gestor = GestorTexturas::nuevo();
        
        // Cargar las texturas disponibles
        gestor.cargar_textura("agua", "texturas/agua.jpg")?;
        gestor.cargar_textura("pasto", "texturas/pasto.jpg")?;
        gestor.cargar_textura("dirt", "texturas/dirt.jpg")?;
        
        println!("✅ Texturas de Minecraft cargadas exitosamente:");
        println!("   - Agua: texturas/agua.jpg");
        println!("   - Pasto: texturas/pasto.jpg");
        println!("   - Tierra: texturas/dirt.jpg");
        
        Ok(gestor)
    }
}