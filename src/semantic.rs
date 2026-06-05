#![allow(dead_code)]
use std::collections::HashMap;

/////////////////////
// RESULTADOS
/////////////////////

pub type SemanticResult<T> = Result<T, String>;

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

pub type TablaVariables = HashMap<String, VariableInfo>;

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

pub type TablaConstantes = HashMap<String, ConstanteInfo>;

/////////////////////
// DIRECCIONES VIRTUALES
/////////////////////

pub struct AdministradorDirecciones {
    pub global_entero: usize,
    pub global_flotante: usize,

    pub temporal_entero: usize,
    pub temporal_flotante: usize,
    pub temporal_bool: usize,

    pub constante_entero: usize,
    pub constante_flotante: usize,
}

impl AdministradorDirecciones {
    pub fn nuevo() -> Self {
        Self {
            global_entero: 1000,
            global_flotante: 2000,

            temporal_entero: 5000,
            temporal_flotante: 6000,
            temporal_bool: 7000,

            constante_entero: 9000,
            constante_flotante: 10000,
        }
    }

    pub fn nueva_global(&mut self, tipo: &Tipo) -> usize {
        match tipo {
            Tipo::Entero => {
                let dir = self.global_entero;
                self.global_entero += 1;
                dir
            }

            Tipo::Flotante => {
                let dir = self.global_flotante;
                self.global_flotante += 1;
                dir
            }

            _ => panic!("Tipo global no soportado"),
        }
    }

    pub fn nueva_temporal(&mut self, tipo: &Tipo) -> usize {
        match tipo {
            Tipo::Entero => {
                let dir = self.temporal_entero;
                self.temporal_entero += 1;
                dir
            }

            Tipo::Flotante => {
                let dir = self.temporal_flotante;
                self.temporal_flotante += 1;
                dir
            }

            Tipo::Bool => {
                let dir = self.temporal_bool;
                self.temporal_bool += 1;
                dir
            }

            _ => panic!("Tipo temporal no soportado"),
        }
    }

    pub fn nueva_constante(&mut self, tipo: &Tipo) -> usize {
        match tipo {
            Tipo::Entero => {
                let dir = self.constante_entero;
                self.constante_entero += 1;
                dir
            }

            Tipo::Flotante => {
                let dir = self.constante_flotante;
                self.constante_flotante += 1;
                dir
            }

            _ => panic!("Tipo constante no soportado"),
        }
    }
}

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

pub type DirectorioFunciones = HashMap<String, FuncionInfo>;

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
        Self { contador: 0 }
    }

    pub fn nuevo_temp(&mut self) -> String {
        self.contador += 1;

        format!("t{}", self.contador)
    }
}

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
        // ARITMÉTICOS
        /////////////////////

        let aritmeticos = vec!["+", "-", "*"];

        for op in aritmeticos {
            reglas.insert((Tipo::Entero, op.to_string(), Tipo::Entero), Tipo::Entero);

            reglas.insert(
                (Tipo::Entero, op.to_string(), Tipo::Flotante),
                Tipo::Flotante,
            );

            reglas.insert(
                (Tipo::Flotante, op.to_string(), Tipo::Entero),
                Tipo::Flotante,
            );

            reglas.insert(
                (Tipo::Flotante, op.to_string(), Tipo::Flotante),
                Tipo::Flotante,
            );
        }

        /////////////////////
        // DIVISIÓN
        /////////////////////

        reglas.insert(
            (Tipo::Entero, "/".to_string(), Tipo::Entero),
            Tipo::Flotante,
        );

        reglas.insert(
            (Tipo::Entero, "/".to_string(), Tipo::Flotante),
            Tipo::Flotante,
        );

        reglas.insert(
            (Tipo::Flotante, "/".to_string(), Tipo::Entero),
            Tipo::Flotante,
        );

        reglas.insert(
            (Tipo::Flotante, "/".to_string(), Tipo::Flotante),
            Tipo::Flotante,
        );

        /////////////////////
        // RELACIONALES
        /////////////////////

        let relacionales = vec![">", "<", ">=", "<=", "==", "!="];

        for op in relacionales {
            reglas.insert((Tipo::Entero, op.to_string(), Tipo::Entero), Tipo::Bool);

            reglas.insert((Tipo::Entero, op.to_string(), Tipo::Flotante), Tipo::Bool);

            reglas.insert((Tipo::Flotante, op.to_string(), Tipo::Entero), Tipo::Bool);

            reglas.insert((Tipo::Flotante, op.to_string(), Tipo::Flotante), Tipo::Bool);
        }

        /////////////////////
        // ASIGNACIÓN
        /////////////////////

        reglas.insert((Tipo::Entero, "=".to_string(), Tipo::Entero), Tipo::Entero);

        reglas.insert(
            (Tipo::Flotante, "=".to_string(), Tipo::Flotante),
            Tipo::Flotante,
        );

        reglas.insert(
            (Tipo::Flotante, "=".to_string(), Tipo::Entero),
            Tipo::Flotante,
        );

        Self { reglas }
    }

    pub fn validar(&self, izquierda: Tipo, operador: &str, derecha: Tipo) -> Tipo {
        self.reglas
            .get(&(izquierda, operador.to_string(), derecha))
            .cloned()
            .unwrap_or(Tipo::Error)
    }
}

/////////////////////
// VARIABLES
/////////////////////

pub fn agregar_variable(
    tabla: &mut TablaVariables,
    nombre: String,
    tipo: Tipo,
    direcciones: &mut AdministradorDirecciones,
) {
    if tabla.contains_key(&nombre) {
        println!("Error: variable '{}' duplicada", nombre);

        return;
    }

    let direccion = direcciones.nueva_global(&tipo);

    tabla.insert(
        nombre.clone(),
        VariableInfo {
            nombre,
            tipo,
            direccion: Some(direccion),
        },
    );
}
pub fn buscar_variable(tabla: &TablaVariables, nombre: &str) -> SemanticResult<Tipo> {
    match tabla.get(nombre) {
        Some(v) => Ok(v.tipo.clone()),

        None => Err(format!("Variable '{}' no declarada", nombre)),
    }
}
pub fn buscar_variable_info(tabla: &TablaVariables, nombre: &str) -> SemanticResult<VariableInfo> {
    match tabla.get(nombre) {
        Some(v) => Ok(v.clone()),

        None => Err(format!("Variable '{}' no declarada", nombre)),
    }
}
pub fn registrar_constante(
    tabla: &mut TablaConstantes,
    valor: String,
    tipo: Tipo,
    direcciones: &mut AdministradorDirecciones,
) -> usize {
    if let Some(info) = tabla.get(&valor) {
        return info.direccion.unwrap();
    }

    let direccion = direcciones.nueva_constante(&tipo);

    tabla.insert(
        valor.clone(),
        ConstanteInfo {
            valor,
            tipo,
            direccion: Some(direccion),
        },
    );

    direccion
}

/////////////////////
// FUNCIONES
/////////////////////

pub fn agregar_funcion(directorio: &mut DirectorioFunciones, nombre: String, tipo_retorno: Tipo) {
    if directorio.contains_key(&nombre) {
        println!("Error: función '{}' duplicada", nombre);

        return;
    }

    directorio.insert(
        nombre.clone(),
        FuncionInfo {
            nombre,

            tipo_retorno,

            parametros: Vec::new(),

            variables: HashMap::new(),

            inicio_cuadruplo: None,
        },
    );
}

/////////////////////
// GENERADOR
/////////////////////

pub struct GeneradorCuadruplos {
    pub operadores: Vec<String>,

    pub operandos: Vec<String>,

    pub tipos: Vec<Tipo>,

    pub saltos: Vec<usize>,

    pub cuadruplos: Vec<Cuadruplo>,

    pub temporales: GeneradorTemporales,
}

impl GeneradorCuadruplos {
    pub fn nuevo() -> Self {
        Self {
            operadores: Vec::new(),

            operandos: Vec::new(),

            tipos: Vec::new(),

            saltos: Vec::new(),

            cuadruplos: Vec::new(),

            temporales: GeneradorTemporales::nuevo(),
        }
    }

    pub fn push_operando(&mut self, valor: String, tipo: Tipo) {
        self.operandos.push(valor);

        self.tipos.push(tipo);
    }

    pub fn push_operador(&mut self, op: String) {
        self.operadores.push(op);
    }

    pub fn generar_operacion(
        &mut self,
        cubo: &CuboSemantico,
        direcciones: &mut AdministradorDirecciones,
    ) -> SemanticResult<()> {
        let derecha = self.operandos.pop().unwrap();

        let izquierda = self.operandos.pop().unwrap();

        let tipo_der = self.tipos.pop().unwrap();

        let tipo_izq = self.tipos.pop().unwrap();

        let operador = self.operadores.pop().unwrap();

        let tipo_resultado = cubo.validar(tipo_izq.clone(), &operador, tipo_der.clone());

        if tipo_resultado == Tipo::Error {
            return Err(format!(
                "Operación inválida: {:?} {} {:?}",
                tipo_izq, operador, tipo_der
            ));
        }

        let temporal = direcciones.nueva_temporal(&tipo_resultado).to_string();

        self.cuadruplos.push(Cuadruplo {
            operador: match operador.as_str() {
                "+" => Operador::Suma,

                "-" => Operador::Resta,

                "*" => Operador::Multiplicacion,

                "/" => Operador::Division,

                ">" => Operador::Mayor,

                "<" => Operador::Menor,

                ">=" => Operador::MayorIgual,

                "<=" => Operador::MenorIgual,

                "==" => Operador::IgualIgual,

                "!=" => Operador::Diferente,

                _ => return Err(format!("Operador desconocido {}", operador)),
            },

            izquierda: Some(izquierda),

            derecha: Some(derecha),

            resultado: Some(temporal.clone()),
        });

        self.operandos.push(temporal);

        self.tipos.push(tipo_resultado);

        Ok(())
    }

    pub fn generar_asignacion(
        &mut self,
        variable: String,
        tipo_variable: Tipo,
        cubo: &CuboSemantico,
    ) -> SemanticResult<()> {
        let valor = self.operandos.pop().unwrap();

        let tipo_valor = self.tipos.pop().unwrap();

        let tipo_resultado = cubo.validar(tipo_variable.clone(), "=", tipo_valor.clone());

        if tipo_resultado == Tipo::Error {
            return Err(format!(
                "Asignación inválida: variable {:?} = valor {:?}",
                tipo_variable, tipo_valor
            ));
        }

        self.cuadruplos.push(Cuadruplo {
            operador: Operador::Asignacion,
            izquierda: Some(valor),
            derecha: None,
            resultado: Some(variable),
        });

        Ok(())
    }
    pub fn generar_print(&mut self) {
        let valor = self.operandos.pop().unwrap();

        self.tipos.pop();

        self.cuadruplos.push(Cuadruplo {
            operador: Operador::Print,

            izquierda: None,

            derecha: None,

            resultado: Some(valor),
        });
    }

    pub fn siguiente_cuadruplo(&self) -> usize {
        self.cuadruplos.len()
    }

    pub fn agregar_salto(&mut self, indice: usize) {
        self.saltos.push(indice);
    }

    pub fn sacar_salto(&mut self) -> Option<usize> {
        self.saltos.pop()
    }
    pub fn generar_gotof(&mut self) -> SemanticResult<usize> {
        let condicion = self.operandos.pop().unwrap();

        let tipo_condicion = self.tipos.pop().unwrap();

        if tipo_condicion != Tipo::Bool {
            return Err("La condición debe ser booleana".to_string());
        }

        let indice = self.cuadruplos.len();

        self.cuadruplos.push(Cuadruplo {
            operador: Operador::GotoF,
            izquierda: Some(condicion),
            derecha: None,
            resultado: None,
        });

        Ok(indice)
    }

    pub fn generar_goto(&mut self, destino: usize) -> usize {
        let indice = self.cuadruplos.len();

        self.cuadruplos.push(Cuadruplo {
            operador: Operador::Goto,
            izquierda: None,
            derecha: None,
            resultado: Some(destino.to_string()),
        });

        indice
    }

    pub fn rellenar_salto(&mut self, indice: usize, destino: usize) {
        self.cuadruplos[indice].resultado = Some(destino.to_string());
    }

    pub fn generar_era(&mut self, nombre_funcion: String) {
        self.cuadruplos.push(Cuadruplo {
            operador: Operador::Era,
            izquierda: None,
            derecha: None,
            resultado: Some(nombre_funcion),
        });
    }

    pub fn generar_param(&mut self, numero_parametro: usize) {
        let valor = self.operandos.pop().unwrap();

        self.tipos.pop();

        self.cuadruplos.push(Cuadruplo {
            operador: Operador::Param,
            izquierda: Some(valor),
            derecha: None,
            resultado: Some(numero_parametro.to_string()),
        });
    }

    pub fn generar_gosub(&mut self, nombre_funcion: String, inicio: usize) {
        self.cuadruplos.push(Cuadruplo {
            operador: Operador::Gosub,
            izquierda: Some(nombre_funcion),
            derecha: None,
            resultado: Some(inicio.to_string()),
        });
    }

    pub fn generar_return(&mut self) {
        let valor = self.operandos.pop().unwrap();

        self.tipos.pop();

        self.cuadruplos.push(Cuadruplo {
            operador: Operador::Return,
            izquierda: Some(valor),
            derecha: None,
            resultado: None,
        });
    }

    pub fn generar_endfunc(&mut self) {
        self.cuadruplos.push(Cuadruplo {
            operador: Operador::EndFunc,
            izquierda: None,
            derecha: None,
            resultado: None,
        });
    }
}
