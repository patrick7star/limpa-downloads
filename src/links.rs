
/* cria link simbólico tanto para a versão em debug, quanto para o 
 * binário final. */
#[cfg(target_os="unix")]
use std::os::unix::fs::symlink;
use std::env::current_exe;
use std::path::PathBuf;

// complementa link ao executável.
pub fn computa_caminho(caminho_str:&str) -> PathBuf {
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

pub fn linka_executaveis(nome_do_linque: &str) {
   let caminho_ao_executavel;
   let mensagem_i: &str;

   // seleção baseado no tipo de optimização na compilação:
   if cfg!(debug_assertions) {
      caminho_ao_executavel = "target/debug/limpa_downloads";
      mensagem_i = "linque do executável(debug) já existe.";
   } else {
      caminho_ao_executavel = "target/release/limpa_downloads";
      mensagem_i = "linque do executável já existe.";
   }

   // caminho aos executáveis.
   // let caminho_ao_executavel = "target/release/limpa_downloads";
   let executavel = computa_caminho(caminho_ao_executavel);
   // caminho do linque para o executável.
   let linque_otimizado = computa_caminho(nome_do_linque);

   // criação do linque para o executável otimizado.
   if linque_otimizado.exists() {
      // println!("linque do executável já existe."); 
      println!("{}", mensagem_i); 
   } else {
      print!("como não existe, criando '{}' ... ", nome_do_linque);
      #[cfg(target_os="unix")]
      let resultado_criacao_do_link = symlink(
         executavel.as_path(), 
         linque_otimizado.as_path()
      );
      #[cfg(target_os="windows")]
       let resultado_criacao_do_link: Result <(), &str> = Err(
          "[error]ainda não compatível com o Windows!!!"
      );
      match resultado_criacao_do_link {
         Ok(_) => {
            println!("com sucesso.");
         } Err(_) => { 
            println!("erro ao tentar criar linque!!!");
         }
      };
   }
}
