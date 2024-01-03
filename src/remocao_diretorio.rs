

/* Tentando fazer computar se é hora ou não de deletar um diretório, baseado 
 * no seu tempo de criação e acesso. Se ele tiver conteúdo, tal tempo será uma
 * média ponderada do mais rencete(com mais peso) até o mais profundo na 
 * raíz do diretório, esses com menos pesos.
 */

use std::path::Path;
use std::io::{Result, Error};
use std::fs::{read_dir,Metadata};

fn atravessa_o_diretorio(raiz: &Path, colecao: &mut Vec<Metadata>) 
  -> Result<()> 
{
   if raiz.is_dir() {
      for entrada in raiz.read_dir()? {
         let sub = entrada?;
         let nova_raiz = sub.path();
         let caminho = nova_raiz.as_path();
         drop(atravessa_o_diretorio(caminho, colecao));
      }
   } else if raiz.is_file() { 
      colecao.push(raiz.metadata()?); 
   }
   Ok(())
}

use std::time::{Duration, SystemTime};
use std::convert::TryInto;
use utilitarios::legivel::tempo;

pub fn tempo_medio_ultimo_acesso<P>(raiz: &P) -> Duration
  where P: AsRef<Path> + ?Sized 
{
   let mut estatisticas = Vec::<Metadata>::with_capacity(10);
   let inicio = SystemTime::now();
   let mut segundos_acumulado = Duration::new(0,0);

   // varrendo à partir da raiz e capturando 'metadados'...
   atravessa_o_diretorio(raiz.as_ref(), &mut estatisticas).unwrap();
   // computando o total de 'metadados' filtrados.
   let total: usize = estatisticas.len();

   for stat in estatisticas.drain(..) {
      let fim = stat.accessed().unwrap();
      // registro do último acesso menos o ínicio do Unix.
      segundos_acumulado += inicio.duration_since(fim).unwrap();
   }
   // computando média...
   segundos_acumulado / total.try_into().unwrap()
}

pub fn tempo_medio_criacao<P>(raiz: &P) -> Duration
  where P: AsRef<Path> + ?Sized 
{
   let mut estatisticas = Vec::<Metadata>::with_capacity(10);
   let inicio = SystemTime::now();
   let mut segundos_acumulado = Duration::new(0,0);

   // varrendo à partir da raiz e capturando 'metadados'...
   atravessa_o_diretorio(raiz.as_ref(), &mut estatisticas).unwrap();
   // computando o total de 'metadados' filtrados.
   let total: usize = estatisticas.len();

   for stat in estatisticas.drain(..) {
      let fim = stat.created().unwrap();
      // registro do último acesso menos o ínicio do Unix.
      segundos_acumulado += inicio.duration_since(fim).unwrap();
   }
   // computando média...
   segundos_acumulado / total.try_into().unwrap()
}


#[cfg(test)]
#[cfg(target_os="windows")]
mod tests {
   use super::*;

   // caminho raíz principal para testes iniciais.
   const CAMINHO_STR: &'static str = concat!(
      env!("HOMEPATH"), '\\',
      "Downloads"
   );

   #[test]
   fn verificacao_basica_de_varredura() {
      let caminho = Path::new(CAMINHO_STR);
      let mut metadados_de_arquivos = Vec::<Metadata>::new();

      assert_eq!(metadados_de_arquivos.len(), 0);
      let _ = atravessa_o_diretorio(caminho, &mut metadados_de_arquivos);
      assert!(metadados_de_arquivos.len() > 0);

      for mt in metadados_de_arquivos
         { println!("{:#?}", mt); }
   }

   #[test]
   fn calculo_do_tempo_medio_de_acesso() {
      let t = tempo_medio_ultimo_acesso(CAMINHO_STR);
      let tempo_legivel = tempo(t.as_secs(), true);
      println!("médio do último acesso: {}", tempo_legivel);
   }

   #[test]
   fn ultimo_acesso_medio_so_diretorios() {
      for entrada in read_dir(CAMINHO_STR).unwrap() {
         let path = entrada.unwrap().path();

         if path.is_dir() {
            let cmh = path.as_path();
            let t = tempo_medio_ultimo_acesso(cmh);
            let _t = tempo_medio_criacao(cmh);
            println!(
               "o arquivo {:#?}, criada há {}, foi acessado há {}.",
               // nome do arquivo com uma string incomum.
               path.file_name().unwrap(),
               tempo(_t.as_secs(), false),
               tempo(t.as_secs(), false)
            );
         }
      }
   }
}
