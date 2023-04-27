
/**!
 Limpa os downloads antigos que podem
 "vencer" rapidamente, ou seja, não será
 utilizado mais.
*/

// meus módulos:
mod item_de_exclusao;
use item_de_exclusao::FilaExclusao;
mod janela_grafica;
use janela_grafica::{Grafico};
mod letreiro;
mod links;
mod tempo_tools;
#[allow(unused)]
mod configuracao;

// biblioteca padrão do Rust:
use std::env::args;


// bloco que executa programa.
fn main() {
   // instânciando limpador ...
   let mut limpeza = FilaExclusao::gera();

   if args().any(|s| s == "ncurses")
      // neste caso inicializa o "ncurses"...
      { Grafico::visualiza(&mut limpeza); }
   else 
      /* padrão, apenas mostra lista de exclusão 
       * de hoje, e deleta já expirados. */
      { limpeza.visualiza(); }

   if cfg!(unix) {
      // criando executáveis se não houver.
      links::linka_executaveis("LD");
   }
}

