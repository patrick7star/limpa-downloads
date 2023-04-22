
/**!
 Um trabalho especial com a geração de
 'Item's e remoção do mesmo, que
 contém subdiretórios e arquivos lá
 dentro.
*/

// biblioteca padrão:
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
         /* excluindo arquivos do diretório e 
          * talvez subdiretórios, então apaga 
          * o diretório. */
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
   return tempo / contador;
}

pub fn diretorio_vazio<P>(caminho: &P) -> bool 
  where P: AsRef<Path> + ?Sized
{
   let mut contador = 0;
   match caminho.as_ref().read_dir() {
      Ok(entradas) => {
         for _ in entradas
            { contador += 1; }
      } Err(_) => ()
   };
   return contador == 0;
}

#[cfg(test)]
mod tests {
   extern crate utilitarios;
   use utilitarios::{
      legivel::tempo,
      arvore::arvore
   };
   use std::path::Path;
   use super::{
      remove_dir,
      acesso_medio_dir, 
      remocao_completa,
      diretorio_vazio
   };
   use std::fs::create_dir;
   use std::env::temp_dir;
   use std::process::Command;
   use std::thread::sleep;
   use std::time::Duration;

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
      let caminho = Path::new("/home/savio/Documents/códigos_rust");
      let t = acesso_medio_dir(caminho);
      println!("último acesso: {}", tempo(t as u64, true));
      // avaliação manual.
      assert!(true);
   }

   #[test]
   #[allow(non_snake_case)]
   fn testaDV() {
      let caminho = temp_dir().as_path().join("DirTeste/");
      println!("{}", caminho.as_path().display());
      assert!(!caminho.exists());
      create_dir(caminho.as_path()).unwrap();
      assert!(caminho.exists());
      sleep(Duration::from_secs(5));
      assert!(diretorio_vazio(caminho.as_path()));
      remove_dir(caminho.as_path()).unwrap();
      assert!(!caminho.as_path().exists());
   }
}
