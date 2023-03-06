

/** Não necessáriamente é relaicionado ao
 desenho dos diretórios, subdiretórios e
 arquivos, porém, tenta lineariza-lôs.
*/

// biblioteca do Rust:
use std::path::{Path, PathBuf};
use std::fs::read_dir;

// do módulo superior:
use crate::arvore::{GALHO_H, GALHO_VH};

// lista com 'paths' são arquivos.
type Arquivos = Vec<PathBuf>;


fn percorre(pai: &Path, folhas: &mut Arquivos) {
   // navegando arquivos e diretórios.
   let lista_no_dir = {
      read_dir(pai)
      .expect("erro ao lê diretório.")
   };
   // navegando em seu conteúdo...
   for item in lista_no_dir {
      let item = item.unwrap();
      let caminho = item.path();
      // grava arquivo.
      let path = caminho.as_path();
      if path.is_file() 
         { folhas.push(caminho); }
      // continua se for um diretório.
      else if path.is_dir() 
         { percorre(path, folhas); }
      // burla links.
      else if path.is_symlink()
         { println!("'{}' link simbólico.", path.display()); }
   }
}

/** retorna lista com todos os arquivos 
 de dado diretório, e também em seus 
 sub-diretórios também.
*/
pub fn listagem(raiz: &Path) -> Arquivos {
   // caminho serão colocadas aqui.
   let mut lista: Arquivos = Arquivos::new(); 
   /* filtrando todos os caminhos dos 
    * arquivos pertencente a este caminho
    * dado, seja eles do atual diretório
    * ou subdiretórios. */
   percorre(raiz, &mut lista);
   return lista;
}

/** retorna uma string com representação
 de árvore do caminho passado.
*/
pub fn ramifica_caminho(caminho: &str) -> String {
   let mut galho: String = String::new();
   // verificado se raíz "já foi computada".
   let mut passou_raiz: bool = false;

   // se começa com barra...
   let mut componentes = caminho.split("/");
   if caminho.starts_with("/") 
      { componentes.next().unwrap(); }

   for (k, entrada) in componentes.enumerate() {
      let esboco: String;
      if !passou_raiz {
         passou_raiz = true;
         esboco = format!("|{}|", entrada);
      } else {
         esboco = format!(
            "{recuo} {}{}{}", 
            GALHO_VH,
            GALHO_H,
            entrada,
            recuo = &" ".repeat(2*k)
         );
      }
      // adiciona caminho recuádo ...
      galho.push_str(esboco.as_str());
      // e quebra-de-linha em seguida.
      galho.push('\n');
   }
   return galho;
}


#[cfg(test)]
mod tests {
   // pegando tudo acima...
   use super::*;

   #[test]
   fn testa_listagem() {
      let caminho = Path::new(env!("HOME"));
      let caminho = caminho.join("Pictures");

      for path in listagem(caminho.as_path()) { 
         let ramicacao = ramifica_caminho(path.to_str().unwrap());
         println!("{}", ramicacao); 
      }
      // avaliação manual.
      assert!(true);
   }

   #[test]
   fn testa_ramifica_caminho() {
      let caminhos = [
         "/home/user_um/Videos/RingII",
         "/etc/default/grub/grub.cfg",
         "/usr/share/themes/backgrounds",
         "target/release/executavel"
      ];
      for cmh in caminhos.iter() 
         { println!("{}", ramifica_caminho(cmh)); }
   }
}
