mod ast;
mod generator;
mod lexer;
mod parser;


use generator::CodeGenerator;
use lexer::Lexer;
use parser::Parser;

fn main() -> Result<(), std::io::Error> {
    let source = r#"
        let nome: string = "João";
        console.log(nome);

        if (true) {
            let idade: number = 25;
            console.log(idade);
        } else {
            console.log("Não é verdadeiro");
        }

        let contador: number = 0;
        while (contador < 5) {
            console.log("Contador é " + contador);
            contador = contador + 1;
        }

        const nomes: string[] = ["Jose", "Juan", "Guilherme", "Vinicius"];
        console.log("Array de nomes:", nomes);

        console.log("Operação:  ( 1 + 2 ) * 3");
        let resultado1: number = (1 + 2) * 3;
        console.log("Resultado:", resultado1);

        console.log("Operação: 10 / ( 2 + 3 )");
        let resultado2: number = 10 / (2 + 3);
        console.log("Resultado:", resultado2);

        console.log("Operação: ( 2 + 3 ) * ( 4 - 1 )");
        let resultado3: number = (2 + 3) * (4 - 1);
        console.log("Resultado:", resultado3);
    "#;

    println!("{}", source);
    let lexer = Lexer::new(source);
    let tokens = lexer.get_tokens();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    // println!("Análise sintática concluída. Gerando código Rust...");
    // for stmt in &statements {
    //     println!("{:?}", stmt);
    // }

    let generator = CodeGenerator::new("dist/rust");
    generator.generate(statements)?;

    println!("Código Rust gerado com sucesso em dist/rust/src/main.rs");
    println!("\nPara executar o código gerado:");
    println!("1. cd dist/rust");
    println!("2. cargo run");
    Ok(())
}
