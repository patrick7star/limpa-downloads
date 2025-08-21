/*! Um trabalho especial com a geração de 'Item's e remoção do mesmo, que
 contém subdiretórios e arquivos lá dentro.
*/

// Biblioteca padrão:
use std::fs::{remove_dir, remove_file};
use std::path::Path;
use std::fmt::Debug;


pub fn remocao_completa<P>(caminho:&P) 
  where P: AsRef<Path> + ?Sized + Debug
{
   let entradas = caminho.as_ref().read_dir().unwrap();
   for entrada in  entradas {
      let x = entrada.unwrap().path(); 
      if x.as_path().is_symlink() 
         { eprintln!("{:#?} é um link simbólico!", x); }
      else if x.as_path().is_file() { 
         let nome = x.as_path().file_name().unwrap();
         eprint!("removendo {:#?}...", nome); 
         match remove_file(x) {
            Ok(_) => 
               { eprintln!("feito!"); }
            Err(_) =>
               { eprintln!("ALGO DEU ERRADO!"); }
         };
      } else if x.as_path().is_dir() { 
         // chamando função de modo recursivo ...
         remocao_completa(x.as_path());
         /* Excluindo arquivos do diretório e talvez subdiretórios, então 
          * apaga o diretório. */
         print!("removendo [{:#?}] ...", x.as_path());
         match remove_dir(x.as_path()) {
            Ok(_) => 
               { eprintln!("feito!"); }
            Err(_) => 
               { eprintln!(""); }
         };
      }
   }
   // removendo diretório "raíz" passado.
   eprint!("removendo [{:#?}] ...", caminho);
   match remove_dir(caminho) {
      Ok(_) => 
         { eprintln!("feito!"); }
      Err(_) => 
         { eprintln!("ALGO DEU ERRADO!!!"); }
   };
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

pub fn acesso_medio_dir<P>(caminho:&P) -> f32
  where P: AsRef<Path> + ?Sized
{
   let mut tempo: f32 = 0.0;
   let mut contador: f32 = 0.0;

   auxiliar_amd(caminho, &mut tempo, &mut contador);
   tempo / contador
}

/* Desconsidera diretórios dentro de diretórios,
 * só retorna 'falso', se e somente se, um
 * arquivo, ou "variantes" dele forem encontrados. */
#[allow(dead_code)]
fn sem_arquivos(caminho: &Path, contador: &mut u8) {
   if caminho.is_dir() {
      for e in caminho.read_dir().unwrap() {
         let path = e.unwrap().path();
         sem_arquivos(path.as_path(), contador);
      }
   } else if caminho.is_file() {
      // falseia conjutura.
      *contador += 1;
   } else {
      panic!("tipo não implementado, mas se não for arquivo então conta!");
   }
}
/* varre todo diretório, e seus subdiretórios,
 * a procura de um único arquivo, se não houver
 * nada, então será apenas considerado como
 * vázio, por mais que haja vários diretórios
 * vázios internos, um dentro dos outros. */
#[allow(dead_code)]
pub fn dir_sem_arquivos<P>(caminho: P) -> bool
  where P: AsRef<Path> 
{ 
   let mut contador = 0;
   sem_arquivos(caminho.as_ref(), &mut contador); 
   /* se contabilizar, no mínimo um arquivo, então
    * o diretório não é composto apenas de diretórios
    * vázios. */
   contador == 0
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
   use std::fs::{remove_dir_all};
   use std::env::temp_dir;
   use std::process::Command;

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

   #[test]
   #[allow(non_snake_case)]
   fn diretorioVazio() {
      // criação temporária da amostra a ser testada.
      Command::new("bash")
      .arg("src/item_de_exclusao/variados-tipos-dirs.sh")
      .output().unwrap();
      println!("árvore de diretórios criadas para testagem.");
      // verifica se foi criada...
      let raiz = temp_dir().join("pasta-testes");
      assert!(raiz.as_path().exists());
      // entradas e saidas esperadas:
      let entrada_e_saida = [
         (raiz.join("amostra_i"), true),
         (raiz.join("amostra_ii"), true),
         (raiz.join("amostra_iii"), true),
         (raiz.join("amostra_iv"), false),
         (raiz.join("amostra_v"), true),
         (raiz.join("amostra_vi"), false),
         (raiz.join("amostra_vii"), true),
      ];
      // execução do teste em sí.
      for (caminho, saida) in entrada_e_saida.into_iter() {
         let nome = &caminho.file_name().unwrap();
         let resultado = dir_sem_arquivos(caminho.clone());
         println!("{:#?}, sem arquivos? {}", nome, resultado);
         assert_eq!(resultado, saida);
      }
      // excluindo raíz.
      remove_dir_all(raiz).unwrap();
   }
}
