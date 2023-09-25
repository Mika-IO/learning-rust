use std::io;

pub fn dinamic() {
    loop {
        println!("Digite uma expressão matemática (ou 'sair' para encerrar):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
        
        // Remova espaços em branco e caracteres de nova linha da entrada.
        let input = input.trim();

        if input == "sair" {
            break;
        }

        let resultado = avaliar_expressao(input);
        
        match resultado {
            Ok(result) => {
                println!("Resultado: {}", result);
            }
            Err(err) => {
                println!("Erro: {}", err);
            }
        }
    }
}

fn avaliar_expressao(expr: &str) -> Result<i32, String> {
    // Divide a expressão em tokens separados por espaços.
    let tokens: Vec<&str> = expr.split_whitespace().collect();

    // Verifica se há pelo menos 3 tokens (número, operador, número).
    if tokens.len() != 3 {
        return Err("Expressão inválida".to_string());
    }

    let operando1 = tokens[0].parse::<i32>();
    let operando2 = tokens[2].parse::<i32>();

    match (operando1, operando2) {
        (Ok(op1), Ok(op2)) => {
            match tokens[1] {
                "+" => Ok(op1 + op2),
                "-" => Ok(op1 - op2),
                "*" => Ok(op1 * op2),
                "/" => {
                    if op2 == 0 {
                        Err("Divisão por zero".to_string())
                    } else {
                        Ok(op1 / op2)
                    }
                }
                _ => Err("Operador inválido".to_string()),
            }
        }
        _ => Err("Operando inválido".to_string()),
    }
}
