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

pub type TablaVariables =
    HashMap<String, VariableInfo>;

/////////////////////
// DIRECTORIO FUNCIONES
/////////////////////

#[derive(Debug)]
pub struct FuncionInfo {
    pub nombre: String,
    pub tipo_retorno: Tipo,
    pub variables: TablaVariables,
}

pub type DirectorioFunciones =
    HashMap<String, FuncionInfo>;

/////////////////////
// CUBO SEMÁNTICO
/////////////////////

type SemanticKey =
    (Tipo, String, Tipo);

pub struct CuboSemantico {
    pub reglas:
        HashMap<SemanticKey, Tipo>,
}

impl CuboSemantico {

    pub fn nuevo() -> Self {

        let mut reglas =
            HashMap::new();

        /////////////////////
        // SUMA
        /////////////////////

        reglas.insert(
            (
                Tipo::Entero,
                "+".to_string(),
                Tipo::Entero
            ),
            Tipo::Entero
        );

        reglas.insert(
            (
                Tipo::Entero,
                "+".to_string(),
                Tipo::Flotante
            ),
            Tipo::Flotante
        );

        reglas.insert(
            (
                Tipo::Flotante,
                "+".to_string(),
                Tipo::Entero
            ),
            Tipo::Flotante
        );

        reglas.insert(
            (
                Tipo::Flotante,
                "+".to_string(),
                Tipo::Flotante
            ),
            Tipo::Flotante
        );

        /////////////////////
        // RESTA
        /////////////////////

        reglas.insert(
            (
                Tipo::Entero,
                "-".to_string(),
                Tipo::Entero
            ),
            Tipo::Entero
        );

        reglas.insert(
            (
                Tipo::Entero,
                "-".to_string(),
                Tipo::Flotante
            ),
            Tipo::Flotante
        );

        reglas.insert(
            (
                Tipo::Flotante,
                "-".to_string(),
                Tipo::Entero
            ),
            Tipo::Flotante
        );

        reglas.insert(
            (
                Tipo::Flotante,
                "-".to_string(),
                Tipo::Flotante
            ),
            Tipo::Flotante
        );

        /////////////////////
        // MULTIPLICACIÓN
        /////////////////////

        reglas.insert(
            (
                Tipo::Entero,
                "*".to_string(),
                Tipo::Entero
            ),
            Tipo::Entero
        );

        reglas.insert(
            (
                Tipo::Entero,
                "*".to_string(),
                Tipo::Flotante
            ),
            Tipo::Flotante
        );

        reglas.insert(
            (
                Tipo::Flotante,
                "*".to_string(),
                Tipo::Entero
            ),
            Tipo::Flotante
        );

        reglas.insert(
            (
                Tipo::Flotante,
                "*".to_string(),
                Tipo::Flotante
            ),
            Tipo::Flotante
        );

        /////////////////////
        // DIVISIÓN
        /////////////////////

        reglas.insert(
            (
                Tipo::Entero,
                "/".to_string(),
                Tipo::Entero
            ),
            Tipo::Flotante
        );

        reglas.insert(
            (
                Tipo::Entero,
                "/".to_string(),
                Tipo::Flotante
            ),
            Tipo::Flotante
        );

        reglas.insert(
            (
                Tipo::Flotante,
                "/".to_string(),
                Tipo::Entero
            ),
            Tipo::Flotante
        );

        reglas.insert(
            (
                Tipo::Flotante,
                "/".to_string(),
                Tipo::Flotante
            ),
            Tipo::Flotante
        );

        /////////////////////
        // RELACIONALES
        /////////////////////

        let relacionales =
            vec![
                ">",
                "<",
                ">=",
                "<=",
                "==",
                "!=",
            ];

        for op in relacionales {

            reglas.insert(
                (
                    Tipo::Entero,
                    op.to_string(),
                    Tipo::Entero
                ),
                Tipo::Bool
            );

            reglas.insert(
                (
                    Tipo::Flotante,
                    op.to_string(),
                    Tipo::Flotante
                ),
                Tipo::Bool
            );

            reglas.insert(
                (
                    Tipo::Entero,
                    op.to_string(),
                    Tipo::Flotante
                ),
                Tipo::Bool
            );

            reglas.insert(
                (
                    Tipo::Flotante,
                    op.to_string(),
                    Tipo::Entero
                ),
                Tipo::Bool
            );
        }

        CuboSemantico {
            reglas
        }
    }

    pub fn validar(
        &self,
        izquierda: Tipo,
        operador: &str,
        derecha: Tipo,
    ) -> Tipo {

        self.reglas
            .get(&(
                izquierda,
                operador.to_string(),
                derecha
            ))
            .cloned()
            .unwrap_or(Tipo::Error)
    }
}

/////////////////////
// TABLA VARIABLES
/////////////////////

pub fn agregar_variable(
    tabla: &mut TablaVariables,
    nombre: String,
    tipo: Tipo,
) {

    if tabla.contains_key(&nombre) {

        println!(
            "Error: variable '{}' duplicada",
            nombre
        );

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

/////////////////////
// DIRECTORIO FUNCIONES
/////////////////////

pub fn agregar_funcion(
    directorio:
        &mut DirectorioFunciones,
    nombre: String,
    tipo_retorno: Tipo,
) {

    if directorio.contains_key(&nombre) {

        println!(
            "Error: función '{}' duplicada",
            nombre
        );

        return;
    }

    directorio.insert(
        nombre.clone(),
        FuncionInfo {
            nombre,
            tipo_retorno,
            variables:
                HashMap::new(),
        },
    );
}

/////////////////////
// CUÁDRUPLOS
/////////////////////

#[derive(Debug, Clone)]
pub struct Cuadruplo {

    pub operador: String,

    pub izquierda: String,

    pub derecha: String,

    pub resultado: String,
}

/////////////////////
// TEMPORALES
/////////////////////

pub struct GeneradorTemporales {

    contador: usize,
}

impl GeneradorTemporales {

    pub fn nuevo() -> Self {

        Self {
            contador: 0
        }
    }

    pub fn nuevo_temp(
        &mut self
    ) -> String {

        self.contador += 1;

        format!(
            "t{}",
            self.contador
        )
    }
}

/////////////////////
// GENERADOR CUÁDRUPLOS
/////////////////////

pub struct GeneradorCuadruplos {

    /////////////////////
    // PILAS
    /////////////////////

    pub operadores:
        Vec<String>,

    pub operandos:
        Vec<String>,

    pub tipos:
        Vec<Tipo>,

    /////////////////////
    // FILA
    /////////////////////

    pub cuadruplos:
        Vec<Cuadruplo>,

    /////////////////////
    // TEMPORALES
    /////////////////////

    pub temporales:
        GeneradorTemporales,
}

impl GeneradorCuadruplos {

    pub fn nuevo() -> Self {

        Self {

            operadores:
                Vec::new(),

            operandos:
                Vec::new(),

            tipos:
                Vec::new(),

            cuadruplos:
                Vec::new(),

            temporales:
                GeneradorTemporales::nuevo(),
        }
    }

    /////////////////////
    // PILAS
    /////////////////////

    pub fn push_operando(
        &mut self,
        valor: String,
        tipo: Tipo,
    ) {

        self.operandos
            .push(valor);

        self.tipos
            .push(tipo);
    }

    pub fn push_operador(
        &mut self,
        op: String,
    ) {

        self.operadores
            .push(op);
    }

    /////////////////////
    // GENERAR OPERACIÓN
    /////////////////////

    pub fn generar_operacion(
        &mut self,
        cubo:
            &CuboSemantico,
    ) {

        let derecha =
            self.operandos
                .pop()
                .unwrap();

        let izquierda =
            self.operandos
                .pop()
                .unwrap();

        let tipo_der =
            self.tipos
                .pop()
                .unwrap();

        let tipo_izq =
            self.tipos
                .pop()
                .unwrap();

        let operador =
            self.operadores
                .pop()
                .unwrap();

        let tipo_resultado =
            cubo.validar(
                tipo_izq.clone(),
                &operador,
                tipo_der.clone(),
            );

        if tipo_resultado
            == Tipo::Error
        {
            panic!(
                "Error semántico"
            );
        }

        let temporal =
            self.temporales
                .nuevo_temp();

        self.cuadruplos.push(
            Cuadruplo {

                operador,

                izquierda,

                derecha,

                resultado:
                    temporal.clone(),
            },
        );

        self.operandos
            .push(
                temporal.clone()
            );

        self.tipos
            .push(
                tipo_resultado
            );
    }

    /////////////////////
    // ASIGNACIÓN
    /////////////////////

    pub fn generar_asignacion(
        &mut self,
        variable: String,
    ) {

        let valor =
            self.operandos
                .pop()
                .unwrap();

        self.tipos.pop();

        self.cuadruplos.push(
            Cuadruplo {

                operador:
                    "=".to_string(),

                izquierda:
                    valor,

                derecha:
                    "-".to_string(),

                resultado:
                    variable,
            },
        );
    }

    /////////////////////
    // PRINT
    /////////////////////

    pub fn generar_print(
        &mut self,
    ) {

        let valor =
            self.operandos
                .pop()
                .unwrap();

        self.tipos.pop();

        self.cuadruplos.push(
            Cuadruplo {

                operador:
                    "PRINT"
                        .to_string(),

                izquierda:
                    "-".to_string(),

                derecha:
                    "-".to_string(),

                resultado:
                    valor,
            },
        );
    }
}