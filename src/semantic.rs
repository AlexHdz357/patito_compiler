use std::collections::HashMap;

/////////////////////
// RESULTADOS
/////////////////////

pub type SemanticResult<T> =
    Result<T, String>;

/////////////////////
// TIPOS SEMÁNTICOS
/////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tipo {
    Entero,
    Flotante,
    Bool,
    Void,
    Error,
}

/////////////////////
// VARIABLES
/////////////////////

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub nombre: String,
    pub tipo: Tipo,

    // Etapa 4
    pub direccion: Option<usize>,
}

pub type TablaVariables =
    HashMap<String, VariableInfo>;

/////////////////////
// CONSTANTES
/////////////////////

#[derive(Debug, Clone)]
pub struct ConstanteInfo {
    pub valor: String,
    pub tipo: Tipo,

    // Etapa 4
    pub direccion: Option<usize>,
}

pub type TablaConstantes =
    HashMap<String, ConstanteInfo>;

/////////////////////
// FUNCIONES
/////////////////////

#[derive(Debug)]
pub struct FuncionInfo {
    pub nombre: String,

    pub tipo_retorno: Tipo,

    pub parametros: Vec<Tipo>,

    pub variables: TablaVariables,

    // Etapa 4
    pub inicio_cuadruplo: Option<usize>,
}

pub type DirectorioFunciones =
    HashMap<String, FuncionInfo>;

/////////////////////
// OPERADORES
/////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum Operador {
    Suma,
    Resta,
    Multiplicacion,
    Division,

    Mayor,
    Menor,
    MayorIgual,
    MenorIgual,
    IgualIgual,
    Diferente,

    Asignacion,

    Print,

    Goto,
    GotoF,

    Era,
    Param,
    Gosub,

    Return,
    EndFunc,
}

/////////////////////
// CUÁDRUPLOS
/////////////////////

#[derive(Debug, Clone)]
pub struct Cuadruplo {
    pub operador: Operador,

    pub izquierda: Option<String>,

    pub derecha: Option<String>,

    pub resultado: Option<String>,
}

/////////////////////
// TEMPORALES
/////////////////////

pub struct GeneradorTemporales {
    pub contador: usize,
}

impl GeneradorTemporales {

    pub fn nuevo() -> Self {
        Self {
            contador: 0,
        }
    }

    pub fn nuevo_temp(
        &mut self,
    ) -> String {

        self.contador += 1;

        format!(
            "t{}",
            self.contador
        )
    }
}

/////////////////////
// CUBO SEMÁNTICO
/////////////////////

type SemanticKey =
    (Tipo, String, Tipo);

pub struct CuboSemantico {

    pub reglas:
        HashMap<
            SemanticKey,
            Tipo
        >,
}

impl CuboSemantico {

    pub fn nuevo() -> Self {

        let mut reglas =
            HashMap::new();

        /////////////////////
        // ARITMÉTICOS
        /////////////////////

        let aritmeticos =
            vec![
                "+",
                "-",
                "*",
            ];

        for op in aritmeticos {

            reglas.insert(
                (
                    Tipo::Entero,
                    op.to_string(),
                    Tipo::Entero
                ),
                Tipo::Entero
            );

            reglas.insert(
                (
                    Tipo::Entero,
                    op.to_string(),
                    Tipo::Flotante
                ),
                Tipo::Flotante
            );

            reglas.insert(
                (
                    Tipo::Flotante,
                    op.to_string(),
                    Tipo::Entero
                ),
                Tipo::Flotante
            );

            reglas.insert(
                (
                    Tipo::Flotante,
                    op.to_string(),
                    Tipo::Flotante
                ),
                Tipo::Flotante
            );
        }

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

            reglas.insert(
                (
                    Tipo::Flotante,
                    op.to_string(),
                    Tipo::Flotante
                ),
                Tipo::Bool
            );
        }

        /////////////////////
        // ASIGNACIÓN
        /////////////////////

        reglas.insert(
            (
                Tipo::Entero,
                "=".to_string(),
                Tipo::Entero
            ),
            Tipo::Entero
        );

        reglas.insert(
            (
                Tipo::Flotante,
                "=".to_string(),
                Tipo::Flotante
            ),
            Tipo::Flotante
        );

        reglas.insert(
            (
                Tipo::Flotante,
                "=".to_string(),
                Tipo::Entero
            ),
            Tipo::Flotante
        );

        Self {
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
            .unwrap_or(
                Tipo::Error
            )
    }
}

/////////////////////
// VARIABLES
/////////////////////

pub fn agregar_variable(
    tabla: &mut TablaVariables,
    nombre: String,
    tipo: Tipo,
) {

    if tabla.contains_key(&nombre)
    {
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
            direccion: None,
        }
    );
}

pub fn buscar_variable(
    tabla: &TablaVariables,
    nombre: &str,
) -> SemanticResult<Tipo> {

    match tabla.get(nombre) {

        Some(v) =>
            Ok(
                v.tipo.clone()
            ),

        None =>
            Err(
                format!(
                    "Variable '{}' no declarada",
                    nombre
                )
            ),
    }
}

/////////////////////
// FUNCIONES
/////////////////////

pub fn agregar_funcion(
    directorio:
        &mut DirectorioFunciones,

    nombre: String,

    tipo_retorno: Tipo,
) {

    if directorio.contains_key(
        &nombre
    ) {

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

            parametros:
                Vec::new(),

            variables:
                HashMap::new(),

            inicio_cuadruplo:
                None,
        }
    );
}

/////////////////////
// GENERADOR
/////////////////////

pub struct GeneradorCuadruplos {

    pub operadores:
        Vec<String>,

    pub operandos:
        Vec<String>,

    pub tipos:
        Vec<Tipo>,

    pub saltos:
        Vec<usize>,

    pub cuadruplos:
        Vec<Cuadruplo>,

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

            saltos:
                Vec::new(),

            cuadruplos:
                Vec::new(),

            temporales:
                GeneradorTemporales::nuevo(),
        }
    }

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

    pub fn generar_operacion(
        &mut self,
        cubo:
            &CuboSemantico,
    ) -> SemanticResult<()> {

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
            return Err(
                format!(
                    "Operación inválida: {:?} {} {:?}",
                    tipo_izq,
                    operador,
                    tipo_der
                )
            );
        }

        let temporal =
            self.temporales
                .nuevo_temp();

        self.cuadruplos.push(
            Cuadruplo {

                operador:
                    match operador.as_str()
                    {
                        "+" =>
                            Operador::Suma,

                        "-" =>
                            Operador::Resta,

                        "*" =>
                            Operador::Multiplicacion,

                        "/" =>
                            Operador::Division,

                        ">" =>
                            Operador::Mayor,

                        "<" =>
                            Operador::Menor,

                        ">=" =>
                            Operador::MayorIgual,

                        "<=" =>
                            Operador::MenorIgual,

                        "==" =>
                            Operador::IgualIgual,

                        "!=" =>
                            Operador::Diferente,

                        _ =>
                            return Err(
                                format!(
                                    "Operador desconocido {}",
                                    operador
                                )
                            ),
                    },

                izquierda:
                    Some(
                        izquierda
                    ),

                derecha:
                    Some(
                        derecha
                    ),

                resultado:
                    Some(
                        temporal.clone()
                    ),
            }
        );

        self.operandos.push(
            temporal
        );

        self.tipos.push(
            tipo_resultado
        );

        Ok(())
    }

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
                    Operador::Asignacion,

                izquierda:
                    Some(valor),

                derecha:
                    None,

                resultado:
                    Some(variable),
            }
        );
    }

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
                    Operador::Print,

                izquierda:
                    None,

                derecha:
                    None,

                resultado:
                    Some(valor),
            }
        );
    }

    pub fn siguiente_cuadruplo(
        &self,
    ) -> usize {

        self.cuadruplos.len()
    }

    pub fn agregar_salto(
        &mut self,
        indice: usize,
    ) {

        self.saltos.push(
            indice
        );
    }

    pub fn sacar_salto(
        &mut self,
    ) -> Option<usize> {

        self.saltos.pop()
    }
}