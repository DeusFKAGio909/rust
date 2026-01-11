// ============================================
// COMPARACIONES: Rust vs Python vs C++
// ============================================

fn main() {
    println!("=== RUST vs PYTHON vs C++ ===\n");

    // ========== 1. MEMORIA Y OWNERSHIP ==========
    println!("1. MEMORIA Y OWNERSHIP:");
    println!("   Python: Garbage Collector maneja memoria automáticamente");
    println!("   C++:    Tú manejas new/delete o smart pointers");
    println!("   Rust:   Ownership system - memoria segura sin GC\n");

    ejemplo_ownership();

    // ========== 2. STRINGS ==========
    println!("\n2. STRINGS:");
    ejemplo_strings();

    // ========== 3. VECTORS/ARRAYS ==========
    println!("\n3. VECTORS/ARRAYS:");
    ejemplo_vectores();

    // ========== 4. ERROR HANDLING ==========
    println!("\n4. MANEJO DE ERRORES:");
    ejemplo_errores();

    // ========== 5. PATTERN MATCHING ==========
    println!("\n5. PATTERN MATCHING:");
    ejemplo_pattern_matching();

    // ========== 6. GENERICS ==========
    println!("\n6. GENERICS (Templates en C++):");
    ejemplo_generics();
}

fn ejemplo_ownership() {
    let s1 = String::from("hola");
    let s2 = s1; // s1 se mueve a s2 (move semantics)
    // println!("{}", s1); // ERROR! s1 ya no existe
    println!("   s2 = '{}' (s1 fue movido)", s2);

    // En Python: s1 y s2 apuntarían al mismo objeto
    // En C++: necesitarías std::move() explícito
    // En Rust: automático y seguro
}

fn ejemplo_strings() {
    // Python: s = "hola"  (inmutable, internado)
    // C++:    std::string s = "hola";
    // Rust:   &str (slice) vs String (owned)

    let slice: &str = "hola"; // Referencia a string literal
    let owned: String = String::from("mundo"); // String owned

    println!("   &str: '{}' (referencia, no owned)", slice);
    println!("   String: '{}' (owned, puede modificar)", owned);

    // Concatenación
    let combinado = format!("{} {}", slice, owned);
    println!("   Combinado: '{}'", combinado);
}

fn ejemplo_vectores() {
    // Python: lista = [1, 2, 3]
    // C++:    std::vector<int> vec = {1, 2, 3};
    // Rust:   let vec = vec![1, 2, 3];

    let mut numeros = vec![1, 2, 3];
    numeros.push(4);

    println!("   Vector: {:?}", numeros);

    // Iteración (similar en los 3, pero Rust es más seguro)
    for num in &numeros {
        println!("   Número: {}", num);
    }

    // Map (Python: list comprehension, C++: std::transform)
    let dobles: Vec<i32> = numeros.iter().map(|x| x * 2).collect();
    println!("   Dobles: {:?}", dobles);
}

fn ejemplo_errores() {
    // Python: try/except
    // C++:    try/catch o códigos de error
    // Rust:   Result<T, E> o Option<T>

    fn dividir(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("División por cero".to_string())
        } else {
            Ok(a / b)
        }
    }

    match dividir(10.0, 2.0) {
        Ok(resultado) => println!("   Resultado: {}", resultado),
        Err(e) => println!("   Error: {}", e),
    }

    // Métodos útiles de Result
    let resultado = dividir(10.0, 0.0).unwrap_or(0.0); // Valor por defecto
    println!("   Con unwrap_or: {}", resultado);
}

fn ejemplo_pattern_matching() {
    // Python: match/case (3.10+), antes if/elif
    // C++:    switch (solo números/enums básicos)
    // Rust:   match (exhaustivo y poderoso)

    let numero = 42;

    let mensaje = match numero {
        0 => "cero",
        1..=10 => "pequeño",
        11..=100 => "mediano",
        _ => "grande",
    };

    println!("   {} es {}", numero, mensaje);

    // Con Option (similar a None en Python, nullptr en C++)
    let opcion: Option<i32> = Some(5);
    match opcion {
        Some(valor) => println!("   Valor encontrado: {}", valor),
        None => println!("   No hay valor"),
    }
}

fn ejemplo_generics() {
    // Python: Duck typing (sin tipos explícitos)
    // C++:    Templates
    // Rust:   Generics (similar a C++, pero más seguro)

    fn obtener_mayor<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }

    println!("   Mayor entre 5 y 3: {}", obtener_mayor(5, 3));
    println!("   Mayor entre 'z' y 'a': {}", obtener_mayor('z', 'a'));

    // Struct genérico
    struct Contenedor<T> {
        valor: T,
    }

    let contenedor_int = Contenedor { valor: 42 };
    let contenedor_str = Contenedor {
        valor: "hola".to_string(),
    };

    println!("   Contenedor int: {}", contenedor_int.valor);
    println!("   Contenedor str: {}", contenedor_str.valor);
}
