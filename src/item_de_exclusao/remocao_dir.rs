/*! Um trabalho especial com a geração de 'Item's e remoção do mesmo, que
 contém subdiretórios e arquivos lá dentro.
*/

// Biblioteca padrão:
use std::fs::{remove_dir, remove_file, OpenOptions};
use std::io::{Write, ErrorKind};
use std::path::Path;
use std::fmt::Debug;
use std::process::{Command};


/** Aplica remoção dentro de diretório, apenas pegando caminhos e subdiretórios
 * expirados dentro do diretório. 
 */
pub fn remocao_completa<P>(caminho:&P) 
  where P: AsRef<Path> + ?Sized + Debug
{
   match caminho.as_ref().read_dir()
   {
      Ok(entradas_do_diretorio) => {
         for entrada in entradas_do_diretorio {
            let x = entrada.unwrap().path(); 

            if x.as_path().is_symlink() 
               { eprintln!("{:#?} é um link simbólico!", x); }
            else if x.as_path().is_file() { 
               if let Err(erro) = remove_file(x) 
                  { registra_no_log(erro.kind(), caminho.as_ref()); }
            } else if x.as_path().is_dir() { 
               // chamando função de modo recursivo ...
               remocao_completa(x.as_path());
               /* Excluindo arquivos do diretório e talvez subdiretórios, 
                * então apaga o diretório. */
               if let Err(erro) = remove_dir(x.as_path()) 
                  { registra_no_log(erro.kind(), caminho.as_ref()); }
            }
         }
      } Err(erro) => { 
      /* Não interrompe mais num erro, apenas registra um log. */
         { registra_no_log(erro.kind(), caminho.as_ref()); }
      }
   };

   // Removendo diretório "raíz" passado.
   if let Err(erro) = remove_dir(caminho) 
      { registra_no_log(erro.kind(), caminho.as_ref()); }
}

pub fn acesso_medio_dir<P>(caminho:&P) -> f32
  where P: AsRef<Path> + ?Sized
{
   let mut tempo: f32 = 0.0;
   let mut contador: f32 = 0.0;

   // Contabilização dos valores, então retorna o cálculo da média.
   auxiliar_amd(caminho, &mut tempo, &mut contador);
   tempo / contador
}

fn auxiliar_amd<P>(caminho:&P, tm:&mut f32, ctd:&mut f32) 
  where P: AsRef<Path> + ?Sized
{
   *ctd += 1.0;
   *tm += match caminho.as_ref().metadata() {
      Ok(metadados) => {
         metadados.accessed()
         .unwrap().elapsed()
         .unwrap().as_secs_f32()
      } Err(_) => 0.0
   };
   for entrada in caminho.as_ref().read_dir().unwrap() {
      let x = entrada.unwrap().path(); 
      if x.as_path().is_symlink()
         { continue; }
      else if x.as_path().is_file() { 
         *tm += {
            match x.as_path().metadata() {
               Ok(metadados) => {
                  metadados
                  .accessed()
                  .unwrap()
                  .elapsed()
                  .unwrap()
                  .as_secs_f32()
               } Err(_) => 0.0
            }
         };
         *ctd += 1.0;
      } else if x.as_path().is_dir() 
         { auxiliar_amd(x.as_path(), tm, ctd); }
   }
}

/** Preciso de uma data para o log, porém não quero, por enquanto, ter que 
 * usar uma biblioteca externa. Portanto, irei usar o programa 'date' do Unix,
 * que sempre tem nos sistemas. Se não houver, a aplicação será interrompida,
 * que é algo válido, pois não parece acontecer frequente.
 */
fn data_e_horario_para_log() -> String 
{
   const PROG:&str = "/usr/bin/date";
   const ARG:&str = "+'%B %d %H:%M:%S'";
   let comando = Command::new(PROG).arg(ARG).output().unwrap();
   let mut saida: String;
   
   saida = String::from_utf8_lossy(&comando.stdout).to_string();
   // Removendo quebra de linha...
   saida.pop();
   // Removendo àspas simples...
   saida = saida.trim_end_matches('\'').to_string();
   saida = saida.trim_matches('\'').to_string();
   saida
}

/// Registra um erro no arquivo de log do programa.
fn registra_no_log(erro: ErrorKind, caminho: &Path)
{
   const CAMINHO_LOG: &str = "data/logs";
   let mut arquivo_log = {
      OpenOptions::new()
      .append(true).create(true)
      .open(CAMINHO_LOG).unwrap()
   };
   let pathfmt = caminho.display();
   let data_horario = data_e_horario_para_log();
   let log_fmt = format!("{} INFO {}: '{}'\n", data_horario, erro, pathfmt);
   let bytes = log_fmt.as_bytes();

   arquivo_log.write(bytes).unwrap();
   arquivo_log.flush().unwrap();
}


#[cfg(test)]
mod tests {
   extern crate utilitarios;

   use utilitarios::{
      legivel::tempo_legivel_decimal,
      arvore::arvore
   };
   use std::path::Path;
   use super::*;
   #[allow(unused)]
   use std::fs::{remove_dir_all};
   use std::env::temp_dir;
   use std::process::Command;

   #[test]
   fn stringficacao_do_comando() {
      let result = data_e_horario_para_log();
      println!("Result: '{}'", result);
   }

   #[test]
   fn testa_rc() {
      // descompactando diretório de teste ...
      let mut comando: Command = Command::new("/usr/bin/unzip");
      comando.arg("./src/item_de_exclusao/testaRC.zip"); 
      comando.arg("-d");
      comando.arg("/tmp");
      println!("{:#?}", comando);
      let msg_ok = "descompactou \"diretório teste\"";
      let msg_erro = "sem descompactar, não possível continuar o teste.";
      match comando.spawn() {
         Ok(mut processo) => { 
            processo.wait().expect("o comando falhou!!");
            println!("{}", msg_ok); 
         }
         Err(erro) => { panic!("{}{}", msg_erro, erro); }
      };
      let caminho = {
         temp_dir()
         .as_path()
         .join("diretório_teste")
      };
      println!("{:#?}", caminho);
      assert!(caminho.exists());
      let caminho_str = {
         caminho
         .as_path()
         .to_str()
         .unwrap()
      };
      println!("{}", arvore(caminho_str, true));
      remocao_completa(caminho.as_path());
      assert!(!caminho.exists());
   }

   #[test]
   fn testa_amd() {
      let caminho = Path::new(env!("RUST_CODES"));
      let t = acesso_medio_dir(caminho);
      let traducao = tempo_legivel_decimal(t, true);
      println!("último acesso: {}", traducao.unwrap());
      // avaliação manual.
      assert!(true);
   }
}
