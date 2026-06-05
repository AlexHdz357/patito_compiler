mod semantic;

use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use semantic::*;

#[derive(Parser)]
#[grammar = "patito.pest"]
struct PatitoParser;

fn obtener_tipo_variable(
    nombre: &str,
    tabla: &TablaVariables,
) -> Tipo {

    match buscar_variable(
        tabla,
        nombre,
    ) {
        Ok(tipo) => tipo,

        Err(msg) => {
            panic!("{}", msg);
        }
    }
}

fn procesar_factor(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    gen: &mut GeneradorCuadruplos,
) {

    let inner =
        pair.into_inner()
            .next()
            .unwrap();

    match inner.as_rule() {

        Rule::id => {

            let nombre =
                inner.as_str()
                    .to_string();

            let tipo =
                obtener_tipo_variable(
                    &nombre,
                    tabla,
                );

            gen.push_operando(
                nombre,
                tipo,
            );
        }

        Rule::cte => {

            let valor =
                inner.as_str()
                    .to_string();

            if valor.contains(".")
            {
                gen.push_operando(
                    valor,
                    Tipo::Flotante,
                );
            }
            else {

                gen.push_operando(
                    valor,
                    Tipo::Entero,
                );
            }
        }

        Rule::expresion => {

            procesar_expresion(
                inner,
                tabla,
                gen,
            );
        }

        _ => {}
    }
}

fn procesar_termino(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    gen: &mut GeneradorCuadruplos,
    cubo: &CuboSemantico,
) {

    let mut inner =
        pair.into_inner();

    let primero =
        inner.next().unwrap();

    procesar_factor(
        primero,
        tabla,
        gen,
    );


    while let Some(op) = inner.next() {

        let operador =
            op.as_str()
                .to_string();

        let siguiente =
            inner.next()
                .unwrap();

        procesar_factor(
            siguiente,
            tabla,
            gen,
        );

        gen.push_operador(
            operador,
        );

        gen.generar_operacion(
            cubo,
        ).unwrap();
    }
}

fn procesar_exp(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    gen: &mut GeneradorCuadruplos,
    cubo: &CuboSemantico,
) {

    let mut inner =
        pair.into_inner();

    let primero =
        inner.next().unwrap();

    procesar_termino(
        primero,
        tabla,
        gen,
        cubo,
    );

    while let Some(op) =
        inner.next()
    {

        let operador =
            op.as_str()
                .to_string();

        let siguiente =
            inner.next()
                .unwrap();

        procesar_termino(
            siguiente,
            tabla,
            gen,
            cubo,
        );

        gen.push_operador(
            operador,
        );

        gen.generar_operacion(
            cubo,
        ).unwrap();
    }
}

fn procesar_expresion(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    gen: &mut GeneradorCuadruplos,
) {

    let cubo =
        CuboSemantico::nuevo();

    let mut inner =
        pair.into_inner();

    let izquierda =
        inner.next()
            .unwrap();

    procesar_exp(
        izquierda,
        tabla,
        gen,
        &cubo,
    );

    if let Some(op_rel) =
        inner.next()
    {

        let operador =
            op_rel.as_str()
                .to_string();

        let derecha =
            inner.next()
                .unwrap();

        procesar_exp(
            derecha,
            tabla,
            gen,
            &cubo,
        );

        gen.push_operador(
            operador,
        );

        gen.generar_operacion(
            &cubo,
        ).unwrap();
    }
}

fn procesar_asignacion(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    gen: &mut GeneradorCuadruplos,
) {

    let cubo =
        CuboSemantico::nuevo();

    let mut inner =
        pair.into_inner();

    let variable =
        inner.next()
            .unwrap()
            .as_str()
            .to_string();

    let tipo_variable =
        obtener_tipo_variable(
            &variable,
            tabla,
        );

    let expresion =
        inner.next()
            .unwrap();

    procesar_expresion(
        expresion,
        tabla,
        gen,
    );

    gen.generar_asignacion(
        variable,
        tipo_variable,
        &cubo,
    ).unwrap();
}
fn procesar_print(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    gen: &mut GeneradorCuadruplos,
) {

    let expresion =
        pair.into_inner()
            .next()
            .unwrap();

    procesar_expresion(
        expresion,
        tabla,
        gen,
    );

    gen.generar_print();
}

fn procesar_cuerpo(
    pair: Pair<Rule>,
    tabla: &TablaVariables,
    gen: &mut GeneradorCuadruplos,
) {

    for estatuto
        in pair.into_inner()
    {

        let inner =
            estatuto
                .into_inner()
                .next()
                .unwrap();

        match inner.as_rule() {

            Rule::asigna => {

                procesar_asignacion(
                    inner,
                    tabla,
                    gen,
                );
            }

            Rule::imprime => {

                procesar_print(
                    inner,
                    tabla,
                    gen,
                );
            }

            _ => {}
        }
    }
}

fn registrar_variables(
    pair: Pair<Rule>,
    tabla:
        &mut TablaVariables,
) {

    for p in pair.into_inner()
    {

        if p.as_rule()
            != Rule::lista_vars
        {
            continue;
        }

        let mut inner =
            p.into_inner();

        while let Some(id) =
            inner.next()
        {

            let tipo =
                inner.next()
                    .unwrap();

            let nombre =
                id.as_str()
                    .to_string();

            let tipo_var =
                match tipo.as_str()
            {
                "entero" =>
                    Tipo::Entero,

                "flotante" =>
                    Tipo::Flotante,

                _ =>
                    Tipo::Error,
            };

            agregar_variable(
                tabla,
                nombre,
                tipo_var,
            );
        }
    }
}

fn main() {

    let programa =
r#"
programa test;

vars:
x: entero;
y: entero;
z: flotante;

inicio

x = 5 + 3;

y = x * 2;

escribe(y);

fin
"#;

    let parse =
        PatitoParser::parse(
            Rule::program,
            programa,
        )
        .expect(
            "Error de sintaxis"
        );

    let mut tabla:
        TablaVariables =
        std::collections::HashMap::new();

    let mut gen =
        GeneradorCuadruplos::nuevo();

    let program =
        parse.into_iter()
            .next()
            .unwrap();

    for nodo
        in program.into_inner()
    {

        match nodo.as_rule()
        {

            Rule::vars_block => {

                registrar_variables(
                    nodo,
                    &mut tabla,
                );
            }

            Rule::cuerpo => {

                procesar_cuerpo(
                    nodo,
                    &tabla,
                    &mut gen,
                );
            }

            _ => {}
        }
    }

    println!(
        "\n=== VARIABLES ==="
    );

    println!(
        "{:#?}",
        tabla
    );

    println!(
        "\n=== CUADRUPLOS ==="
    );

    for (i, c)
        in gen.cuadruplos
            .iter()
            .enumerate()
    {

        println!(
            "{} -> {:?}",
            i,
            c
        );
    }
}