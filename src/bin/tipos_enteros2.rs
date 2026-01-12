// ============================================
// GUÍA DE TIPOS DE ENTEROS EN RUST
// Versión mejorada con explicaciones sobre por qué necesitamos tipos
// ============================================

fn main() {
    titulo("Guía rápida: tipos de enteros en Rust");

    seccion("1) 8 bits (i8 / u8)", ejemplo_i8_u8);
    seccion("2) 16 bits (i16 / u16)", ejemplo_i16_u16);
    seccion("3) 32 bits (i32 / u32) — default", ejemplo_i32_u32);
    seccion("4) 64 bits (i64 / u64)", ejemplo_i64_u64);
    seccion("5) 128 bits (i128 / u128)", ejemplo_i128_u128);
    seccion("6) Tamaño según arquitectura (isize / usize)", ejemplo_isize_usize);

    seccion("7) Casos prácticos: ¿cuál usar?", casos_practicos);
    seccion("8) Errores comunes y cómo evitarlos", errores_comunes);
    
    // Nueva sección: Explicación de por qué necesitamos tipos
    seccion("9) ¿Por qué Rust necesita tipos definidos?", explicacion_tipos);
}

fn titulo(texto: &str) {
    println!("\n{}\n{}", texto, "-".repeat(texto.len()));
}

fn seccion(titulo: &str, f: fn()) {
    println!("\n{titulo}");
    println!("{}", "─".repeat(titulo.len()));
    f();
}

fn rango(tipo: &str, min: &str, max: &str) {
    println!("Rango {tipo}: {min} .. {max}");
}

// ===================== Explicación de Tipos =====================

fn explicacion_tipos() {
    println!("\n🔍 ¿POR QUÉ RUST NECESITA SABER EL TIPO DE CADA VARIABLE?");
    println!("{}", "─".repeat(60));
    
    println!("\n1. SEGURIDAD DE MEMORIA:");
    println!("   Rust garantiza seguridad sin Garbage Collector.");
    println!("   Para esto, necesita saber:");
    println!("   - Cuánta memoria reservar");
    println!("   - Cuándo liberar la memoria");
    println!("   - Si el valor puede ser negativo o no");
    
    println!("\n   Ejemplo:");
    println!("   let edad: u8 = 9;");
    println!("   ↑ Rust sabe: 'Reservar 1 byte, valor siempre positivo'");
    println!("   let temp: i8 = -10;");
    println!("   ↑ Rust sabe: 'Reservar 1 byte, valor puede ser negativo'");
    
    println!("\n2. PREVENCIÓN DE ERRORES EN TIEMPO DE COMPILACIÓN:");
    println!("   Rust detecta errores ANTES de ejecutar el programa.");
    
    println!("\n   Ejemplo de error que Rust previene:");
    println!("   let edad: u8 = 300;  // ❌ ERROR: u8 solo llega a 255");
    println!("   Rust te avisa: 'No puedes guardar 300 en u8'");
    println!("   En Python/C++ esto causaría overflow silencioso");
    
    println!("\n3. OPTIMIZACIÓN DE MEMORIA:");
    println!("   Rust usa solo la memoria necesaria.");
    
    println!("\n   Comparación:");
    println!("   Python:  edad = 9      → Ocupa ~28 bytes (objeto completo)");
    println!("   Rust:    let edad: u8 = 9; → Ocupa 1 byte ✅");
    println!("   Rust:    let edad: i32 = 9; → Ocupa 4 bytes (innecesario)");
    
    println!("\n4. INFERENCIA DE TIPOS (Rust es inteligente):");
    println!("   No siempre necesitas escribir el tipo explícitamente.");
    
    println!("\n   Ejemplo:");
    println!("   let numero = 42;  // Rust infiere: i32 (tipo por defecto)");
    println!("   let edad: u8 = 9;  // Especificas: u8 (optimización)");
    
    println!("\n5. OWNERSHIP Y BORROWING:");
    println!("   El tipo determina cómo se maneja la memoria:");
    
    println!("\n   Ejemplo:");
    println!("   let s1: String = String::from(\"hola\");  // Owned (tiene dueño)");
    println!("   let s2: &str = \"mundo\";  // Referencia (no tiene dueño)");
    println!("   Rust sabe quién es responsable de liberar la memoria");
    
    println!("\n6. OPERACIONES MATEMÁTICAS SEGURAS:");
    println!("   Rust previene overflow en operaciones.");
    
    println!("\n   Ejemplo:");
    println!("   let a: u8 = 200;");
    println!("   let b: u8 = 100;");
    println!("   let suma = a + b;  // Rust verifica: ¿300 cabe en u8?");
    println!("   // En modo debug: panic si overflow");
    println!("   // En modo release: puede hacer wrap (según configuración)");
    
    println!("\n7. COMPARACIÓN CON OTROS LENGUAJES:");
    println!("\n   Python:");
    println!("     edad = 9  # Tipo dinámico, se decide en tiempo de ejecución");
    println!("     Ventaja: Flexible");
    println!("     Desventaja: Más lento, errores en tiempo de ejecución");
    
    println!("\n   C++:");
    println!("     int edad = 9;  // Tipo estático, pero puedes hacer cosas peligrosas");
    println!("     Ventaja: Rápido");
    println!("     Desventaja: Puedes causar errores de memoria");
    
    println!("\n   Rust:");
    println!("     let edad: u8 = 9;  // Tipo estático + seguridad");
    println!("     Ventaja: Rápido + Seguro + Sin GC");
    println!("     Desventaja: Más verboso (pero vale la pena)");
    
    println!("\n8. REGLA DE ORO:");
    println!("   - Si Rust puede inferir el tipo → Déjalo inferir");
    println!("   - Si necesitas optimizar memoria → Especifica el tipo");
    println!("   - Si el código no compila → Especifica el tipo explícitamente");
    
    println!("\n   Ejemplos:");
    println!("   ✅ let numero = 42;  // Inferido (i32)");
    println!("   ✅ let edad: u8 = 9;  // Especificado (optimización)");
    println!("   ✅ let vec: Vec<i32> = Vec::new();  // Necesario (no puede inferir)");
}

// ===================== Ejemplos =====================

fn ejemplo_i8_u8() {
    rango("i8", "-128", "127");
    rango("u8", "0", "255");

    // i8: valores pequeños que pueden ser negativos
    let temp_c: i8 = -10;

    // u8: valores pequeños siempre positivos
    let edad: u8 = 9;
    let nota: u8 = 100;

    println!("Ejemplos:");
    println!("  temp_c = {temp_c} (i8)");
    println!("  edad   = {edad} (u8)");
    println!("  nota   = {nota} (u8)");

    // Fuera de rango (no compila):
    // let x: u8 = 300;
}

fn ejemplo_i16_u16() {
    rango("i16", "-32768", "32767");
    rango("u16", "0", "65535");

    let altura_diff_m: i16 = -1500;
    let puerto_tcp: u16 = 8080;
    let año: u16 = 2024;

    println!("Ejemplos:");
    println!("  altura_diff_m = {altura_diff_m} (i16)");
    println!("  puerto_tcp    = {puerto_tcp} (u16)");
    println!("  año           = {año} (u16)");
}

fn ejemplo_i32_u32() {
    rango("i32", "-2147483648", "2147483647");
    rango("u32", "0", "4294967295");
    println!("Nota: si no indicas tipo, Rust suele inferir i32 para enteros.");

    let n = 42; // inferido (normalmente i32)
    let saldo_centavos: i32 = -5_000;
    let poblacion_pais: u32 = 130_000_000;

    println!("Ejemplos:");
    println!("  n             = {n} (inferido)");
    println!("  saldo_centavos= {saldo_centavos} (i32)");
    println!("  poblacion_pais= {poblacion_pais} (u32)");
}

fn ejemplo_i64_u64() {
    rango("i64", "-9_223_372_036_854_775_808", "9_223_372_036_854_775_807");
    rango("u64", "0", "18_446_744_073_709_551_615");

    let poblacion_mundial: i64 = 8_000_000_000;
    let timestamp_unix: i64 = 1_704_067_200;
    let disco_bytes: u64 = 1_000_000_000_000; // ~1TB

    println!("Ejemplos:");
    println!("  poblacion_mundial = {poblacion_mundial} (i64)");
    println!("  timestamp_unix    = {timestamp_unix} (i64)");
    println!("  disco_bytes       = {disco_bytes} (u64)");
}

fn ejemplo_i128_u128() {
    rango("i128", "≈ -1.7e38", "≈ 1.7e38");
    rango("u128", "0", "≈ 3.4e38");
    println!("Úsalos solo si realmente necesitas números enormes (son más pesados).");

    let año_luz_km: i128 = 9_461_000_000_000_000;
    let id: u128 = u128::MAX;

    println!("Ejemplos:");
    println!("  año_luz_km = {año_luz_km} (i128)");
    println!("  u128::MAX  = {id} (u128)");
}

fn ejemplo_isize_usize() {
    println!("usize/isize dependen de la arquitectura (32 o 64 bits).");
    println!("Regla práctica: índices y tamaños -> usize.");

    let vec = vec![1, 2, 3, 4, 5];
    let i: usize = 2;

    let largo = "Hola".len(); // usize
    let offset: isize = -10;

    println!("Ejemplos:");
    println!("  vec[{i}] = {} (usize)", vec[i]);
    println!("  \"Hola\".len() = {largo} (usize)");
    println!("  offset = {offset} (isize)");
}

// ===================== Guía práctica =====================

fn casos_practicos() {
    println!("Reglas rápidas (no perfectas, pero útiles):");
    println!("  - Si no sabes: i32");
    println!("  - Solo positivos pequeños: u8/u16");
    println!("  - Índices / tamaños: usize");
    println!("  - Contadores grandes / tiempo / bytes: u64 o i64 (según si puede ser negativo)");
    println!("  - Muy grande / crypto: u128 (raro)");

    // Mini ejemplos concretos:
    let edad: u8 = 9;
    let temp: i8 = -10;
    let puerto: u16 = 8080;
    let poblacion: u32 = 1_500_000;
    let bytes_disco: u64 = 1_000_000_000_000;

    println!("\nMini ejemplos:");
    println!("  edad = {edad} (u8)");
    println!("  temp = {temp} (i8)");
    println!("  puerto = {puerto} (u16)");
    println!("  poblacion = {poblacion} (u32)");
    println!("  bytes_disco = {bytes_disco} (u64)");
}

fn errores_comunes() {
    println!("1) Overflow (fuera de rango):");
    println!("   // let x: u8 = 300;  // no compila: u8 llega a 255");
    println!("   Solución: usar u16/u32 según corresponda.\n");

    println!("2) Índices con tipo incorrecto:");
    println!("   let v = vec![1, 2, 3];");
    println!("   // let i: u32 = 0;");
    println!("   // v[i]; // no compila: índice debe ser usize");
    println!("   Solución: let i: usize = 0;\n");

    println!("3) Usar tipos gigantes sin necesidad:");
    println!("   let edad: u8 = 25;  // suficiente");
    println!("   // let edad: i128 = 25; // excesivo\n");

    println!("4) Confundir signed/unsigned:");
    println!("   // let t: u8 = -10; // no compila");
    println!("   let t: i8 = -10; // correcto");
}
