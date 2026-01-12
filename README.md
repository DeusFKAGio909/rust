# 🦀 Rust Learning Project - Tips Avanzados

Este proyecto contiene ejemplos educativos avanzados de Rust que no se enseñan normalmente en tutoriales básicos.

## 📁 Estructura del Proyecto

```
platzi_course/
│
├── Cargo.toml          ← Configuración del proyecto
│                         (dice dónde están los binarios, dependencias y nombres)
│
├── README.md           ← Este archivo
│
├── .gitignore          ← Archivos ignorados por Git
│
├── src/                ← CÓDIGO FUENTE (lo que escribes)
│   │
│   ├── main.rs         ← Binario principal (por defecto)
│   │                    (cargo run ejecuta este)
│   │
│   ├── lib.rs          ← Librería (código reutilizable)
│   │                    (otros proyectos pueden usar esto)
│   │
│   ├── bin/            ← MÚLTIPLES BINARIOS (programas ejecutables)
│   │   │                Convención especial de Cargo
│   │   ├── comparaciones.rs      → cargo run --bin comparaciones
│   │   ├── tipos_enteros.rs      → cargo run --bin tipos_enteros
│   │   ├── tipos_enteros2.rs     → cargo run --bin tipos_enteros2
│   │   ├── features_avanzadas.rs → cargo run --bin features_avanzadas
│   │   └── modulos_demo.rs       → cargo run --bin modulos_demo
│   │
│   └── modulos/        ← TUS MÓDULOS PERSONALES (organización)
│       ├── mod.rs      ← Punto de entrada del módulo
│       ├── matematicas.rs
│       ├── strings.rs
│       └── estructuras.rs
│
└── target/             ← ARCHIVOS COMPILADOS (generados automáticamente)
    └── debug/
        ├── main.exe              ← main.rs compilado
        ├── comparaciones.exe     ← comparaciones.rs compilado
        ├── tipos_enteros.exe     ← tipos_enteros.rs compilado
        ├── tipos_enteros2.exe    ← tipos_enteros2.rs compilado
        ├── features_avanzadas.exe ← features_avanzadas.rs compilado
        └── modulos_demo.exe      ← modulos_demo.rs compilado
```

## 🚀 Cómo Ejecutar

### Binario Principal
```bash
cargo run
# o específicamente:
cargo run --bin main
```

### Otros Binarios
```bash
# Comparaciones Rust vs Python vs C++
cargo run --bin comparaciones

# Guía completa de tipos de enteros (versión original)
cargo run --bin tipos_enteros

# Guía de tipos de enteros (versión mejorada con explicaciones)
cargo run --bin tipos_enteros2

# Features avanzadas (lifetimes, traits, macros, etc.)
cargo run --bin features_avanzadas

# Demostración de módulos y subcarpetas
cargo run --bin modulos_demo
```

## 🎓 Conceptos Cubiertos

### 1. Múltiples Binarios
- Un proyecto puede tener múltiples programas ejecutables
- Cada binario es independiente
- Útil para crear herramientas CLI múltiples en un solo proyecto

### 2. Módulos en Subcarpetas
- `mod.rs` es el punto de entrada (similar a `__init__.py` en Python)
- Organización de código en módulos
- Re-exportación de funciones (`pub use`)

### 3. Librería + Binario
- `lib.rs` convierte el proyecto en una librería
- Puedes tener AMBOS: librería y binarios
- Otros proyectos pueden usar tu código como dependencia

### 4. Comparaciones con Otros Lenguajes
- **Python**: Garbage Collector, duck typing, list comprehensions
- **C++**: Templates, smart pointers, manual memory management
- **Rust**: Ownership, lifetimes, zero-cost abstractions

### 5. Features Avanzadas
- **Lifetimes**: Garantías de validez de referencias
- **Traits**: Similar a interfaces en C++, protocols en Python
- **Macros**: Más poderosas que en C++
- **Closures**: Lambdas con captura de entorno
- **Iterators**: Lazy evaluation
- **Smart Pointers**: Rc, Arc (similar a shared_ptr en C++)

## 💡 Tips Ocultos

### 1. `#[allow(dead_code)]`
Suprime warnings de código no usado. Útil durante desarrollo.

### 2. `pub use` (Re-exportación)
Permite reorganizar tu API pública sin romper código externo.

### 3. `mod.rs` vs Archivos de Módulo
- `mod.rs`: Punto de entrada de un módulo en subcarpeta
- Archivo directo: `mod nombre;` busca `nombre.rs` o `nombre/mod.rs`

### 4. Múltiples `main()` en un Proyecto
Cada `[[bin]]` en `Cargo.toml` puede tener su propio `main()`.

### 5. `edition` en Cargo.toml
- `2015`: Rust original
- `2018`: Mejoras importantes
- `2021`: Actual (recomendado)

## 🔍 Diferencias Clave con Python

| Concepto | Python | Rust |
|----------|--------|------|
| Memoria | GC automático | Ownership system |
| Tipos | Dinámicos | Estáticos (inferencia) |
| Null | `None` | `Option<T>` |
| Errores | Excepciones | `Result<T, E>` |
| Strings | Inmutables | `&str` (slice) vs `String` (owned) |

## 🔍 Diferencias Clave con C++

| Concepto | C++ | Rust |
|----------|-----|------|
| Memoria | Manual/smart pointers | Ownership (automático) |
| Templates | Compile-time | Generics (más seguro) |
| Null | `nullptr` | `Option<T>` |
| Errores | Excepciones/códigos | `Result<T, E>` |
| Unsafe | Todo puede ser unsafe | Solo bloques explícitos |

## 📚 Recursos Adicionales

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Ejercicios interactivos

## 🎯 Próximos Pasos

1. Experimenta modificando los ejemplos
2. Crea tus propios módulos
3. Prueba agregar dependencias externas
4. Explora `cargo doc --open` para documentación generada
