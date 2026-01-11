// ============================================
// FEATURES AVANZADAS DE RUST
// ============================================

fn main() {
    println!("=== FEATURES AVANZADAS DE RUST ===\n");

    // ========== 1. LIFETIMES ==========
    println!("1. LIFETIMES (no existen en Python/C++):");
    ejemplo_lifetimes();

    // ========== 2. MACROS ==========
    println!("\n2. MACROS:");
    ejemplo_macros();

    // ========== 3. TRAITS ==========
    println!("\n3. TRAITS (similar a interfaces en C++, protocols en Python):");
    ejemplo_traits();

    // ========== 4. CLOSURES ==========
    println!("\n4. CLOSURES (lambdas en C++, funciones anónimas en Python):");
    ejemplo_closures();

    // ========== 5. ITERATORS ==========
    println!("\n5. ITERATORS (lazy evaluation):");
    ejemplo_iterators();

    // ========== 6. SMART POINTERS ==========
    println!("\n6. SMART POINTERS:");
    ejemplo_smart_pointers();

    // ========== 7. UNSAFE RUST ==========
    println!("\n7. UNSAFE RUST (para casos especiales):");
    ejemplo_unsafe();
}

// ========== LIFETIMES ==========
// Garantizan que las referencias sean válidas
// No existe en Python (GC lo maneja) ni en C++ (tú te encargas)

fn ejemplo_lifetimes() {
    // 'a es un lifetime parameter
    fn obtener_mayor<'a>(a: &'a str, b: &'a str) -> &'a str {
        if a.len() > b.len() {
            a
        } else {
            b
        }
    }

    let s1 = String::from("hola");
    let s2 = String::from("mundo");
    let mayor = obtener_mayor(&s1, &s2);
    println!("   Mayor string: '{}'", mayor);

    // El compilador garantiza que 'mayor' no sobreviva a s1 y s2
}

// ========== MACROS ==========
// Más poderosas que en C++, similares a decoradores en Python pero más potentes

macro_rules! mi_macro {
    // Macro simple
    ($x:expr) => {
        println!("   Macro recibió: {}", $x);
    };
    // Macro con múltiples patrones
    ($x:expr, $y:expr) => {
        println!("   Macro recibió: {} y {}", $x, $y);
    };
}

fn ejemplo_macros() {
    mi_macro!(42);
    mi_macro!("hola", "mundo");

    // Macros estándar útiles:
    println!("   vec! macro: {:?}", vec![1, 2, 3]);
    println!("   format! macro: {}", format!("Hola {}", "Rust"));
}

// ========== TRAITS ==========
// Similar a interfaces en C++ o protocols en Python

trait Hablador {
    fn hablar(&self) -> String;
    fn gritar(&self) -> String {
        // Método por defecto (similar a métodos virtuales en C++)
        self.hablar().to_uppercase()
    }
}

struct Persona {
    nombre: String,
}

struct Gato;

impl Hablador for Persona {
    fn hablar(&self) -> String {
        format!("Hola, soy {}", self.nombre)
    }
}

impl Hablador for Gato {
    fn hablar(&self) -> String {
        "Miau".to_string()
    }
}

fn ejemplo_traits() {
    let persona = Persona {
        nombre: "Juan".to_string(),
    };
    let gato = Gato;

    println!("   Persona: {}", persona.hablar());
    println!("   Gato: {}", gato.hablar());
    println!("   Persona gritando: {}", persona.gritar());

    // Función genérica con trait bound
    fn hacer_hablar<T: Hablador>(hablador: T) {
        println!("   Función genérica: {}", hablador.hablar());
    }

    hacer_hablar(persona);
    hacer_hablar(gato);
}

// ========== CLOSURES ==========
// Más poderosas que lambdas en C++, similares a Python pero con tipos

fn ejemplo_closures() {
    // Closure simple
    let sumar = |a: i32, b: i32| a + b;
    println!("   Suma: {}", sumar(5, 3));

    // Closure que captura variables del entorno
    let multiplicador = 2;
    let multiplicar = |x: i32| x * multiplicador;
    println!("   Multiplicar 5 por {}: {}", multiplicador, multiplicar(5));

    // Closure con move (toma ownership)
    let s = String::from("hola");
    let closure_move = move || {
        println!("   String movido: {}", s);
    };
    // println!("{}", s); // ERROR: s fue movido
    closure_move();
}

// ========== ITERATORS ==========
// Lazy evaluation, similar a generators en Python

fn ejemplo_iterators() {
    let numeros = vec![1, 2, 3, 4, 5];

    // Iterator chain (lazy - no se ejecuta hasta collect)
    let resultado: Vec<i32> = numeros
        .iter()
        .map(|x| x * 2)      // Doblar
        .filter(|&x| x > 5)  // Filtrar mayores a 5
        .collect();          // Forzar evaluación

    println!("   Iterator chain: {:?}", resultado);

    // Iterator con take (similar a itertools.islice en Python)
    let primeros: Vec<i32> = numeros.iter().take(3).copied().collect();
    println!("   Primeros 3: {:?}", primeros);
}

// ========== SMART POINTERS ==========
// Similar a std::shared_ptr en C++, pero más seguro

use std::rc::Rc; // Reference Counted (single-threaded)
use std::sync::Arc; // Atomic Reference Counted (multi-threaded)

fn ejemplo_smart_pointers() {
    // Rc - múltiples owners del mismo dato (single-threaded)
    let dato = Rc::new(42);
    let _referencia1 = Rc::clone(&dato);
    let _referencia2 = Rc::clone(&dato);

    println!("   Rc count: {}", Rc::strong_count(&dato));
    println!("   Valor: {}", *dato);

    // Arc - thread-safe version
    let dato_arc = Arc::new(100);
    println!("   Arc valor: {}", *dato_arc);
}

// ========== UNSAFE RUST ==========
// Para casos donde necesitas bypassear las garantías de seguridad
// Similar a usar punteros raw en C++

fn ejemplo_unsafe() {
    let mut numero = 5;

    // Unsafe block - puedes hacer cosas peligrosas aquí
    unsafe {
        let puntero = &mut numero as *mut i32;
        *puntero = 10;
    }

    println!("   Número después de unsafe: {}", numero);

    // ⚠️ Solo usa unsafe cuando realmente lo necesites
    // La mayoría del código Rust es safe
}
