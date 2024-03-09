use::std::io;
#[derive(Default)]
struct Elemento{
    nome:String, simbolo:String, serie_quimica:String, estado_materia:String, classe_magnetica:String,
    densidade:f32, dureza:f32, massa_atomica:f32, ponto_fusao:f32, ponto_ebulicao:f32, eletronegatividade:f32
}
fn cadastra_elemento() -> Elemento{
    println!(" | Cadastrando novo elemento");
    println!(" > Insira o nome: ");
    let mut nome_elemento:String = Default::default();
    io::stdin().read_line(&mut nome_elemento).expect("Erro ao ler o nome do elemento");
    let mut elementos:Elemento = Default::default();
    elementos.nome = nome_elemento;
    lista_elementos(&elementos);
    return elementos
}
fn lista_elementos(elementos:&Elemento){
    println!("{}", elementos.nome);
}
fn main() {
    cadastra_elemento();
}