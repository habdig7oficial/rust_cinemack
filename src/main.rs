mod basic;
use crate::basic::*;
use chrono::prelude::*;
struct Filme {
  nome: String,
  capacidade: u32,
  sessao: Vec<Sessao>,
  avaliacao: Vec<u8>
}
struct Sessao {
  hora: DateTime<Local>,
  lotacao: u32
}


fn print_filmes(filmes: &Vec<Filme>) -> u8{
  for i in 0..filmes.len(){
    println!("{}) {}", i + 1, &filmes[i].nome);
  }
  let opt_filme: u8 = rm_endl(input("\nEscolha um filme:"))
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

fn verifica_sessao(opt_filme: u8, filmes: Vec<Filme>) -> u8 {
  println!("\nSessões: ");
  for i in 0..filmes[usize::from(opt_filme) - 1].sessao.len() {
    println!("{}) {}", i + 1, filmes[usize::from(opt_filme) - 1].sessao[i].hora);
  }
  
  let opt_sessao: u8 = rm_endl(input("\nEscolha entre uma as sessões disponíveis: ")).parse().unwrap();

  if usize::from(opt_sessao) > filmes[usize::from(opt_filme) - 1].sessao.len() || opt_sessao == 0 {
    println!("Sessão {opt_sessao} inválida");
    return verifica_sessao(opt_filme, filmes);
  }
  else {
    return opt_sessao;
  }

}

fn main() {
    let filmes: Vec<Filme> = vec![
     Filme{
        nome: String::from("Elisangela.py"),
        capacidade: 50,
        sessao: vec![
          Sessao{
            hora: Local::now(),
            lotacao: 0
          }, 
          Sessao{
            hora: Local::now(),
            lotacao: 0
          }],  
        avaliacao: vec![],
     },
     Filme{
        nome: String::from("JamilINO"),
        capacidade: 40,
        sessao: vec![
          Sessao{
            hora: Local::now(),
            lotacao: 0
          }, 
          Sessao{
            hora: Local::now(),
            lotacao: 0
          }],  
        avaliacao: vec![],
     },
     Filme{
        nome: String::from("Alcides no país dos Fluxogramas"),
        capacidade: 30,
        sessao: vec![
          Sessao{
            hora: Local::now(),
            lotacao: 0
          }, 
          Sessao{
            hora: Local::now(),
            lotacao: 0
          }],  
        avaliacao: vec![],
     },
     ];
     println!("-- CINEMACK --\n");
     let opt = print_filmes(&filmes);
     verifica_sessao(opt, filmes);
}
