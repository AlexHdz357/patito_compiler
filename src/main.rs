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
struct FuncionMeta {
    inicio: usize,
    parametros: Vec<String>,
}

fn obtener_variable(nombre: &str, tabla: &TablaVariables) -> (Tipo, String) {
    match buscar_variable_info(tabla, nombre) {
        Ok(info) => (info.tipo, info.direccion.unwrap().to_string()),
        Err(msg) => panic!("{}", msg),
    }
}

fn procesar_factor(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
) {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::id => {
            let nombre = inner.as_str().to_string();
            let (tipo, direccion) = obtener_variable(&nombre, tabla);
            generador.push_operando(direccion, tipo);
        }

        Rule::cte => {
            let valor = inner.as_str().to_string();

            let tipo = if valor.contains(".") {
                Tipo::Flotante
            } else {
                Tipo::Entero
            };

            let direccion = registrar_constante(constantes, valor, tipo.clone(), direcciones);
            generador.push_operando(direccion.to_string(), tipo);
        }

        Rule::expresion => {
            procesar_expresion(inner, tabla, constantes, direcciones, generador);
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
) {
    let mut inner = pair.into_inner();
    let primero = inner.next().unwrap();

    procesar_factor(primero, tabla, constantes, direcciones, generador);

    while let Some(op) = inner.next() {
        let operador = op.as_str().to_string();
        let siguiente = inner.next().unwrap();

        procesar_factor(siguiente, tabla, constantes, direcciones, generador);

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
) {
    let mut inner = pair.into_inner();
    let primero = inner.next().unwrap();

    procesar_termino(primero, tabla, constantes, direcciones, generador, cubo);

    while let Some(op) = inner.next() {
        let operador = op.as_str().to_string();
        let siguiente = inner.next().unwrap();

        procesar_termino(siguiente, tabla, constantes, direcciones, generador, cubo);

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
) {
    let cubo = CuboSemantico::nuevo();

    let mut inner = pair.into_inner();
    let izquierda = inner.next().unwrap();

    procesar_exp(izquierda, tabla, constantes, direcciones, generador, &cubo);

    if let Some(op_rel) = inner.next() {
        let operador = op_rel.as_str().to_string();
        let derecha = inner.next().unwrap();

        procesar_exp(derecha, tabla, constantes, direcciones, generador, &cubo);

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
) {
    let cubo = CuboSemantico::nuevo();

    let mut inner = pair.into_inner();

    let variable = inner.next().unwrap().as_str().to_string();
    let (tipo_variable, direccion_variable) = obtener_variable(&variable, tabla);

    let expresion = inner.next().unwrap();

    procesar_expresion(expresion, tabla, constantes, direcciones, generador);

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
) {
    for nodo in pair.into_inner() {
        if nodo.as_rule() == Rule::expresion {
            procesar_expresion(nodo, tabla, constantes, direcciones, generador);
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
) {
    for nodo in pair.into_inner() {
        if nodo.as_rule() == Rule::expresion {
            procesar_expresion(nodo, tabla, constantes, direcciones, generador);
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
    inicios_funciones: &HashMap<String, FuncionMeta>,
) {
    let mut inner = pair.into_inner();

    let nombre_funcion = inner.next().unwrap().as_str().to_string();

    let meta = inicios_funciones
        .get(&nombre_funcion)
        .unwrap_or_else(|| panic!("Función '{}' no declarada", nombre_funcion))
        .clone();

    generador.generar_era(nombre_funcion.clone());

    let mut contador_param = 0;

    for nodo in inner {
        if nodo.as_rule() == Rule::argumentos {
            for arg in nodo.into_inner() {
                if arg.as_rule() == Rule::expresion {
                    procesar_expresion(arg, tabla, constantes, direcciones, generador);

                    let destino_param = meta
                        .parametros
                        .get(contador_param)
                        .unwrap_or_else(|| {
                            panic!("Demasiados argumentos para función '{}'", nombre_funcion)
                        })
                        .clone();

                    generador.generar_param(destino_param);

                    contador_param += 1;
                }
            }
        }
    }

    generador.generar_gosub(nombre_funcion, meta.inicio);
}
fn procesar_condicion(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    inicios_funciones: &HashMap<String, FuncionMeta>,
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
    );

    let salto_falso = generador.generar_gotof().unwrap();

    procesar_cuerpo(
        cuerpos.remove(0),
        tabla,
        constantes,
        direcciones,
        generador,
        inicios_funciones,
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
            inicios_funciones,
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
    inicios_funciones: &HashMap<String, FuncionMeta>,
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
    );

    let salto_falso = generador.generar_gotof().unwrap();

    procesar_cuerpo(
        cuerpo_ciclo.unwrap(),
        tabla,
        constantes,
        direcciones,
        generador,
        inicios_funciones,
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
    inicios_funciones: &HashMap<String, FuncionMeta>,
) {
    for estatuto in pair.into_inner() {
        let inner = estatuto.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::asigna => {
                procesar_asignacion(inner, tabla, constantes, direcciones, generador);
            }

            Rule::imprime => {
                procesar_print(inner, tabla, constantes, direcciones, generador);
            }

            Rule::condicion => {
                procesar_condicion(
                    inner,
                    tabla,
                    constantes,
                    direcciones,
                    generador,
                    inicios_funciones,
                );
            }

            Rule::ciclo => {
                procesar_ciclo(
                    inner,
                    tabla,
                    constantes,
                    direcciones,
                    generador,
                    inicios_funciones,
                );
            }

            Rule::retorno => {
                procesar_retorno(inner, tabla, constantes, direcciones, generador);
            }

            Rule::llamada => {
                procesar_llamada(
                    inner,
                    tabla,
                    constantes,
                    direcciones,
                    generador,
                    inicios_funciones,
                );
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

            let tipo_var = match tipo.as_str() {
                "entero" => Tipo::Entero,
                "flotante" => Tipo::Flotante,
                _ => Tipo::Error,
            };

            agregar_variable(tabla, nombre, tipo_var, direcciones);
        }
    }
}

fn obtener_nombre_funcion(pair: Pair<Rule>) -> String {
    for nodo in pair.into_inner() {
        if nodo.as_rule() == Rule::id {
            return nodo.as_str().to_string();
        }
    }

    panic!("Función sin nombre");
}

fn procesar_funcion(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
    inicios_funciones: &mut HashMap<String, FuncionMeta>,
) {
    let inicio_funcion = generador.siguiente_cuadruplo();

    let mut nombre_funcion = String::new();

    let mut cuerpo_funcion: Option<Pair<Rule>> = None;

    let mut parametros_funcion: Vec<String> = Vec::new();

    let mut tabla_local = tabla.clone();

    for nodo in pair.into_inner() {
        match nodo.as_rule() {
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

                    let tipo_param = match tipo_nodo.as_str() {
                        "entero" => Tipo::Entero,
                        "flotante" => Tipo::Flotante,
                        _ => Tipo::Error,
                    };

                    let direccion = direcciones.nueva_local(&tipo_param);

                    tabla_local.insert(
                        nombre_param.clone(),
                        VariableInfo {
                            nombre: nombre_param,
                            tipo: tipo_param,
                            direccion: Some(direccion),
                        },
                    );

                    parametros_funcion.push(direccion.to_string());
                }
            }

            Rule::cuerpo => {
                cuerpo_funcion = Some(nodo);
            }

            _ => {}
        }
    }

    inicios_funciones.insert(
        nombre_funcion.clone(),
        FuncionMeta {
            inicio: inicio_funcion,
            parametros: parametros_funcion,
        },
    );

    println!(
        "Función '{}' inicia en cuádruplo {}",
        nombre_funcion, inicio_funcion
    );

    if let Some(cuerpo) = cuerpo_funcion {
        procesar_cuerpo(
            cuerpo,
            &tabla_local,
            constantes,
            direcciones,
            generador,
            inicios_funciones,
        );
    }

    generador.generar_endfunc();
}
fn main() {
    let programa = r#"
programa test;

vars:
x: entero;

entero cuenta(n: entero) {
    mientras (n > 0) haz {
        escribe(n);
        n = n - 1;
    }
    return n;
}

inicio

x = 3;

cuenta(x);

fin
"#;

    let parse = PatitoParser::parse(Rule::program, programa).expect("Error de sintaxis");

    let mut tabla: TablaVariables = std::collections::HashMap::new();
    let mut constantes: TablaConstantes = std::collections::HashMap::new();
    let mut direcciones = AdministradorDirecciones::nuevo();
    let mut generador = GeneradorCuadruplos::nuevo();

    let mut inicios_funciones: HashMap<String, FuncionMeta> = HashMap::new();

    let program = parse.into_iter().next().unwrap();
    let nodos: Vec<Pair<Rule>> = program.into_inner().collect();

    let salto_main = generador.generar_goto(0);

    // Primera pasada: registrar variables globales
    for nodo in nodos.clone() {
        if nodo.as_rule() == Rule::vars_block {
            registrar_variables(nodo, &mut tabla, &mut direcciones);
        }
    }

    // Segunda pasada: registrar inicio de funciones
    for nodo in nodos.clone() {
        if nodo.as_rule() == Rule::funcs {
            for funcion in nodo.into_inner() {
                if funcion.as_rule() == Rule::func {
                    let inicio = generador.siguiente_cuadruplo();
                    let nombre_funcion = obtener_nombre_funcion(funcion.clone());

                    inicios_funciones.insert(
                        nombre_funcion,
                        FuncionMeta {
                            inicio,
                            parametros: Vec::new(),
                        },
                    );
                }
            }
        }
    }

    // Tercera pasada: generar funciones
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
                        &mut inicios_funciones,
                    );
                }
            }
        }
    }

    // Parchear salto inicial hacia main
    let inicio_main = generador.siguiente_cuadruplo();

    generador.rellenar_salto(salto_main, inicio_main);

    // Cuarta pasada: generar main
    for nodo in nodos {
        if nodo.as_rule() == Rule::cuerpo {
            procesar_cuerpo(
                nodo,
                &tabla,
                &mut constantes,
                &mut direcciones,
                &mut generador,
                &inicios_funciones,
            );
        }
    }

    println!("\n=== VARIABLES ===");
    println!("{:#?}", tabla);

    println!("\n=== CONSTANTES ===");
    println!("{:#?}", constantes);

    println!("\n=== INICIOS FUNCIONES ===");
    println!("{:#?}", inicios_funciones);

    println!("\n=== CUADRUPLOS ===");

    for (i, c) in generador.cuadruplos.iter().enumerate() {
        println!("{} -> {:?}", i, c);
    }

    println!("\n=== EJECUCIÓN MÁQUINA VIRTUAL ===");

    let mut vm = MaquinaVirtual::nueva(generador.cuadruplos.clone(), &constantes);

    vm.ejecutar();
}
