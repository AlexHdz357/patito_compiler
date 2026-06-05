mod semantic;

use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use semantic::*;

#[derive(Parser)]
#[grammar = "patito.pest"]
struct PatitoParser;

fn obtener_variable(nombre: &str, tabla: &TablaVariables) -> (Tipo, String) {
    match buscar_variable_info(tabla, nombre) {
        Ok(info) => (info.tipo, info.direccion.unwrap().to_string()),

        Err(msg) => {
            panic!("{}", msg);
        }
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

fn procesar_condicion(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    constantes: &mut TablaConstantes,
    direcciones: &mut AdministradorDirecciones,
    generador: &mut GeneradorCuadruplos,
) {
    let mut expresion_condicion: Option<Pair<Rule>> = None;
    let mut cuerpos: Vec<Pair<Rule>> = Vec::new();

    for nodo in pair.into_inner() {
        match nodo.as_rule() {
            Rule::expresion => {
                expresion_condicion = Some(nodo);
            }

            Rule::cuerpo => {
                cuerpos.push(nodo);
            }

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

    procesar_cuerpo(cuerpos.remove(0), tabla, constantes, direcciones, generador);

    if !cuerpos.is_empty() {
        let salto_fin = generador.generar_goto(0);

        let inicio_sino = generador.siguiente_cuadruplo();

        generador.rellenar_salto(salto_falso, inicio_sino);

        procesar_cuerpo(cuerpos.remove(0), tabla, constantes, direcciones, generador);

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
) {
    let inicio_ciclo = generador.siguiente_cuadruplo();

    let mut expresion_condicion: Option<Pair<Rule>> = None;
    let mut cuerpo_ciclo: Option<Pair<Rule>> = None;

    for nodo in pair.into_inner() {
        match nodo.as_rule() {
            Rule::expresion => {
                expresion_condicion = Some(nodo);
            }

            Rule::cuerpo => {
                cuerpo_ciclo = Some(nodo);
            }

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
                procesar_condicion(inner, tabla, constantes, direcciones, generador);
            }

            Rule::ciclo => {
                procesar_ciclo(inner, tabla, constantes, direcciones, generador);
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

fn main() {
    let programa = r#"
programa test;

vars:
x: entero;
y: entero;

inicio

x = 0;
y = 3;

mientras (x < y) haz {
    escribe(x);
    x = x + 1;
}

si (x == y) {
    escribe(x);
} sino {
    escribe(y);
}

fin
"#;

    let parse = PatitoParser::parse(Rule::program, programa).expect("Error de sintaxis");

    let mut tabla: TablaVariables = std::collections::HashMap::new();

    let mut constantes: TablaConstantes = std::collections::HashMap::new();

    let mut direcciones = AdministradorDirecciones::nuevo();

    let mut generador = GeneradorCuadruplos::nuevo();

    let program = parse.into_iter().next().unwrap();

    for nodo in program.into_inner() {
        match nodo.as_rule() {
            Rule::vars_block => {
                registrar_variables(nodo, &mut tabla, &mut direcciones);
            }

            Rule::cuerpo => {
                procesar_cuerpo(
                    nodo,
                    &tabla,
                    &mut constantes,
                    &mut direcciones,
                    &mut generador,
                );
            }

            _ => {}
        }
    }

    println!("\n=== VARIABLES ===");
    println!("{:#?}", tabla);

    println!("\n=== CONSTANTES ===");
    println!("{:#?}", constantes);

    println!("\n=== CUADRUPLOS ===");

    for (i, c) in generador.cuadruplos.iter().enumerate() {
        println!("{} -> {:?}", i, c);
    }
}
