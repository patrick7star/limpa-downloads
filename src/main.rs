
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

   // criando executáveis se não houver.
   linka_executaveis();

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

/* cria link simbólico tanto para a versão em
 * debug, quanto para o binário final. */
use std::os::unix::fs::symlink;
use std::env::current_exe;
use std::path::{Path, PathBuf};

// complementa link ao executável.
fn computa_caminho(caminho_str:&str) -> PathBuf {
   // à partir do caminho do executável ...
   match current_exe() {
      Ok(mut base) => {
         // remove executável do caminho.
         base.pop(); 
         // sai do subdiretório 'release'.
         base.pop(); 
         // sai do subdiretório 'target'.
         base.pop();
         // complementa com o caminho passado.
         base.push(caminho_str);
         return base;
      } Err(_) =>
         { panic!("não foi possível obter o caminho do executável!"); }
   }
}

fn linka_executaveis() {
   // caminho aos executáveis.
   let caminho_str = "target/release/limpa_downloads";
   let executavel = computa_caminho(caminho_str);
   let executavel_debug: PathBuf;
   let caminho_str = "target/debug/limpa_downloads";
   executavel_debug = computa_caminho(caminho_str);

   // seus links simbólicos:
   let ld_link = computa_caminho("LD");
   let ld_debug_link = computa_caminho("LD_debug");

   if ld_link.as_path().exists() && 
   ld_link.as_path().is_symlink() {
      if executavel.as_path().exists() 
         { println!("binário do executável existe."); }
      println!("LD já existe.");
   } else {
      match symlink(executavel.as_path(), ld_link.as_path()) {
         Ok(_) => {
            print!("criando LD ... ");
            println!("com sucesso.");
         } Err(erro) => 
            { println!("erro:[{}]", erro); }
      };
   }

   if ld_debug_link.as_path().exists() && 
   ld_link.as_path().is_symlink() { 
      println!("LD(debug) já existe.");
      if executavel_debug.exists() 
         { println!("binário do executável(DEBUG) existe."); }
   } else {
      print!("criando LD(DEBUG) ... ");
      match symlink(executavel_debug.as_path(), ld_debug_link.as_path()) {
         Ok(_) => {
            print!("criando LD(debug) ... ");
            println!("com sucesso.");
         } Err(erro) => 
            { println!("erro:[{}]", erro); }
      };
   }
}
