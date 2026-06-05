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

    pub local_entero: usize,
    pub local_flotante: usize,

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

            local_entero: 3000,
            local_flotante: 4000,

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

    pub fn nueva_local(&mut self, tipo: &Tipo) -> usize {
        match tipo {
            Tipo::Entero => {
                let dir = self.local_entero;
                self.local_entero += 1;
                dir
            }

            Tipo::Flotante => {
                let dir = self.local_flotante;
                self.local_flotante += 1;
                dir
            }

            _ => panic!("Tipo local no soportado"),
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

        let resultado = if destino == 0 {
            None
        } else {
            Some(destino.to_string())
        };

        self.cuadruplos.push(Cuadruplo {
            operador: Operador::Goto,
            izquierda: None,
            derecha: None,
            resultado,
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

    pub fn generar_param(&mut self, direccion_parametro: String) {
        let valor = self.operandos.pop().unwrap();

        self.tipos.pop();

        self.cuadruplos.push(Cuadruplo {
            operador: Operador::Param,
            izquierda: Some(valor),
            derecha: None,
            resultado: Some(direccion_parametro),
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

/////////////////////
// MÁQUINA VIRTUAL
/////////////////////

#[derive(Debug, Clone)]
pub enum Valor {
    Entero(i64),
    Flotante(f64),
    Bool(bool),
    Vacio,
}

pub struct MaquinaVirtual {
    pub cuadruplos: Vec<Cuadruplo>,
    pub memoria: HashMap<usize, Valor>,
    pub ip: usize,
    pub pila_retorno: Vec<usize>,
}

impl MaquinaVirtual {
    pub fn nueva(cuadruplos: Vec<Cuadruplo>, constantes: &TablaConstantes) -> Self {
        let mut memoria = HashMap::new();

        for constante in constantes.values() {
            let direccion = constante.direccion.unwrap();

            let valor = match constante.tipo {
                Tipo::Entero => Valor::Entero(constante.valor.parse::<i64>().unwrap()),

                Tipo::Flotante => Valor::Flotante(constante.valor.parse::<f64>().unwrap()),

                _ => Valor::Vacio,
            };

            memoria.insert(direccion, valor);
        }

        Self {
            cuadruplos,
            memoria,
            ip: 0,
            pila_retorno: Vec::new(),
        }
    }

    fn direccion(valor: &Option<String>) -> usize {
        valor.as_ref().unwrap().parse::<usize>().unwrap()
    }

    fn obtener(&self, direccion: usize) -> Valor {
        self.memoria
            .get(&direccion)
            .cloned()
            .unwrap_or(Valor::Vacio)
    }

    fn guardar(&mut self, direccion: usize, valor: Valor) {
        self.memoria.insert(direccion, valor);
    }

    fn sumar(a: Valor, b: Valor) -> Valor {
        match (a, b) {
            (Valor::Entero(x), Valor::Entero(y)) => Valor::Entero(x + y),

            (Valor::Entero(x), Valor::Flotante(y)) => Valor::Flotante(x as f64 + y),

            (Valor::Flotante(x), Valor::Entero(y)) => Valor::Flotante(x + y as f64),

            (Valor::Flotante(x), Valor::Flotante(y)) => Valor::Flotante(x + y),

            _ => panic!("Suma inválida"),
        }
    }

    fn restar(a: Valor, b: Valor) -> Valor {
        match (a, b) {
            (Valor::Entero(x), Valor::Entero(y)) => Valor::Entero(x - y),

            (Valor::Entero(x), Valor::Flotante(y)) => Valor::Flotante(x as f64 - y),

            (Valor::Flotante(x), Valor::Entero(y)) => Valor::Flotante(x - y as f64),

            (Valor::Flotante(x), Valor::Flotante(y)) => Valor::Flotante(x - y),

            _ => panic!("Resta inválida"),
        }
    }

    fn multiplicar(a: Valor, b: Valor) -> Valor {
        match (a, b) {
            (Valor::Entero(x), Valor::Entero(y)) => Valor::Entero(x * y),

            (Valor::Entero(x), Valor::Flotante(y)) => Valor::Flotante(x as f64 * y),

            (Valor::Flotante(x), Valor::Entero(y)) => Valor::Flotante(x * y as f64),

            (Valor::Flotante(x), Valor::Flotante(y)) => Valor::Flotante(x * y),

            _ => panic!("Multiplicación inválida"),
        }
    }

    fn dividir(a: Valor, b: Valor) -> Valor {
        match (a, b) {
            (Valor::Entero(x), Valor::Entero(y)) => Valor::Flotante(x as f64 / y as f64),

            (Valor::Entero(x), Valor::Flotante(y)) => Valor::Flotante(x as f64 / y),

            (Valor::Flotante(x), Valor::Entero(y)) => Valor::Flotante(x / y as f64),

            (Valor::Flotante(x), Valor::Flotante(y)) => Valor::Flotante(x / y),

            _ => panic!("División inválida"),
        }
    }

    fn comparar_menor(a: Valor, b: Valor) -> Valor {
        match (a, b) {
            (Valor::Entero(x), Valor::Entero(y)) => Valor::Bool(x < y),

            (Valor::Entero(x), Valor::Flotante(y)) => Valor::Bool((x as f64) < y),

            (Valor::Flotante(x), Valor::Entero(y)) => Valor::Bool(x < y as f64),

            (Valor::Flotante(x), Valor::Flotante(y)) => Valor::Bool(x < y),

            _ => panic!("Comparación inválida"),
        }
    }

    fn comparar_mayor(a: Valor, b: Valor) -> Valor {
        match (a, b) {
            (Valor::Entero(x), Valor::Entero(y)) => Valor::Bool(x > y),

            (Valor::Entero(x), Valor::Flotante(y)) => Valor::Bool((x as f64) > y),

            (Valor::Flotante(x), Valor::Entero(y)) => Valor::Bool(x > y as f64),

            (Valor::Flotante(x), Valor::Flotante(y)) => Valor::Bool(x > y),

            _ => panic!("Comparación inválida"),
        }
    }
    fn comparar_menor_igual(a: Valor, b: Valor) -> Valor {
        match (a, b) {
            (Valor::Entero(x), Valor::Entero(y)) => Valor::Bool(x <= y),
            (Valor::Entero(x), Valor::Flotante(y)) => Valor::Bool((x as f64) <= y),
            (Valor::Flotante(x), Valor::Entero(y)) => Valor::Bool(x <= y as f64),
            (Valor::Flotante(x), Valor::Flotante(y)) => Valor::Bool(x <= y),
            _ => panic!("Comparación <= inválida"),
        }
    }

    fn comparar_mayor_igual(a: Valor, b: Valor) -> Valor {
        match (a, b) {
            (Valor::Entero(x), Valor::Entero(y)) => Valor::Bool(x >= y),
            (Valor::Entero(x), Valor::Flotante(y)) => Valor::Bool((x as f64) >= y),
            (Valor::Flotante(x), Valor::Entero(y)) => Valor::Bool(x >= y as f64),
            (Valor::Flotante(x), Valor::Flotante(y)) => Valor::Bool(x >= y),
            _ => panic!("Comparación >= inválida"),
        }
    }

    fn comparar_diferente(a: Valor, b: Valor) -> Valor {
        match (a, b) {
            (Valor::Entero(x), Valor::Entero(y)) => Valor::Bool(x != y),
            (Valor::Flotante(x), Valor::Flotante(y)) => Valor::Bool(x != y),
            (Valor::Bool(x), Valor::Bool(y)) => Valor::Bool(x != y),
            _ => Valor::Bool(true),
        }
    }
    fn comparar_igual(a: Valor, b: Valor) -> Valor {
        match (a, b) {
            (Valor::Entero(x), Valor::Entero(y)) => Valor::Bool(x == y),

            (Valor::Flotante(x), Valor::Flotante(y)) => Valor::Bool(x == y),

            (Valor::Bool(x), Valor::Bool(y)) => Valor::Bool(x == y),

            _ => Valor::Bool(false),
        }
    }

    pub fn ejecutar(&mut self) {
        while self.ip < self.cuadruplos.len() {
            let cuad = self.cuadruplos[self.ip].clone();

            match cuad.operador {
                Operador::Asignacion => {
                    let origen = Self::direccion(&cuad.izquierda);

                    let destino = Self::direccion(&cuad.resultado);

                    let valor = self.obtener(origen);

                    self.guardar(destino, valor);

                    self.ip += 1;
                }

                Operador::Suma
                | Operador::Resta
                | Operador::Multiplicacion
                | Operador::Division
                | Operador::Menor
                | Operador::Mayor
                | Operador::MenorIgual
                | Operador::MayorIgual
                | Operador::IgualIgual
                | Operador::Diferente => {
                    let izq = Self::direccion(&cuad.izquierda);

                    let der = Self::direccion(&cuad.derecha);

                    let res = Self::direccion(&cuad.resultado);

                    let a = self.obtener(izq);

                    let b = self.obtener(der);

                    let resultado = match cuad.operador {
                        Operador::Suma => Self::sumar(a, b),
                        Operador::Resta => Self::restar(a, b),
                        Operador::Multiplicacion => Self::multiplicar(a, b),
                        Operador::Division => Self::dividir(a, b),
                        Operador::Menor => Self::comparar_menor(a, b),
                        Operador::Mayor => Self::comparar_mayor(a, b),
                        Operador::MenorIgual => Self::comparar_menor_igual(a, b),
                        Operador::MayorIgual => Self::comparar_mayor_igual(a, b),
                        Operador::IgualIgual => Self::comparar_igual(a, b),
                        Operador::Diferente => Self::comparar_diferente(a, b),
                        _ => unreachable!(),
                    };

                    self.guardar(res, resultado);

                    self.ip += 1;
                }

                Operador::Print => {
                    let direccion = Self::direccion(&cuad.resultado);

                    let valor = self.obtener(direccion);

                    println!("OUTPUT: {:?}", valor);

                    self.ip += 1;
                }

                Operador::Goto => {
                    self.ip = cuad.resultado.unwrap().parse::<usize>().unwrap();
                }

                Operador::GotoF => {
                    let direccion_condicion = Self::direccion(&cuad.izquierda);

                    let condicion = self.obtener(direccion_condicion);

                    match condicion {
                        Valor::Bool(false) => {
                            self.ip = cuad.resultado.unwrap().parse::<usize>().unwrap();
                        }

                        Valor::Bool(true) => {
                            self.ip += 1;
                        }

                        _ => {
                            panic!("GOTOF esperaba un valor booleano");
                        }
                    }
                }

                Operador::Era => {
                    self.ip += 1;
                }

                Operador::Param => {
                    let origen = Self::direccion(&cuad.izquierda);

                    let destino = Self::direccion(&cuad.resultado);

                    let valor = self.obtener(origen);

                    self.guardar(destino, valor);

                    self.ip += 1;
                }

                Operador::Gosub => {
                    let destino = cuad.resultado.unwrap().parse::<usize>().unwrap();

                    self.pila_retorno.push(self.ip + 1);

                    self.ip = destino;
                }

                Operador::Return => {
                    if let Some(retorno) = self.pila_retorno.pop() {
                        self.ip = retorno;
                    } else {
                        self.ip += 1;
                    }
                }

                Operador::EndFunc => {
                    if let Some(retorno) = self.pila_retorno.pop() {
                        self.ip = retorno;
                    } else {
                        self.ip += 1;
                    }
                }
            }
        }
    }
}
