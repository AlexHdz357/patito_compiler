mod semantic;

use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use semantic::*;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "patito.pest"]
struct PatitoParser;

#[derive(Debug, Clone)]
struct ParamMeta {
    nombre: String,
    tipo: Tipo,
    direccion: String,
}

#[derive(Debug, Clone)]
struct FuncionMeta {
    inicio: usize,
    tipo_retorno: Tipo,
    parametros: Vec<ParamMeta>,
}

fn tipo_desde_str(s: &str) -> Tipo {
    match s {
        "entero" => Tipo::Entero,
        "flotante" => Tipo::Flotante,
        _ => Tipo::Error,
    }
}

fn obtener_variable(nombre: &str, tabla: &TablaVariables) -> (Tipo, String) {
    match buscar_variable_info(tabla, nombre) {
        Ok(info) => (info.tipo, info.direccion.unwrap().to_string()),
        Err(msg) => panic!("{}", msg),
    }
}

fn procesar_llamada_expr(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    funciones: &HashMap<String, FuncionMeta>,
) {
    let mut inner = pair.into_inner();
    let nombre_funcion = inner.next().unwrap().as_str().to_string();

    let meta = funciones
        .get(&nombre_funcion)
        .unwrap_or_else(|| panic!("Función '{}' no declarada", nombre_funcion))
        .clone();

    generador.generar_era(nombre_funcion.clone());

    let mut contador_param = 0;

    for nodo in inner {
        if nodo.as_rule() == Rule::argumentos {
            for arg in nodo.into_inner() {
                if arg.as_rule() == Rule::expresion {
                    procesar_expresion(arg, tabla, constantes, direcciones, generador, funciones);

                    let destino_param = meta
                        .parametros
                        .get(contador_param)
                        .unwrap_or_else(|| {
                            panic!("Demasiados argumentos para función '{}'", nombre_funcion)
                        })
                        .direccion
                        .clone();

                    generador.generar_param(destino_param);
                    contador_param += 1;
                }
            }
        }
    }

    if contador_param != meta.parametros.len() {
        panic!(
            "Cantidad incorrecta de argumentos para función '{}': esperados {}, recibidos {}",
            nombre_funcion,
            meta.parametros.len(),
            contador_param
        );
    }

    let temporal_retorno = direcciones.nueva_temporal(&meta.tipo_retorno).to_string();

    generador.generar_gosub_con_retorno(nombre_funcion, meta.inicio, temporal_retorno.clone());
    generador.push_operando(temporal_retorno, meta.tipo_retorno);
}

fn procesar_factor(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    funciones: &HashMap<String, FuncionMeta>,
) {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::llamada_expr => {
            procesar_llamada_expr(inner, tabla, constantes, direcciones, generador, funciones);
        }

        Rule::id => {
            let nombre = inner.as_str().to_string();
            let (tipo, direccion) = obtener_variable(&nombre, tabla);
            generador.push_operando(direccion, tipo);
        }

        Rule::cte => {
            let valor = inner.as_str().to_string();
            let tipo = if valor.contains('.') {
                Tipo::Flotante
            } else {
                Tipo::Entero
            };

            let direccion = registrar_constante(constantes, valor, tipo.clone(), direcciones);
            generador.push_operando(direccion.to_string(), tipo);
        }

        Rule::expresion => {
            procesar_expresion(inner, tabla, constantes, direcciones, generador, funciones);
        }

        _ => {}
    }
}

fn procesar_termino(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    cubo: &CuboSemantico,
    funciones: &HashMap<String, FuncionMeta>,
) {
    let mut inner = pair.into_inner();
    let primero = inner.next().unwrap();

    procesar_factor(
        primero,
        tabla,
        constantes,
        direcciones,
        generador,
        funciones,
    );

    while let Some(op) = inner.next() {
        let operador = op.as_str().to_string();
        let siguiente = inner.next().unwrap();

        procesar_factor(
            siguiente,
            tabla,
            constantes,
            direcciones,
            generador,
            funciones,
        );
        generador.push_operador(operador);
        generador.generar_operacion(cubo, direcciones).unwrap();
    }
}

fn procesar_exp(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    cubo: &CuboSemantico,
    funciones: &HashMap<String, FuncionMeta>,
) {
    let mut inner = pair.into_inner();
    let primero = inner.next().unwrap();

    procesar_termino(
        primero,
        tabla,
        constantes,
        direcciones,
        generador,
        cubo,
        funciones,
    );

    while let Some(op) = inner.next() {
        let operador = op.as_str().to_string();
        let siguiente = inner.next().unwrap();

        procesar_termino(
            siguiente,
            tabla,
            constantes,
            direcciones,
            generador,
            cubo,
            funciones,
        );
        generador.push_operador(operador);
        generador.generar_operacion(cubo, direcciones).unwrap();
    }
}

fn procesar_expresion(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    funciones: &HashMap<String, FuncionMeta>,
) {
    let cubo = CuboSemantico::nuevo();

    let mut inner = pair.into_inner();
    let izquierda = inner.next().unwrap();

    procesar_exp(
        izquierda,
        tabla,
        constantes,
        direcciones,
        generador,
        &cubo,
        funciones,
    );

    if let Some(op_rel) = inner.next() {
        let operador = op_rel.as_str().to_string();
        let derecha = inner.next().unwrap();

        procesar_exp(
            derecha,
            tabla,
            constantes,
            direcciones,
            generador,
            &cubo,
            funciones,
        );

        generador.push_operador(operador);
        generador.generar_operacion(&cubo, direcciones).unwrap();
    }
}

fn procesar_asignacion(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    funciones: &HashMap<String, FuncionMeta>,
) {
    let cubo = CuboSemantico::nuevo();

    let mut inner = pair.into_inner();
    let variable = inner.next().unwrap().as_str().to_string();
    let (tipo_variable, direccion_variable) = obtener_variable(&variable, tabla);
    let expresion = inner.next().unwrap();

    procesar_expresion(
        expresion,
        tabla,
        constantes,
        direcciones,
        generador,
        funciones,
    );

    generador
        .generar_asignacion(direccion_variable, tipo_variable, &cubo)
        .unwrap();
}

fn procesar_print(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    funciones: &HashMap<String, FuncionMeta>,
) {
    for nodo in pair.into_inner() {
        if nodo.as_rule() == Rule::expresion {
            procesar_expresion(nodo, tabla, constantes, direcciones, generador, funciones);
            generador.generar_print();
            return;
        }
    }

    panic!("Error: no se encontró expresión en escribe()");
}

fn procesar_retorno(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    funciones: &HashMap<String, FuncionMeta>,
) {
    for nodo in pair.into_inner() {
        if nodo.as_rule() == Rule::expresion {
            procesar_expresion(nodo, tabla, constantes, direcciones, generador, funciones);
            generador.generar_return();
            return;
        }
    }

    panic!("Error: return sin expresión");
}

fn procesar_llamada(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    funciones: &HashMap<String, FuncionMeta>,
) {
    let mut inner = pair.into_inner();
    let nombre_funcion = inner.next().unwrap().as_str().to_string();

    let meta = funciones
        .get(&nombre_funcion)
        .unwrap_or_else(|| panic!("Función '{}' no declarada", nombre_funcion))
        .clone();

    generador.generar_era(nombre_funcion.clone());

    let mut contador_param = 0;

    for nodo in inner {
        if nodo.as_rule() == Rule::argumentos {
            for arg in nodo.into_inner() {
                if arg.as_rule() == Rule::expresion {
                    procesar_expresion(arg, tabla, constantes, direcciones, generador, funciones);

                    let destino_param = meta
                        .parametros
                        .get(contador_param)
                        .unwrap_or_else(|| {
                            panic!("Demasiados argumentos para función '{}'", nombre_funcion)
                        })
                        .direccion
                        .clone();

                    generador.generar_param(destino_param);
                    contador_param += 1;
                }
            }
        }
    }

    if contador_param != meta.parametros.len() {
        panic!(
            "Cantidad incorrecta de argumentos para función '{}': esperados {}, recibidos {}",
            nombre_funcion,
            meta.parametros.len(),
            contador_param
        );
    }

    generador.generar_gosub(nombre_funcion, meta.inicio);
}

fn procesar_condicion(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    funciones: &HashMap<String, FuncionMeta>,
) {
    let mut expresion_condicion: Option<Pair<Rule>> = None;
    let mut cuerpos: Vec<Pair<Rule>> = Vec::new();

    for nodo in pair.into_inner() {
        match nodo.as_rule() {
            Rule::expresion => expresion_condicion = Some(nodo),
            Rule::cuerpo => cuerpos.push(nodo),
            _ => {}
        }
    }

    procesar_expresion(
        expresion_condicion.unwrap(),
        tabla,
        constantes,
        direcciones,
        generador,
        funciones,
    );

    let salto_falso = generador.generar_gotof().unwrap();

    procesar_cuerpo(
        cuerpos.remove(0),
        tabla,
        constantes,
        direcciones,
        generador,
        funciones,
    );

    if !cuerpos.is_empty() {
        let salto_fin = generador.generar_goto(0);
        let inicio_sino = generador.siguiente_cuadruplo();

        generador.rellenar_salto(salto_falso, inicio_sino);

        procesar_cuerpo(
            cuerpos.remove(0),
            tabla,
            constantes,
            direcciones,
            generador,
            funciones,
        );

        let fin = generador.siguiente_cuadruplo();
        generador.rellenar_salto(salto_fin, fin);
    } else {
        let fin = generador.siguiente_cuadruplo();
        generador.rellenar_salto(salto_falso, fin);
    }
}

fn procesar_ciclo(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    funciones: &HashMap<String, FuncionMeta>,
) {
    let inicio_ciclo = generador.siguiente_cuadruplo();

    let mut expresion_condicion: Option<Pair<Rule>> = None;
    let mut cuerpo_ciclo: Option<Pair<Rule>> = None;

    for nodo in pair.into_inner() {
        match nodo.as_rule() {
            Rule::expresion => expresion_condicion = Some(nodo),
            Rule::cuerpo => cuerpo_ciclo = Some(nodo),
            _ => {}
        }
    }

    procesar_expresion(
        expresion_condicion.unwrap(),
        tabla,
        constantes,
        direcciones,
        generador,
        funciones,
    );

    let salto_falso = generador.generar_gotof().unwrap();

    procesar_cuerpo(
        cuerpo_ciclo.unwrap(),
        tabla,
        constantes,
        direcciones,
        generador,
        funciones,
    );

    generador.generar_goto(inicio_ciclo);

    let fin = generador.siguiente_cuadruplo();
    generador.rellenar_salto(salto_falso, fin);
}

fn procesar_cuerpo(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    funciones: &HashMap<String, FuncionMeta>,
) {
    for estatuto in pair.into_inner() {
        let inner = estatuto.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::asigna => {
                procesar_asignacion(inner, tabla, constantes, direcciones, generador, funciones);
            }

            Rule::imprime => {
                procesar_print(inner, tabla, constantes, direcciones, generador, funciones);
            }

            Rule::condicion => {
                procesar_condicion(inner, tabla, constantes, direcciones, generador, funciones);
            }

            Rule::ciclo => {
                procesar_ciclo(inner, tabla, constantes, direcciones, generador, funciones);
            }

            Rule::retorno => {
                procesar_retorno(inner, tabla, constantes, direcciones, generador, funciones);
            }

            Rule::llamada => {
                procesar_llamada(inner, tabla, constantes, direcciones, generador, funciones);
            }

            _ => {}
        }
    }
}

fn registrar_variables(
    pair: Pair<Rule>,
    tabla: &mut TablaVariables,
    direcciones: &mut AdministradorDirecciones,
) {
    for p in pair.into_inner() {
        if p.as_rule() != Rule::lista_vars {
            continue;
        }

        let mut inner = p.into_inner();

        while let Some(id) = inner.next() {
            let tipo = inner.next().unwrap();
            let nombre = id.as_str().to_string();
            let tipo_var = tipo_desde_str(tipo.as_str());

            agregar_variable(tabla, nombre, tipo_var, direcciones);
        }
    }
}

fn registrar_firma_funcion(
    pair: Pair<Rule>,
    direcciones: &mut AdministradorDirecciones,
    inicio: usize,
) -> (String, FuncionMeta) {
    let mut tipo_retorno = Tipo::Void;
    let mut nombre_funcion = String::new();
    let mut parametros: Vec<ParamMeta> = Vec::new();

    for nodo in pair.into_inner() {
        match nodo.as_rule() {
            Rule::TIPO => {
                if tipo_retorno == Tipo::Void {
                    tipo_retorno = tipo_desde_str(nodo.as_str());
                }
            }

            Rule::id => {
                if nombre_funcion.is_empty() {
                    nombre_funcion = nodo.as_str().to_string();
                }
            }

            Rule::parametros => {
                let mut inner = nodo.into_inner();

                while let Some(param_id) = inner.next() {
                    let tipo_nodo = inner.next().unwrap();
                    let nombre_param = param_id.as_str().to_string();
                    let tipo_param = tipo_desde_str(tipo_nodo.as_str());
                    let direccion = direcciones.nueva_local(&tipo_param).to_string();

                    parametros.push(ParamMeta {
                        nombre: nombre_param,
                        tipo: tipo_param,
                        direccion,
                    });
                }
            }

            _ => {}
        }
    }

    if nombre_funcion.is_empty() {
        panic!("Función sin nombre");
    }

    (
        nombre_funcion,
        FuncionMeta {
            inicio,
            tipo_retorno,
            parametros,
        },
    )
}

fn procesar_funcion(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    funciones: &HashMap<String, FuncionMeta>,
) {
    let mut nombre_funcion = String::new();
    let mut cuerpo_funcion: Option<Pair<Rule>> = None;

    for nodo in pair.into_inner() {
        match nodo.as_rule() {
            Rule::id => {
                if nombre_funcion.is_empty() {
                    nombre_funcion = nodo.as_str().to_string();
                }
            }

            Rule::cuerpo => {
                cuerpo_funcion = Some(nodo);
            }

            _ => {}
        }
    }

    let meta = funciones
        .get(&nombre_funcion)
        .unwrap_or_else(|| panic!("Función '{}' no registrada", nombre_funcion))
        .clone();

    let mut tabla_local = tabla.clone();

    for param in &meta.parametros {
        tabla_local.insert(
            param.nombre.clone(),
            VariableInfo {
                nombre: param.nombre.clone(),
                tipo: param.tipo.clone(),
                direccion: Some(param.direccion.parse::<usize>().unwrap()),
            },
        );
    }

    println!(
        "Función '{}' inicia en cuádruplo {}",
        nombre_funcion, meta.inicio
    );

    if let Some(cuerpo) = cuerpo_funcion {
        procesar_cuerpo(
            cuerpo,
            &tabla_local,
            constantes,
            direcciones,
            generador,
            funciones,
        );
    }

    generador.generar_endfunc();
}

fn compilar(
    programa: &str,
) -> (
    TablaVariables,
    TablaConstantes,
    HashMap<String, FuncionMeta>,
    GeneradorCuadruplos,
) {
    let parse = PatitoParser::parse(Rule::program, programa).expect("Error de sintaxis");

    let mut tabla: TablaVariables = std::collections::HashMap::new();
    let mut constantes: TablaConstantes = std::collections::HashMap::new();
    let mut direcciones = AdministradorDirecciones::nuevo();
    let mut generador = GeneradorCuadruplos::nuevo();
    let mut funciones: HashMap<String, FuncionMeta> = HashMap::new();

    let program = parse.into_iter().next().unwrap();
    let nodos: Vec<Pair<Rule>> = program.into_inner().collect();

    let salto_main = generador.generar_goto(0);

    for nodo in nodos.clone() {
        if nodo.as_rule() == Rule::vars_block {
            registrar_variables(nodo, &mut tabla, &mut direcciones);
        }
    }

    // Registrar firmas de funciones antes de generar cuerpos.
    // Esto permite llamadas recursivas y llamadas entre funciones.
    for nodo in nodos.clone() {
        if nodo.as_rule() == Rule::funcs {
            for funcion in nodo.into_inner() {
                if funcion.as_rule() == Rule::func {
                    let inicio = generador.siguiente_cuadruplo();
                    let (nombre, meta) =
                        registrar_firma_funcion(funcion.clone(), &mut direcciones, inicio);
                    funciones.insert(nombre, meta);

                    // Reservar el espacio de los cuádruplos todavía no se hace aquí.
                    // El inicio se mantiene correcto porque los cuerpos se generan en el mismo orden.
                }
            }
        }
    }

    for nodo in nodos.clone() {
        if nodo.as_rule() == Rule::funcs {
            for funcion in nodo.into_inner() {
                if funcion.as_rule() == Rule::func {
                    procesar_funcion(
                        funcion,
                        &tabla,
                        &mut constantes,
                        &mut direcciones,
                        &mut generador,
                        &funciones,
                    );
                }
            }
        }
    }

    let inicio_main = generador.siguiente_cuadruplo();
    generador.rellenar_salto(salto_main, inicio_main);

    for nodo in nodos {
        if nodo.as_rule() == Rule::cuerpo {
            procesar_cuerpo(
                nodo,
                &tabla,
                &mut constantes,
                &mut direcciones,
                &mut generador,
                &funciones,
            );
        }
    }

    (tabla, constantes, funciones, generador)
}

fn main() {
    let programa = r#"
programa test;

vars:
resultado: entero;

entero fact(n: entero) {
    si (n <= 1) {
        return 1;
    } sino {
        return n * fact(n - 1);
    }
}

inicio

resultado = fact(5);
escribe(resultado);

fin
"#;

    let (tabla, constantes, funciones, generador) = compilar(programa);

    println!("\n=== VARIABLES ===");
    println!("{:#?}", tabla);

    println!("\n=== CONSTANTES ===");
    println!("{:#?}", constantes);

    println!("\n=== INICIOS FUNCIONES ===");
    println!("{:#?}", funciones);

    println!("\n=== CUADRUPLOS ===");

    for (i, c) in generador.cuadruplos.iter().enumerate() {
        println!("{} -> {:?}", i, c);
    }

    println!("\n=== EJECUCIÓN MÁQUINA VIRTUAL ===");

    let mut vm = MaquinaVirtual::nueva(generador.cuadruplos.clone(), &constantes);
    vm.ejecutar();
}
