use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "patito.pest"]
struct PatitoParser;

fn main() {
    let tests = vec![
        // =========================
        // CASOS VÁLIDOS
        // =========================
        ("Válido 1: Programa mínimo", 
        r#"programa a;
inicio
fin"#, true),

        ("Válido 2: Declaración de variables", 
        r#"programa a;
vars:
x: entero;
inicio
x = 10;
fin"#, true),

        ("Válido 3: Expresión aritmética", 
        r#"programa a;
vars:
x: entero;
inicio
x = 5 + 3;
fin"#, true),

        ("Válido 4: Impresión", 
        r#"programa a;
vars:
x: entero;
inicio
x = 5;
escribe(x);
fin"#, true),

        ("Válido 5: Expresión compleja", 
        r#"programa a;
vars:
x: entero;
inicio
x = (5 + 3) * 2;
fin"#, true),

        // =========================
        // CASOS INVÁLIDOS
        // =========================
        ("Error 1: Falta identificador", 
        "programa ;", false),

        ("Error 2: Falta palabra clave programa", 
        "x = 5;", false),

        ("Error 3: Expresión incompleta", 
        r#"programa a;
inicio
x = ;
fin"#, false),

        ("Error 4: Falta punto y coma", 
        r#"programa a;
inicio
x = 5
fin"#, false),

        ("Error 5: Paréntesis no balanceados", 
        r#"programa a;
inicio
x = (5 + 3;
fin"#, false),

        // =========================
        // CASOS LÍMITE
        // =========================
        ("Edge 1: Número grande", 
        r#"programa a;
vars:
x: entero;
inicio
x = 999999999;
fin"#, true),

        ("Edge 2: Número flotante", 
        r#"programa a;
vars:
x: flotante;
inicio
x = 1.2345;
fin"#, true),

        ("Edge 3: Expresión larga", 
        r#"programa a;
vars:
x: entero;
inicio
x = 1 + 2 + 3 + 4 + 5;
fin"#, true),
    ];

    let mut passed = 0;
    for (name, input, should_pass) in &tests {
        println!("\n--- {} ---", name);

        let result = PatitoParser::parse(Rule::program, input);

        match (result, *should_pass) {
            (Ok(_), true) => {
                println!("Correcto (aceptado)");
                passed += 1;
            }
            (Err(e), false) => {
                println!("Correcto (error detectado)");
                println!("   {}", e);
                passed += 1;
            }
            (Ok(_), false) => {
                println!("Fallo: se esperaba error pero fue válido");
            }
            (Err(e), true) => {
                println!("Fallo: se esperaba válido pero hubo error");
                println!("   {}", e);
            }
        }
    }

    println!("\n=========================");
    println!("Resumen: {}/{} casos correctos", passed, tests.len());
}