
/* cria link simbólico tanto para a versão em debug, quanto para o 
 * binário final. */
#[cfg(target_os="linux")]
use std::os::unix::fs::symlink;
use std::env::current_exe;
use std::ffi::{OsStr};
use std::path::{Path, Component, PathBuf};
use std::env::{var, VarError};
use std::io;


/* Computa o caminho do diretório do projeto, então anexa o resto do caminho
 * dado. Entretanto, é um método diferente do que já tem deste tipo de 
 * função aqui. */
pub fn computa_caminho<P: AsRef<Path>>(complemento: P) -> PathBuf
{
   let mut caminho = current_exe().unwrap();
   const PROJETO: &str = "limpa-downloads";
   let bate = OsStr::new(PROJETO);
   let mut base = caminho.file_name();

   while base != Some(&bate) { 
      caminho.pop(); 
      base = caminho.file_name();
   }
   caminho.join(complemento)
}

pub fn linka_executaveis(nome_do_linque: &str) 
{
   match cria_linques_no_repositorio(nome_do_linque) {
      Ok(caminho) => { 
         assert!(caminho.exists()); 
         println!("Linque criado com sucesso em $LINKS."); 
      } Err(classificacao_do_erro) => { 
         match classificacao_do_erro.kind() {
            io::ErrorKind::AlreadyExists =>
               { println!("Já existe um linque em $LINKS."); }
            io::ErrorKind::Unsupported =>
               { println!("Versão 'debug' não cria linque do repositório.");}
            _ =>
               { panic!("{}", classificacao_do_erro); }
         }
      } 
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
   let alvo_correto = fonte.components().any(|part| part == bate);

   if cfg!(debug_assertions)
      { println!("fonte: {:?}\ndestino: {:?}", fonte, destino); }
   
   // Verificação se estamos nos referindo apenas da parte 'release'.
   if alvo_correto {
      symlink(fonte, &destino)?;
      Ok(destino)
   } else
      { Err(io::ErrorKind::Unsupported.into()) }
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
