# JS to Rust Compiler

Um compilador que converte uma linguagem similar ao JavaScript/TypeScript para código Rust. Este projeto é um exemplo de como implementar um compilador simples, demonstrando as etapas de análise léxica, análise sintática e geração de código.

## Funcionalidades

O compilador suporta as seguintes construções da linguagem:

### Declaração de Variáveis
```typescript
let nome: string = "João";
const idade: number = 25;
```

### Console Log
```typescript
console.log("Olá, mundo!");
console.log(nome);
```

### Estruturas Condicionais
```typescript
if (true) {
    let mensagem: string = "É verdadeiro";
    console.log(mensagem);
} else {
    console.log("Não é verdadeiro");
}
```

## Tokens Suportados

O compilador reconhece os seguintes tokens:

### Palavras-chave
- `console.log` - Para impressão no console
- `if` - Para condicionais
- `else` - Para o bloco alternativo de condicionais
- `let` - Para declaração de variáveis mutáveis
- `const` - Para declaração de constantes

### Tipos
- `string` - Para strings
- `number` - Para números inteiros
- `boolean` - Para valores booleanos

### Operadores e Pontuação
- `:` - Para anotação de tipo
- `;` - Para fim de declaração
- `(` - Parêntese aberto
- `)` - Parêntese fechado
- `{` - Chave aberta
- `}` - Chave fechada

### Literais
- Strings: `"texto"`
- Números: `123`
- Identificadores: `nomeDaVariavel`

## Estrutura do Projeto

```
src/
├── main.rs           # Ponto de entrada do programa
├── lexer/           # Analisador léxico
│   ├── mod.rs       # Implementação do lexer
│   └── tokens.rs    # Definição dos tokens
├── parser/          # Analisador sintático
│   └── mod.rs       # Implementação do parser
├── ast/             # Árvore sintática abstrata
│   └── mod.rs       # Definição dos nós da AST
└── generator/       # Gerador de código
    └── mod.rs       # Implementação do gerador de código Rust
```

## Como Usar

1. Clone o repositório:
```bash
git clone [url-do-repositorio]
cd js-compiler
```

2. Compile o projeto:
```bash
cargo build
```

3. Execute o compilador:
```bash
cargo run
```

4. O código Rust gerado estará em `dist/rust/`. Para executá-lo:
```bash
cd dist/rust
cargo run
```

## Exemplo de Saída

Para o código de entrada:
```typescript
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
```

O compilador gera o seguinte código Rust:
```rust
fn main() {
    let nome: String = String::from("João");
    println!("{:?}", nome);
    if true {
        let idade: i32 = 25;
        println!("{:?}", idade);
    } else {
        println!("{:?}", String::from("Não é verdadeiro"));
    }
    let mut contador: i32 = 0;
    while contador < 5 {
        println!("{:?}", format!("Contador é {}", contador));
        contador = contador + 1;
    }
    let nomes: Vec<String> = vec![
        String::from("Jose"),
        String::from("Juan"),
        String::from("Guilherme"),
        String::from("Vinicius"),
    ];
    println!("Array de nomes:{:?}", nomes);
}
```

## Limitações Atuais

- Suporta apenas tipos básicos (string, number, boolean e arrays)
- Não suporta operações aritméticas
- Não suporta funções
- Não suporta o loop "for"
- Não suporta objetos

## Próximos Passos

- [ ] Adicionar suporte a operações aritméticas
- [ ] Implementar suporte a funções
- [ ] Adicionar suporte a loops (for)
- [ ] Implementar suporte a objetos
- [ ] Adicionar verificação de tipos em tempo de compilação
- [ ] Melhorar o tratamento de erros

## Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou enviar pull requests.

## Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo LICENSE para detalhes. 
