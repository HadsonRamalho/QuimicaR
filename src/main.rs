use::std::io;
use std::io::Read;

#[derive(Default)]
struct Elemento{
    nome:String, simbolo:String, serie_quimica:String, estado_materia:String, classe_magnetica:String,
    densidade:f32, dureza:f32, massa_atomica:f32, ponto_fusao:f32, ponto_ebulicao:f32, eletronegatividade:f32
}

const TM:u32 = 10;

fn cadastra_elemento() -> Elemento {
    let mut nome:String = Default::default();
    let mut simbolo:String = Default::default();
    let mut serie_quimica:String = Default::default();

    println!(" | Cadastrando novo elemento");
    println!(" > Insira o nome: ");
    io::stdin().read_line(&mut nome).expect(" > Erro ao ler o nome do elemento");
    println!(" > Insira o símbolo do elemento: ");
    io::stdin().read_line(&mut simbolo).expect(" > Erro ao ler o símbolo do elemento");
    println!(" > Insira a série química do elemento: ");
    io::stdin().read_line(&mut serie_quimica).expect(" > Erro ao ler a série química do elemento");

    let mut elemento:Elemento = Default::default();
    elemento.nome = nome;
    elemento.simbolo = simbolo;
    elemento.serie_quimica = serie_quimica;

    println!("Elemento cadastrado!");
    exibe_elemento(&elemento);

    return elemento
}

fn inicializa_elementos(vet_elementos:&mut [Elemento; TM as usize]){
    vet_elementos[0].nome = "Hidrogênio".to_string();
    vet_elementos[0].simbolo = "H".to_string();

    vet_elementos[1].nome = "Hélio".to_string();
    vet_elementos[1].simbolo = "He".to_string();

    vet_elementos[2].nome = "Lítio".to_string();
    vet_elementos[2].simbolo = "Li".to_string();

    vet_elementos[3].nome = "Berílio".to_string();
    vet_elementos[3].simbolo = "Be".to_string();

    vet_elementos[4].nome = "Boro".to_string();
    vet_elementos[4].simbolo = "B".to_string();

    vet_elementos[5].nome = "Carbono".to_string();
    vet_elementos[5].simbolo = "C".to_string();

    vet_elementos[6].nome = "Nitrogênio".to_string();
    vet_elementos[6].simbolo = "N".to_string();

    vet_elementos[7].nome = "Oxigênio".to_string();
    vet_elementos[7].simbolo = "O".to_string();

    vet_elementos[8].nome = "Flúor".to_string();
    vet_elementos[8].simbolo = "F".to_string();

    vet_elementos[9].nome = "Neônio".to_string();
    vet_elementos[9].simbolo = "Ne".to_string();
}

fn exibe_elemento(elementos:&Elemento){
    println!("Nome: {}", elementos.nome);
    println!("Símbolo: {}", elementos.simbolo);
    println!("Série química: {}", elementos.serie_quimica);
}

fn elem_erro() -> Elemento{
    let er:Elemento = Default::default();
    return er
}

fn stoi(y:String) -> i32{
    let y: i32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => return 0
    };
    return y
}

fn lista_elementos(elementos:[Elemento; TM as usize]) -> Elemento{
    for i in elementos{
        println!("Nome: {}", i.nome);
        println!("Símbolo: {}", i.simbolo);
        println!("Série química: {}", i.serie_quimica);
        println!("\n");
    }
    let el_retorno:Elemento = Default::default();
    return el_retorno
}

fn pesquisar_elementos(elementos:[Elemento; TM as usize]) -> Elemento{
    println!(" | Deseja pesquisar por qual campo?\n\
     1 | Nome\n\
     2 | Símbolo\n");

    let mut opc:String = Default::default();
    io::stdin().read_line(& mut opc).expect(" > Erro ao ler a opção");
    let mut opcao = stoi(opc);
    let mut el_retorno:Elemento = Default::default();
    match (opcao){
        1 => {
            println!("Digite o nome do elemento: ");
            let mut nome_elem:String = Default::default();
            io::stdin().read_line(&mut nome_elem).expect(" > Erro ao ler o nome do elemento.");
            let mut encontrado = false;
            for i in elementos.iter() {
                if (i.nome.eq_ignore_ascii_case(nome_elem.trim())) {
                    println!(" | ELEMENTO ENCONTRADO! \n");
                    exibe_elemento(&i);
                    encontrado = true;
                    break;
                }
            }
            if (!encontrado){
                println!(" > Elemento não encontrado!");
            }
        }
        2 =>{
            println!("Digite o símbolo do elemento: ");
            let mut simb_elem:String = Default::default();
            io::stdin().read_line(&mut simb_elem).expect(" > Erro ao ler o símbolo do elemento");
            let mut encontrado = false;
            for i in elementos.iter(){
                if(i.simbolo.eq_ignore_ascii_case((simb_elem.trim()))){
                    println!(" | ELEMENTO ENCONTRADO \n");
                    exibe_elemento(&i);
                    encontrado = true;
                    break;
                }
            }
        }
        _ => {println!(" > Erro na pesquisa.");}
    }
    return el_retorno
}

fn menu(){
    let mut vet_elementos:[Elemento; TM as usize] = Default::default();
    inicializa_elementos(&mut vet_elementos);
    println!("\tMENU PRINCIPAL\n\
    1 | Listar todos os elementos\n\
    2 | Pesquisar um elemento\n\
    3 |[DEV] Editar um elemento\n\
     > Selecione uma opção: ");
    let mut opc:String = Default::default();
    io::stdin().read_line(&mut opc).expect(" > Erro ao ler a opção");
    let mut opcao = stoi(opc);
    match(opcao){
        1 => {
            println!(" | LISTANDO ELEMENTOS\n");
            lista_elementos(vet_elementos)
        },
        2 => {
            pesquisar_elementos(vet_elementos)
        },
        _ => {elem_erro()}
    };

}

fn main() {
    menu();
}