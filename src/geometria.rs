use nalgebra::{Vector3, Point3};
use crate::materiales::Material;

#[derive(Clone)]
pub struct Rayo {
    pub origen: Point3<f64>,
    pub direccion: Vector3<f64>,
}

impl Rayo {
    pub fn new(origen: Point3<f64>, direccion: Vector3<f64>) -> Self {
        Self { origen, direccion: direccion.normalize() }
    }
    
    pub fn punto_en(&self, t: f64) -> Point3<f64> {
        self.origen + self.direccion * t
    }
}

pub struct Interseccion {
    pub punto: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub material: Material,
}

pub trait Figura {
    fn intersectar(&self, rayo: &Rayo) -> Option<Interseccion>;
}

// Cubo texturizado - nuestro elemento básico del diorama
pub struct Cubo {
    pub centro: Point3<f64>,
    pub tamano: f64,
    pub material: Material,
}

impl Cubo {
    pub fn nuevo(centro: Point3<f64>, tamano: f64, material: Material) -> Self {
        Self { centro, tamano, material }
    }
}

impl Figura for Cubo {
    fn intersectar(&self, rayo: &Rayo) -> Option<Interseccion> {
        let min = self.centro - Vector3::new(self.tamano/2.0, self.tamano/2.0, self.tamano/2.0);
        let max = self.centro + Vector3::new(self.tamano/2.0, self.tamano/2.0, self.tamano/2.0);
        
        let mut tmin = (min.x - rayo.origen.x) / rayo.direccion.x;
        let mut tmax = (max.x - rayo.origen.x) / rayo.direccion.x;
        
        if tmin > tmax { std::mem::swap(&mut tmin, &mut tmax); }
        
        let mut tymin = (min.y - rayo.origen.y) / rayo.direccion.y;
        let mut tymax = (max.y - rayo.origen.y) / rayo.direccion.y;
        
        if tymin > tymax { std::mem::swap(&mut tymin, &mut tymax); }
        
        if tmin > tymax || tymin > tmax { return None; }
        
        tmin = tmin.max(tymin);
        tmax = tmax.min(tymax);
        
        let mut tzmin = (min.z - rayo.origen.z) / rayo.direccion.z;
        let mut tzmax = (max.z - rayo.origen.z) / rayo.direccion.z;
        
        if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax); }
        
        if tmin > tzmax || tzmin > tmax { return None; }
        
        tmin = tmin.max(tzmin);
        tmax = tmax.min(tzmax);
        
        let t = if tmin > 0.001 { tmin } else if tmax > 0.001 { tmax } else { return None; };
        
        let punto = rayo.punto_en(t);
        let mut normal = Vector3::new(0.0, 0.0, 0.0);
        
        // Determinar la cara que fue golpeada
        let distancias = [
            (punto.x - min.x).abs(), // izquierda
            (max.x - punto.x).abs(), // derecha
            (punto.y - min.y).abs(), // abajo
            (max.y - punto.y).abs(), // arriba
            (punto.z - min.z).abs(), // atrás
            (max.z - punto.z).abs(), // adelante
        ];
        
        let cara_minima = distancias.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap().0;
        
        match cara_minima {
            0 => normal = Vector3::new(-1.0, 0.0, 0.0), // izquierda
            1 => normal = Vector3::new(1.0, 0.0, 0.0),  // derecha
            2 => normal = Vector3::new(0.0, -1.0, 0.0), // abajo
            3 => normal = Vector3::new(0.0, 1.0, 0.0),  // arriba
            4 => normal = Vector3::new(0.0, 0.0, -1.0), // atrás
            5 => normal = Vector3::new(0.0, 0.0, 1.0),  // adelante
            _ => unreachable!(),
        }
        
        Some(Interseccion {
            punto,
            normal,
            t,
            material: self.material.clone(),
        })
    }
}

// Esfera para elementos decorativos
pub struct Esfera {
    pub centro: Point3<f64>,
    pub radio: f64,
    pub material: Material,
}

impl Esfera {
    pub fn nueva(centro: Point3<f64>, radio: f64, material: Material) -> Self {
        Self { centro, radio, material }
    }
}

impl Figura for Esfera {
    fn intersectar(&self, rayo: &Rayo) -> Option<Interseccion> {
        let oc = rayo.origen - self.centro;
        let a = rayo.direccion.dot(&rayo.direccion);
        let b = 2.0 * oc.dot(&rayo.direccion);
        let c = oc.dot(&oc) - self.radio * self.radio;
        
        let discriminante = b * b - 4.0 * a * c;
        
        if discriminante < 0.0 {
            return None;
        }
        
        let sqrt_discriminante = discriminante.sqrt();
        let t1 = (-b - sqrt_discriminante) / (2.0 * a);
        let t2 = (-b + sqrt_discriminante) / (2.0 * a);
        
        let t = if t1 > 0.001 { t1 } else if t2 > 0.001 { t2 } else { return None; };
        
        let punto = rayo.punto_en(t);
        let normal = (punto - self.centro).normalize();
        
        Some(Interseccion {
            punto,
            normal,
            t,
            material: self.material.clone(),
        })
    }
}

// Plano para el suelo
pub struct Plano {
    pub punto: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: Material,
}

impl Plano {
    pub fn nuevo(punto: Point3<f64>, normal: Vector3<f64>, material: Material) -> Self {
        Self { punto, normal: normal.normalize(), material }
    }
}

impl Figura for Plano {
    fn intersectar(&self, rayo: &Rayo) -> Option<Interseccion> {
        let denom = self.normal.dot(&rayo.direccion);
        
        if denom.abs() < 1e-6 {
            return None; // Rayo paralelo al plano
        }
        
        let t = (self.punto - rayo.origen).dot(&self.normal) / denom;
        
        if t > 0.001 {
            let punto = rayo.punto_en(t);
            Some(Interseccion {
                punto,
                normal: self.normal,
                t,
                material: self.material.clone(),
            })
        } else {
            None
        }
    }
}