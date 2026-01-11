// ============================================
// MAIN PRINCIPAL - Punto de entrada por defecto
// ============================================
// 
// EJEMPLOS PRÁCTICOS DE TIPOS DE ENTEROS
// ¿Cuándo usar cada tipo según el valor?

use std::env;

fn main() {
    println!("=== GUÍA PRÁCTICA: TIPOS DE ENTEROS ===\n");
    
    // ========== EJEMPLOS PRÁCTICOS ==========
    
    //  EDAD: u8 es perfecto (0-255 años es suficiente)
    let edad: u8 = 9;  // Una persona de 9 años
    println!("Edad: {} años -> Tipo: u8 (perfecto)", edad);
    println!("   Rango u8: 0 a 255 años (suficiente para cualquier edad humana)");
    
    //  TEMPERATURA: i8 (puede ser negativa)
    let temperatura: i8 = -10;  // 10 grados bajo cero
    println!("\nTemperatura: {}°C -> Tipo: i8 ✅", temperatura);
    println!("   Rango i8: -128 a 127°C (suficiente para temperaturas normales)");
    
    //  AÑO: u16 (años desde 0 hasta 65535)
    let año: u16 = 2024;
    println!("\nAño: {} -> Tipo: u16 ✅", año);
    println!("   Rango u16: 0 a 65535 (cubre muchos años)");
    
    //  POBLACIÓN CIUDAD: u32 (hasta 4 mil millones)
    let poblacion: u32 = 1_500_000;  // 1.5 millones
    println!("\nPoblación ciudad: {} habitantes -> Tipo: u32 (perfecto)", poblacion);
    println!("   Rango u32: 0 a 4,294,967,295 (suficiente para ciudades grandes)");
    
    //  SALDO (puede ser negativo): i32
    let saldo: i32 = -5000;  // Deuda de 5000 unidades
    println!("\nSaldo cuenta: {} -> Tipo: i32 (perfecto)", saldo);
    println!("   Rango i32: -2,147,483,648 a 2,147,483,647");
    
    //  ÍNDICE DE ARRAY: usize (SIEMPRE para índices)
    let numeros = vec![10, 20, 30, 40, 50];
    let indice: usize = 2;  // Tercer elemento
    println!("\nÍndice array: {} -> Tipo: usize (perfecto)", indice);
    println!("   Elemento en índice {}: {}", indice, numeros[indice]);
    println!("   ⚠️ IMPORTANTE: Vec/arrays SIEMPRE usan usize para índices");
    
    // ✅ TIPO POR DEFECTO (si no especificas)
    let numero = 42;  // Rust infiere i32 automáticamente
    println!("\nNúmero sin tipo: {} -> Rust infiere: i32 (perfecto)", numero);
    println!("   i32 es el tipo por defecto en Rust");
    
    // ========== COMPARACIÓN: ¿Por qué u8 para edad? ==========
    println!("\n=== ¿POR QUÉ u8 PARA EDAD? ===");
    println!("Edad de 9 años:");
    println!("  - u8:  Ocupa 1 byte, rango 0-255  Perfecto");
    println!("  - i32: Ocupa 4 bytes, rango enorme  Desperdicio");
    println!("  - i8:  Ocupa 1 byte, pero permite negativos  No necesario");
    println!("  Conclusión: u8 es la mejor opción para edad");
    
    // ========== DEMOSTRACIÓN DE RANGOS ==========
    println!("\n=== RANGOS DE VALORES ===");
    println!("u8:  0 a 255              -> Edad, calificaciones, días del mes");
    println!("i8:  -128 a 127           -> Temperatura, diferencias pequeñas");
    println!("u16: 0 a 65,535           -> Años, puertos TCP, poblaciones pequeñas");
    println!("i16: -32,768 a 32,767     -> Temperaturas extremas, diferencias medianas");
    println!("u32: 0 a 4,294,967,295    -> Poblaciones, tamaños de archivo");
    println!("i32: -2,147M a 2,147M     -> Tipo por defecto, saldos, contadores");
    println!("u64: 0 a 18 quintillones  -> Población mundial, timestamps");
    println!("i64: -9 quintillones a +9 -> Valores muy grandes con signo");
    println!("usize: Depende del sistema -> Índices, tamaños (SIEMPRE para arrays)");
    
    // ========== MENÚ DE OTROS BINARIOS ==========
    println!("\n=== OTROS BINARIOS DISPONIBLES ===");
    println!("  cargo run --bin tipos_enteros      (guía completa de tipos)");
    println!("  cargo run --bin comparaciones      (Rust vs Python vs C++)");
    println!("  cargo run --bin features_avanzadas (conceptos avanzados)");
    println!("  cargo run --bin modulos_demo       (demostración de módulos)");
    
    // Ejemplo con argumentos
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("\nArgumentos recibidos: {:?}", &args[1..]);
    }
}
