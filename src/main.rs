/*!
   Limpa os downloads antigos que podem "vencer" rapidamente, ou seja, 
 não será utilizado mais.

   O programa tem dois tipos de interfaces: a em modo-de-texto e uma
 "semi-gráfica"(utilizando ncurses). Você pode acessar qualquer uma,
 sendo a primeira apenas executando o programa em terminal, e a outra
 com a opção '--ncurses'. Dentro desta segunda também tem a opção 
 "lança janela", que desanexa a janela ncurses do atual terminal
 chamado; para chamar tal opção use '--lanca-janela', é ótimo para 
 executar tal programa de forma agendada.
*/

// Módulos do programa.
mod item_de_exclusao;
mod janela_grafica;
mod letreiro;
mod links;
//mod configuracao;
mod notificacoes;

// Próprio módulos do projeto:
use item_de_exclusao::FilaExclusao;
use janela_grafica::{Grafico};
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
      println!("Abriu uma nova janela para execução do programa.");
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
   // Lançando notificações sobre as operações realizadas.
   alerta_sobre_remocoes(&mut limpeza, total_inicial);
   // Cria linques locais e no repositório específico pra eles.
   links::linka_executaveis("limpa-downloads");
}

