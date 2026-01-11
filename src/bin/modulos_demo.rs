// ============================================
// DEMOSTRACIÓN DE MÓDULOS Y SUBCARPETAS
// ============================================

// Importar módulos desde la librería
// Como este es un binario, podemos usar el crate name
use platzi_course::modulos;

fn main() {
    println!("=== DEMOSTRACIÓN DE MÓDULOS ===\n");

    // ========== USO DE MÓDULOS ==========
    println!("1. Funciones desde módulos:");
    
    // Usando funciones re-exportadas
    let suma = modulos::sumar(5, 3);
    println!("   Suma: {}", suma);

    let producto = modulos::multiplicar(4, 7);
    println!("   Producto: {}", producto);

    let nombre = modulos::formatear_nombre("Juan", "Pérez");
    println!("   Nombre formateado: {}", nombre);

    // ========== ESTRUCTURAS ==========
    println!("\n2. Estructuras desde módulos:");
    
    use modulos::estructuras::{Persona, Estado, procesar_estado};
    
    let persona = Persona::new("María".to_string(), 25);
    println!("   {}", persona.presentarse());

    // ========== ENUMS Y PATTERN MATCHING ==========
    println!("\n3. Enums y pattern matching:");
    
    let estados = vec![
        Estado::Activo,
        Estado::Inactivo,
        Estado::Pendiente { dias: 5 },
        Estado::Error("Conexión perdida".to_string()),
    ];

    for estado in estados {
        println!("   Estado: {:?} -> {}", estado, procesar_estado(&estado));
    }

    // ========== MÓDULO DE MATEMÁTICAS ==========
    println!("\n4. Módulo de matemáticas:");
    
    use modulos::matematicas::Calculadora;
    
    let mut calc = Calculadora::new(10);
    calc.agregar(5);
    calc.agregar(3);
    println!("   Calculadora: {}", calc.obtener());

    // ========== STRINGS ==========
    println!("\n5. Manejo de strings:");
    
    use modulos::strings::{procesar_string, procesar_sin_ownership};
    
    let texto1 = String::from("Hola");
    let resultado = procesar_string(texto1);
    // texto1 ya no existe aquí (fue movido)
    println!("   Con ownership: {}", resultado);

    let mut texto2 = String::from("Mundo");
    procesar_sin_ownership(&mut texto2);
    // texto2 sigue existiendo
    println!("   Sin ownership: {}", texto2);
}
