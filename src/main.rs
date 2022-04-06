
/**!
 Limpa os downloads antigos que podem
 "vencer" rapidamente, ou seja, não será
 utilizado mais.
*/

// meus módulos:
mod item_de_exclusao;
use item_de_exclusao::FilaExclusao;
mod janela_grafica;
use janela_grafica::Grafico;

// biblioteca externa:
extern crate utilitarios;
use utilitarios::{legivel, aleatorio::sortear};

// biblioteca padrão do Rust:
use std::thread;
use std::time::{Duration, Instant};
use std::env::args;


// bloco que executa programa.
fn main() {
   // instânciando limpador ...
   let mut limpeza = FilaExclusao::gera();

   // neste caso inicializa o "ncurses"...
   let opcao:Vec<_> = args().collect();
   if opcao.contains(&String::from("ncurses"))
      { Grafico::visualiza(&mut limpeza); }
   // padrão, com output no terminal mesmo em tempos ...
   else 
      { visualizacao_padrao(limpeza); }

}

fn visualizacao_padrao(mut fila:FilaExclusao) {
   // limite de alguns minutos.
   let minutos:u64 = {
      /* convertendo em minutos, e convertendo 
       * o tipo primitivo. */
      // delimite intervalo do sorteio..
      sortear::u8(9..=23) as u64 * 60_u64
   };
   let limite:Duration = Duration::from_secs(minutos);
   // criando cronômetro para contar tempo.
   let cronometro = Instant::now();

   /* a contagem não pode passar o delimitado e,
    * todos ítens não podem ter sido apagados. */
   while cronometro.elapsed() < limite 
   && !fila.vazia() {
      // visualizando progresso.
      fila.visualiza();
      // tempo restante de visualização, delimitado.
      let restante = limite - cronometro.elapsed();
      println!(
         "loop do dashboard termina {}",
         legivel::tempo(restante.as_secs(), true)
      );
      // a cada tempo delimitado.
      thread::sleep(Duration::from_secs_f32(17.7));
   }
}
