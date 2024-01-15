
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
//mod configuracao;
// #[allow(unused_imports)]
// mod interface_grafica;
#[allow(unused_imports)]
mod notificacoes;

// biblioteca padrão do Rust:
use std::env::args;
use std::process::Command;
use std::path::PathBuf;
use std::convert::TryInto;

// procedimento que levanta uma notificação por algo corrido no programa.
fn alerta_sobre_remocoes(obj: &mut FilaExclusao, total_inicial: usize) 
{
   if cfg!(debug_assertions)
      { println!("qtd. inicial: {total_inicial}"); }
   // computando uma diminuição do total.
   let diminuicao: usize = {
      let a = total_inicial as i32;
      let b = obj.total() as i32;
      (a - b).abs().try_into().unwrap()
   };
   // tentando visualizar o 'overflow'.
   if cfg!(debug_assertions)
      { println!("diminuição: {diminuicao}"); }
   // verificando se algo foi removido.
   let houve_alguma_variacao = diminuicao > 0;

   // emissão da notificação se necessária.
   if houve_alguma_variacao 
      { notificacoes::informa_n_itens_removidos(diminuicao); }
}

// seleção do modo gráfico/ou não de visualizar este programa.
fn tipo_de_visualizacao(objeto: &mut FilaExclusao) {
   if args().any(|s| s == "--ncurses")
      // neste caso inicializa o "ncurses"...
      { Grafico::visualiza(objeto); }
   /* para encurtar o comando no agendador 'cron', portanto não será 
    * mais preciso escrever o comando do emulador de terminal, para 
    * lançar o comando propriamente. */
   else if args().any(|s| s == "--lanca-janela") {
      // caminho do executável.
      let funcao: fn(&str) -> PathBuf = links::computa_caminho;
      let caminho_exe = funcao("target/debug/limpa_downloads");
      let executavel = format!("{} --ncurses", caminho_exe.display());
      /* criando comando para lança o programa na interface do ncurses 
      numa nova janela. */
      let mut comando = Command::new("mate-terminal");

      comando.args([
         "--hide-menubar",
         "--command",
         executavel.as_str(),
      ]);

      comando.spawn().unwrap().wait().unwrap();
      println!("abriu uma nova janela para execução do programa.");
   }
   else 
      /* padrão, apenas mostra lista de exclusão de hoje, e deleta já 
       * expirados. */
      { objeto.visualiza(); }
}

fn main() {
   let mut limpeza = FilaExclusao::gera();
   // total de itens inicialmente, quando a fila é gerada.
   let total_inicial = limpeza.total();

   tipo_de_visualizacao(&mut limpeza);
   // lançando notificações sobre as operações realizadas.
   alerta_sobre_remocoes(&mut limpeza, total_inicial);

   if cfg!(unix) {
      // criando executáveis se não houver, apenas no Linux por enquanto.
      links::linka_executaveis("LD");
   }
}

