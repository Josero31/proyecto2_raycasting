use nalgebra::{Vector3, Point3};
use crate::geometria::Rayo;
use std::f64::consts::PI;

pub struct Camara {
    pub posicion: Point3<f64>,
    pub objetivo: Point3<f64>,
    pub arriba: Vector3<f64>,
    pub campo_vision: f64,
    pub aspecto: f64,
    
    // Vectores de la base de la cámara
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
    pub w: Vector3<f64>,
    
    // Dimensiones del plano de imagen
    pub mitad_ancho: f64,
    pub mitad_alto: f64,
    pub esquina_inferior_izquierda: Point3<f64>,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
}

impl Camara {
    pub fn nueva(posicion: Point3<f64>, objetivo: Point3<f64>, arriba: Vector3<f64>, 
                 campo_vision: f64, aspecto: f64) -> Self {
        let theta = campo_vision * PI / 180.0;
        let mitad_alto = (theta / 2.0).tan();
        let mitad_ancho = aspecto * mitad_alto;
        
        let w = (posicion - objetivo).normalize();
        let u = arriba.cross(&w).normalize();
        let v = w.cross(&u);
        
        let horizontal = u * (2.0 * mitad_ancho);
        let vertical = v * (2.0 * mitad_alto);
        let esquina_inferior_izquierda = posicion - horizontal/2.0 - vertical/2.0 - w;
        
        Self {
            posicion,
            objetivo,
            arriba,
            campo_vision,
            aspecto,
            u,
            v,
            w,
            mitad_ancho,
            mitad_alto,
            esquina_inferior_izquierda,
            horizontal,
            vertical,
        }
    }
    
    pub fn obtener_rayo(&self, s: f64, t: f64) -> Rayo {
        let direccion = self.esquina_inferior_izquierda + s * self.horizontal + t * self.vertical - self.posicion;
        Rayo::new(self.posicion, direccion)
    }
    
    // Funciones para rotar la cámara alrededor del objetivo
    pub fn rotar_horizontal(&mut self, angulo: f64) {
        let distancia = (self.posicion - self.objetivo).magnitude();
        
        // Calcular nueva posición rotando alrededor del eje Y
        let cos_angulo = angulo.cos();
        let sin_angulo = angulo.sin();
        
        let direccion_actual = (self.posicion - self.objetivo).normalize();
        let nueva_x = direccion_actual.x * cos_angulo - direccion_actual.z * sin_angulo;
        let nueva_z = direccion_actual.x * sin_angulo + direccion_actual.z * cos_angulo;
        
        self.posicion = self.objetivo + Vector3::new(nueva_x, direccion_actual.y, nueva_z) * distancia;
        self.actualizar_vectores();
    }
    
    pub fn rotar_vertical(&mut self, angulo: f64) {
        let distancia = (self.posicion - self.objetivo).magnitude();
        let direccion_actual = (self.posicion - self.objetivo).normalize();
        
        // Limitar rotación vertical para evitar voltear la cámara
        let nueva_y = (direccion_actual.y + angulo).clamp(-0.9, 0.9);
        let factor_horizontal = (1.0 - nueva_y * nueva_y).sqrt();
        
        let direccion_horizontal = Vector3::new(direccion_actual.x, 0.0, direccion_actual.z).normalize();
        let nueva_direccion = Vector3::new(
            direccion_horizontal.x * factor_horizontal,
            nueva_y,
            direccion_horizontal.z * factor_horizontal
        );
        
        self.posicion = self.objetivo + nueva_direccion * distancia;
        self.actualizar_vectores();
    }
    
    pub fn acercar(&mut self, factor: f64) {
        let direccion = (self.posicion - self.objetivo).normalize();
        let distancia_actual = (self.posicion - self.objetivo).magnitude();
        let nueva_distancia = (distancia_actual * factor).max(2.0); // Mínimo 2 unidades de distancia
        
        self.posicion = self.objetivo + direccion * nueva_distancia;
        self.actualizar_vectores();
    }
    
    fn actualizar_vectores(&mut self) {
        self.w = (self.posicion - self.objetivo).normalize();
        self.u = self.arriba.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);
        
        self.horizontal = self.u * (2.0 * self.mitad_ancho);
        self.vertical = self.v * (2.0 * self.mitad_alto);
        self.esquina_inferior_izquierda = self.posicion - self.horizontal/2.0 - self.vertical/2.0 - self.w;
    }
}