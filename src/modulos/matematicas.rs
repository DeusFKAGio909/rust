// Módulo de matemáticas - demuestra funciones privadas vs públicas

// Función pública - puede ser usada desde fuera del módulo
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiplicar(a: i32, b: i32) -> i32 {
    a * b
}

// Función privada - solo visible dentro de este módulo
fn dividir(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        f64::NAN
    }
}

// Función pública que usa una privada
pub fn dividir_seguro(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("División por cero".to_string())
    } else {
        Ok(dividir(a, b))
    }
}

// Estructura con métodos
pub struct Calculadora {
    valor: i32,
}

impl Calculadora {
    pub fn new(valor: i32) -> Self {
        Calculadora { valor }
    }

    pub fn agregar(&mut self, n: i32) {
        self.valor += n;
    }

    pub fn obtener(&self) -> i32 {
        self.valor
    }
}
