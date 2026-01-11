// mod.rs es el punto de entrada de un módulo en una subcarpeta
// Esto es equivalente a __init__.py en Python

pub mod matematicas;
pub mod strings;
pub mod estructuras;

// Re-exportar funciones comunes (patrón común en Rust)
pub use matematicas::{sumar, multiplicar};
pub use strings::formatear_nombre;
