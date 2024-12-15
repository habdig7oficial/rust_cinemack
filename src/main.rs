mod basic;
use crate::basic::*;
struct Filme {
  nome: String,
  capacidade: u32,
  sessao: Vec<u32>,
  avaliacao: Vec<u8>
}
fn print_filmes(filmes: Vec<Filme>) -> u8{
  for i in 0..filmes.len(){
    println!("{}) {}", i + 1, &filmes[i].nome);
  }
  let opt_filme: u8 = rm_endl(input("Escolha um filme:"))
      .parse()
      .unwrap();
  if usize::from(opt_filme) > filmes.len() || opt_filme == 0 {
    println!("Número de filme {opt_filme} inválido\n");
    return print_filmes(filmes);
  }
  else {
  println!("Filme {} selecionado!", &filmes[usize::from(opt_filme) - 1].nome);
  return opt_filme;
  }
}

fn main() {
    let filmes: Vec<Filme> = vec![
     Filme{
        nome: String::from("Elisangela.py"),
        capacidade: 50,
        sessao: vec![0, 0],  
        avaliacao: vec![],
     },
     Filme{
        nome: String::from("JamilINO"),
        capacidade: 40,
        sessao: vec![0, 0],  
        avaliacao: vec![],
     },
     Filme{
        nome: String::from("Alcides no país dos Fluxogramas"),
        capacidade: 30,
        sessao: vec![0, 0],  
        avaliacao: vec![],
     },
     ];
     print_filmes(filmes);
}
