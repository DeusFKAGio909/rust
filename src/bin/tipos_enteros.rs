// ============================================
// GUÍA COMPLETA DE TIPOS DE ENTEROS EN RUST
// ============================================

fn main() {
    println!("=== GUÍA DE TIPOS DE ENTEROS EN RUST ===\n");

    // ========== 1. i8 / u8 (8 bits) ==========
    println!("1. i8 / u8 (8 bits):");
    ejemplo_i8_u8();

    // ========== 2. i16 / u16 (16 bits) ==========
    println!("\n2. i16 / u16 (16 bits):");
    ejemplo_i16_u16();

    // ========== 3. i32 / u32 (32 bits) ==========
    println!("\n3. i32 / u32 (32 bits) - DEFAULT:");
    ejemplo_i32_u32();

    // ========== 4. i64 / u64 (64 bits) ==========
    println!("\n4. i64 / u64 (64 bits):");
    ejemplo_i64_u64();

    // ========== 5. i128 / u128 (128 bits) ==========
    println!("\n5. i128 / u128 (128 bits):");
    ejemplo_i128_u128();

    // ========== 6. isize / usize (tamaño de arquitectura) ==========
    println!("\n6. isize / usize (tamaño de arquitectura):");
    ejemplo_isize_usize();

    // ========== 7. CASOS PRÁCTICOS ==========
    println!("\n7. CASOS PRÁCTICOS - ¿Qué tipo usar?");
    casos_practicos();

    // ========== 8. ERRORES COMUNES ==========
    println!("\n8. ERRORES COMUNES Y CÓMO EVITARLOS:");
    errores_comunes();
}

fn ejemplo_i8_u8() {
    println!("   Rango i8:  -128 a 127");
    println!("   Rango u8:  0 a 255");
    
    // i8: Para valores que pueden ser negativos y pequeños
    let temperatura_celsius: i8 = -10;  // Temperatura puede ser negativa
    let temperatura_fahrenheit: i8 = 32;
    
    // u8: Para valores siempre positivos y pequeños
    let edad: u8 = 9;  // ✅ Perfecto para edad (0-255 años)
    let calificacion: u8 = 100;  // Calificación 0-100
    let dias_mes: u8 = 31;  // Días en un mes
    
    println!("   Edad: {} años (u8)", edad);
    println!("   Temperatura: {}°C (i8)", temperatura_celsius);
    println!("   Temperatura Fahrenheit: {}°F (i8)", temperatura_fahrenheit);
    println!("   Calificación: {}% (u8)", calificacion);
    println!("   Días en mes: {} (u8)", dias_mes);
    
    // ⚠️ ERROR si intentas valores fuera de rango:
    // let edad_invalida: u8 = 300;  // ❌ ERROR: overflow
    // let temp_invalida: i8 = 200;  // ❌ ERROR: overflow
}

fn ejemplo_i16_u16() {
    println!("   Rango i16:  -32,768 a 32,767");
    println!("   Rango u16:  0 a 65,535");
    
    // i16: Para valores negativos medianos
    let temperatura_extrema: i16 = -50;  // Temperaturas muy frías
    let diferencia_altura: i16 = -1500;  // Diferencia en metros
    
    // u16: Para valores positivos medianos
    let año: u16 = 2024;  // Años (0-65535)
    let puerto_tcp: u16 = 8080;  // Puertos de red (0-65535)
    let poblacion_ciudad_pequeña: u16 = 50000;  // Población pequeña
    
    println!("   Año: {} (u16)", año);
    println!("   Puerto TCP: {} (u16)", puerto_tcp);
    println!("   Temperatura extrema: {}°C (i16)", temperatura_extrema);
    println!("   Diferencia altura: {}m (i16)", diferencia_altura);
    println!("   Población ciudad pequeña: {} (u16)", poblacion_ciudad_pequeña);
}

fn ejemplo_i32_u32() {
    println!("   Rango i32:  -2,147,483,648 a 2,147,483,647");
    println!("   Rango u32:  0 a 4,294,967,295");
    println!("   ⭐ ESTE ES EL TIPO POR DEFECTO EN RUST");
    
    // i32: Tipo por defecto para enteros con signo
    let numero = 42;  // Rust infiere i32 por defecto
    let poblacion_ciudad: i32 = 1_500_000;  // 1.5 millones
    let saldo_cuenta: i32 = -5000;  // Puede ser negativo
    
    // u32: Para valores grandes siempre positivos
    let poblacion_pais: u32 = 130_000_000;  // 130 millones
    let tamaño_archivo_bytes: u32 = 2_147_483_647;  // ~2GB
    
    println!("   Número (inferido): {} (i32)", numero);
    println!("   Población ciudad: {} (i32)", poblacion_ciudad);
    println!("   Población país: {} (u32)", poblacion_pais);
    println!("   Saldo cuenta: {} (i32)", saldo_cuenta);
    println!("   Tamaño archivo: {} bytes (~2GB) (u32)", tamaño_archivo_bytes);
    
    // Nota: En la mayoría de casos, i32 es suficiente
}

fn ejemplo_i64_u64() {
    println!("   Rango i64:  -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807");
    println!("   Rango u64:  0 a 18,446,744,073,709,551,615");
    
    // i64: Para valores muy grandes que pueden ser negativos
    let poblacion_mundial: i64 = 8_000_000_000;  // 8 mil millones
    let timestamp_unix: i64 = 1_704_067_200;  // Timestamps grandes
    let distancia_km: i64 = 150_000_000;  // Distancia a Marte en km
    
    // u64: Para valores enormes siempre positivos
    let tamaño_disco_bytes: u64 = 1_000_000_000_000;  // 1TB en bytes
    let transacciones_globales: u64 = 1_000_000_000_000_000;  // 1 billón
    
    println!("   Población mundial: {} (i64)", poblacion_mundial);
    println!("   Tamaño disco: {} bytes (1TB) (u64)", tamaño_disco_bytes);
    println!("   Timestamp Unix: {} (i64)", timestamp_unix);
    println!("   Distancia a Marte: {} km (i64)", distancia_km);
    println!("   Transacciones globales: {} (u64)", transacciones_globales);
}

fn ejemplo_i128_u128() {
    println!("   Rango i128:  -170,141,183,460,469,231,731,687,303,715,884,105,728 a ...");
    println!("   Rango u128:  0 a 340,282,366,920,938,463,463,374,607,431,768,211,455");
    println!("   ⚠️ Usar solo cuando realmente necesites números ENORMES");
    
    // i128: Para cálculos científicos o financieros muy grandes
    let distancia_astronomica: i128 = 9_461_000_000_000_000;  // 1 año luz en km
    let calculo_cientifico: i128 = 1_000_000_000_000_000_000_000;  // 1 sextillón
    
    // u128: Para IDs únicos globales, hashes, criptografía
    let id_unico_global: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    
    println!("   Distancia astronómica: {} km (1 año luz) (i128)", distancia_astronomica);
    println!("   Cálculo científico: {} (i128)", calculo_cientifico);
    println!("   ID único: {} (u128)", id_unico_global);
    
    // Nota: i128/u128 son más lentos, úsalos solo cuando sea necesario
}

fn ejemplo_isize_usize() {
    println!("   isize/usize: Tamaño depende de tu arquitectura");
    println!("   - 32-bit: 32 bits (como i32/u32)");
    println!("   - 64-bit: 64 bits (como i64/u64)");
    println!("   ⭐ USAR PARA: Índices de arrays, tamaños, offsets");
    
    // usize: Para índices y tamaños (siempre positivo)
    let tamaño_array: usize = 1000;
    let indice: usize = 5;
    let longitud_string: usize = "Hola".len();
    
    // isize: Para offsets que pueden ser negativos
    let offset: isize = -10;
    
    println!("   Tamaño array: {} (usize)", tamaño_array);
    println!("   Índice: {} (usize)", indice);
    println!("   Longitud string: {} (usize)", longitud_string);
    println!("   Offset: {} (isize)", offset);
    
    // ⚠️ IMPORTANTE: Vec, arrays, slices usan usize para índices
    let vec = vec![1, 2, 3, 4, 5];
    let indice: usize = 2;  // ✅ Correcto
    // let indice: u32 = 2;  // ❌ ERROR: necesitas usize
    println!("   Elemento en índice {}: {}", indice, vec[indice]);
}

fn casos_practicos() {
    println!("\n   📋 GUÍA RÁPIDA: ¿Qué tipo usar?");
    println!("\n   EDAD:");
    println!("      Edad de persona: u8 (0-255 años es suficiente)");
    let edad: u8 = 9;
    println!("      Ejemplo: {} años -> u8 ✅", edad);
    
    println!("\n   TEMPERATURA:");
    println!("      Temperatura Celsius: i8 (puede ser negativa, rango -128 a 127)");
    let temp: i8 = -10;
    println!("      Ejemplo: {}°C -> i8 ✅", temp);
    
    println!("\n   AÑO:");
    println!("      Año calendario: u16 (0-65535 cubre desde año 0 hasta 65535)");
    let año: u16 = 2024;
    println!("      Ejemplo: {} -> u16 ✅", año);
    
    println!("\n   POBLACIÓN:");
    println!("      Ciudad pequeña: u16 (hasta 65,535 personas)");
    println!("      Ciudad grande: u32 (hasta 4 mil millones)");
    println!("      País/Mundo: u64 (hasta 18 quintillones)");
    let poblacion_ciudad: u32 = 1_500_000;
    println!("      Ejemplo: {} habitantes -> u32 ✅", poblacion_ciudad);
    
    println!("\n   DINERO:");
    println!("      Centavos en cuenta pequeña: i32");
    println!("      Centavos en cuenta grande: i64");
    let saldo_centavos: i32 = -5000;  // -50.00 unidades
    println!("      Ejemplo: {} centavos -> i32 ✅", saldo_centavos);
    
    println!("\n   ÍNDICES Y TAMAÑOS:");
    println!("      SIEMPRE usa usize para índices de arrays/vecs");
    let indice: usize = 0;
    println!("      Ejemplo: índice {} -> usize ✅", indice);
    
    println!("\n   REGLA GENERAL:");
    println!("      - Si no estás seguro: usa i32 (tipo por defecto)");
    println!("      - Si es siempre positivo y pequeño: u8/u16");
    println!("      - Si es índice o tamaño: usize");
    println!("      - Si es muy grande: i64/u64");
    println!("      - Si es ENORME: i128/u128 (raro)");
}

fn errores_comunes() {
    println!("\n   ⚠️ ERRORES COMUNES:");
    
    println!("\n   1. OVERFLOW (valores fuera de rango):");
    println!("      let edad: u8 = 300;  // ❌ ERROR: u8 solo llega a 255");
    println!("      Solución: usar u16 o u32");
    
    println!("\n   2. USAR TIPO INCORRECTO PARA ÍNDICES:");
    println!("      let vec = vec![1, 2, 3];");
    println!("      let i: u32 = 0;");
    println!("      vec[i];  // ❌ ERROR: necesita usize");
    println!("      Solución: let i: usize = 0;");
    
    println!("\n   3. USAR TIPO MUY GRANDE INNECESARIAMENTE:");
    println!("      let edad: i128 = 25;  // ❌ Ineficiente");
    println!("      Solución: let edad: u8 = 25;  // ✅ Suficiente");
    
    println!("\n   4. NO ESPECIFICAR TIPO CUANDO ES NECESARIO:");
    println!("      let numero = 100;  // Rust infiere i32");
    println!("      Si necesitas u8 específicamente:");
    println!("      let numero: u8 = 100;  // ✅ Explícito");
    
    println!("\n   5. CONFUNDIR SIGNED/UNSIGNED:");
    println!("      let temperatura: u8 = -10;  // ❌ ERROR: u8 no puede ser negativo");
    println!("      Solución: let temperatura: i8 = -10;  // ✅");
    
    println!("\n   💡 TIP: Deja que Rust infiera el tipo cuando sea posible:");
    println!("      let edad = 9;  // Rust usa i32 por defecto");
    println!("      Si necesitas optimizar memoria, especifica:");
    println!("      let edad: u8 = 9;  // Ocupa menos memoria");
}
