
/* cria link simbólico tanto para a versão em debug, quanto para o 
 * binário final. */
#[cfg(target_os="linux")]
use std::os::unix::fs::symlink;
use std::env::current_exe;
use std::path::PathBuf;
use std::ffi::{OsStr};
use std::path::{Path, Component};
use std::env::{var, VarError};
use std::io;


/// Computa o caminho até o projeto, baseado que, tal executável deve está
/// no lugar comum onde uma simples compilação do Rust o faz se não for 
/// definida diferente. Portanto em algum subdiretório do diretório 'target'.
pub fn computa_caminho(caminho_str:&str) -> PathBuf {
   match current_exe() {
   // à partir do caminho do executável ...
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

fn cria_linques_no_repositorio(nome_do_linque: &str) -> io::Result<PathBuf> 
{
   let caminho_do_executavel = current_exe()?;
   let caminho_repositorio = {
      match var("LINKS") {
         Ok(data) => Ok(data),
         Err(tipo_de_erro) => {
            let erro_a = io::ErrorKind::InvalidInput;
            let erro_b = io::ErrorKind::InvalidData;

            match tipo_de_erro {
               VarError::NotPresent => Err(erro_a),
               VarError::NotUnicode(_) => Err(erro_b)
            }
         }
      }
   }?;
   let fonte = &caminho_do_executavel;
   let destino = Path::new(&caminho_repositorio).join(nome_do_linque);
   let bate = Component::Normal(OsStr::new("release"));

   // Verificação se estamos falando apenas da parte 'release'.
   assert!(fonte.components().any(|part| part == bate));
   println!("fonte: {:?}\ndestino: {:?}", fonte, destino);
   symlink(fonte, &destino)?;
   Ok(destino)
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
   let executavel = computa_caminho(caminho_ao_executavel);
   // caminho do linque para o executável.
   let linque_otimizado = computa_caminho(nome_do_linque);

   // criação do linque para o executável otimizado.
   if linque_otimizado.exists() {
      // println!("linque do executável já existe."); 
      println!("{}", mensagem_i); 
   } else {
      print!("como não existe, criando '{}' ... ", nome_do_linque);
      #[cfg(target_os="linux")]
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

   match cria_linques_no_repositorio(nome_do_linque) {
      Ok(caminho) => { 
         assert!(caminho.exists()); 
         println!("Linque criado com sucesso em $LINKS."); 
      } Err(classificacao_do_erro) => { 
         match classificacao_do_erro.kind() {
            io::ErrorKind::AlreadyExists =>
               { println!("Já existe um linque em $LINKS."); }
            _ =>
               { panic!("{}", classificacao_do_erro); }
         }
      } 
   }
}

#[cfg(test)]
mod tests {
   use std::fs::{remove_file};

   #[test]
   fn criacao_de_linque_no_repositorio_de_linques() {
      let nome = "linque-do-limpa-downloads-teste";
      let out = super::cria_linques_no_repositorio(nome).unwrap();

      assert!(remove_file(out).is_ok());
   }
}
