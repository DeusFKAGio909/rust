// Este archivo convierte tu proyecto en una librería
// Puedes tener AMBOS: lib.rs (librería) y main.rs (binario)
// Otros proyectos pueden usar tu código como librería

pub mod modulos; // Re-exporta el módulo para uso externo

pub fn funcion_publica() -> String {
    "Esta función puede ser usada por otros proyectos".to_string()
}

#[allow(dead_code)]
fn funcion_privada() {
    // Esta solo es visible dentro de este módulo
    // Ejemplo de función privada que no se usa externamente
}
