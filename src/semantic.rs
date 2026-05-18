use std::collections::HashMap;

/////////////////////
// TIPOS SEMÁNTICOS
/////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tipo {
    Entero,
    Flotante,
    Bool,
    Error,
}

/////////////////////
// TABLA DE VARIABLES
/////////////////////

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub nombre: String,
    pub tipo: Tipo,
}

pub type TablaVariables = HashMap<String, VariableInfo>;

/////////////////////
// DIRECTORIO DE FUNCIONES
/////////////////////

#[derive(Debug)]
pub struct FuncionInfo {
    pub nombre: String,
    pub tipo_retorno: Tipo,
    pub variables: TablaVariables,
}

pub type DirectorioFunciones = HashMap<String, FuncionInfo>;

/////////////////////
// CUBO SEMÁNTICO
/////////////////////

type SemanticKey = (Tipo, String, Tipo);

pub struct CuboSemantico {
    pub reglas: HashMap<SemanticKey, Tipo>,
}

impl CuboSemantico {
    pub fn nuevo() -> Self {
        let mut reglas = HashMap::new();

        /////////////////////
        // SUMAS
        /////////////////////

        // entero + entero = entero
        reglas.insert(
            (Tipo::Entero, "+".to_string(), Tipo::Entero),
            Tipo::Entero,
        );

        // entero + flotante = flotante
        reglas.insert(
            (Tipo::Entero, "+".to_string(), Tipo::Flotante),
            Tipo::Flotante,
        );

        // flotante + entero = flotante
        reglas.insert(
            (Tipo::Flotante, "+".to_string(), Tipo::Entero),
            Tipo::Flotante,
        );

        // flotante + flotante = flotante
        reglas.insert(
            (Tipo::Flotante, "+".to_string(), Tipo::Flotante),
            Tipo::Flotante,
        );

        /////////////////////
        // RELACIONALES
        /////////////////////

        // entero > entero = bool
        reglas.insert(
            (Tipo::Entero, ">".to_string(), Tipo::Entero),
            Tipo::Bool,
        );

        // flotante > flotante = bool
        reglas.insert(
            (Tipo::Flotante, ">".to_string(), Tipo::Flotante),
            Tipo::Bool,
        );

        CuboSemantico { reglas }
    }

    /////////////////////
    // VALIDAR OPERACIÓN
    /////////////////////

    pub fn validar(
        &self,
        izquierda: Tipo,
        operador: &str,
        derecha: Tipo,
    ) -> Tipo {
        self.reglas
            .get(&(izquierda, operador.to_string(), derecha))
            .cloned()
            .unwrap_or(Tipo::Error)
    }
}

/////////////////////
// FUNCIONES AUXILIARES
/////////////////////

pub fn agregar_variable(
    tabla: &mut TablaVariables,
    nombre: String,
    tipo: Tipo,
) {
    // Validar duplicados
    if tabla.contains_key(&nombre) {
        println!("Error: variable '{}' duplicada", nombre);
        return;
    }

    tabla.insert(
        nombre.clone(),
        VariableInfo {
            nombre,
            tipo,
        },
    );
}

pub fn agregar_funcion(
    directorio: &mut DirectorioFunciones,
    nombre: String,
    tipo_retorno: Tipo,
) {
    // Validar duplicados
    if directorio.contains_key(&nombre) {
        println!("Error: función '{}' duplicada", nombre);
        return;
    }

    directorio.insert(
        nombre.clone(),
        FuncionInfo {
            nombre,
            tipo_retorno,
            variables: HashMap::new(),
        },
    );
}