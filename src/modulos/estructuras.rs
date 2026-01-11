// Estructuras y enums - conceptos poderosos de Rust

// Struct similar a class en Python/C++, pero más ligero
#[derive(Debug, Clone)] // Macros automáticas (como decoradores en Python)
pub struct Persona {
    pub nombre: String,
    pub edad: u32,
    // campos privados por defecto si no tienen 'pub'
    activo: bool,
}

impl Persona {
    // Constructor (convención: new)
    pub fn new(nombre: String, edad: u32) -> Self {
        Persona {
            nombre,
            edad,
            activo: true,
        }
    }

    // Método que toma &self (referencia inmutable)
    pub fn presentarse(&self) -> String {
        format!("Soy {}, tengo {} años", self.nombre, self.edad)
    }

    // Método que toma &mut self (referencia mutable)
    pub fn desactivar(&mut self) {
        self.activo = false;
    }

    // Método que toma self (consume el objeto)
    pub fn into_nombre(self) -> String {
        self.nombre // Consume la struct, retorna solo el nombre
    }
}

// Enum - más poderoso que en C++, similar a Union Types en Python 3.10+
#[derive(Debug)]
pub enum Estado {
    Activo,
    Inactivo,
    Pendiente { dias: u32 }, // Variante con datos
    Error(String),           // Variante con String
}

impl Estado {
    pub fn es_activo(&self) -> bool {
        match self {
            Estado::Activo => true,
            _ => false,
        }
    }
}

// Pattern matching (similar a switch en C++, pero más poderoso)
pub fn procesar_estado(estado: &Estado) -> String {
    match estado {
        Estado::Activo => "Todo bien".to_string(),
        Estado::Inactivo => "Necesita atención".to_string(),
        Estado::Pendiente { dias } => format!("Esperando {} días", dias),
        Estado::Error(msg) => format!("Error: {}", msg),
    }
}
