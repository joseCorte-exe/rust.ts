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
    let nomes: Vec<String> = vec![String::from("Jose"),String::from("Juan"),String::from("Guilherme"),String::from("Vinicius")];
    println!("Array de nomes:{:?}", nomes);
}
