// Módulo de manipulación de strings
// Rust tiene ownership, diferente a Python

pub fn formatear_nombre(nombre: &str, apellido: &str) -> String {
    // &str es una referencia (no toma ownership)
    // String es owned (tiene ownership)
    format!("{} {}", nombre, apellido)
}

// Diferencia clave con Python:
// En Python: s = "hola"  -> s es una referencia
// En Rust:   s: &str     -> referencia (no owned)
//            s: String   -> owned (puede modificar)

pub fn procesar_string(mut texto: String) -> String {
    // Toma ownership del String
    texto.push_str(" (procesado)");
    texto // Ownership se transfiere al retornar
}

pub fn procesar_sin_ownership(texto: &mut String) {
    // Toma una referencia mutable
    texto.push_str(" (modificado)");
    // No toma ownership, el original sigue existiendo
}
