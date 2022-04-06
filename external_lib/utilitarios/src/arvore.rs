/*
 * fazer uma árvore para visualização
 * de forma organizada, ramificando seus 
 * arquivos e sub-diretórios.
 */


// biblioteca padrão do Rust.
use std::fs::read_dir;
use std::path::Path;

// meus módulos:
mod constroi_simbolos;
use constroi_simbolos::{matriciar_string, matriz_para_string};
use crate::terminal_dimensao::{dimensao, Largura};


// tipos de galhos:
/// galho do tipo horizontal.
const GALHO_H:char = '\u{2500}';
/// galho vertical.
const GALHO_V:char = '\u{2502}';
/// galho conector vertical e horizontal.
const GALHO_VH:char = '\u{2570}';
/// conector entre dois verticais e um horizontal.
const GALHO_VHV:char = '\u{251c}';


/* escreva toda uma trilha, com ramificações
 * de sub-diretórios e arquivos, dado uma raiz
 * principal. */
fn desenha_trilha(esboco:&mut String, caminho:&Path , pfd:&mut u8) {
   // navegando arquivos e diretórios.
   let lista_no_dir = match read_dir(caminho) {
       Ok(iterador) => iterador,
       Err(_) => panic!("diretório erro:\"{:#?}\"", caminho),
    };
   
   // navegando em seu conteúdo...
   for item in lista_no_dir {
      let item = item.unwrap();

      // possível link-símbolico.
      match item.path().as_path().read_link() {
          //se for link-simbólico, passar ele...
          Ok(sl) => {
            println!("é link-simbólico, burlando...{:#?}",sl); 
           continue;
          },
          Err(_) => (), //apenas ignorando...
      };

      // string do caminho.
      let pth_str = {
         item.path()
         .into_os_string()
         .into_string()
         .expect("falha ao obter caminho no formato de string!")
      };
      // nome do arquivo/diretório do caminho.
      let nome_pth = item.file_name().into_string().unwrap();

      // se for um diretório usar de recursividade.
      let espacamento = " ".repeat((*pfd) as usize);
      let pth = Path::new(pth_str.as_str());
      if pth.is_dir() {
          // molde de diretório(dois pontos).
          let mut str_aux = format!("{1}{2}{3}{4} {0}:\n",
                                    nome_pth,espacamento,
                                    GALHO_VH,GALHO_H,GALHO_H);
          // ajusta a string na tela.
          ajusta_string(&mut str_aux, true);
          esboco.push_str(str_aux.as_str());
          let novo_path = Path::new(pth_str.as_str());
          // cada chamada recursiva, aumenta a profundidade.
          (*pfd) += 3; 
          desenha_trilha(esboco, novo_path, pfd);
          // "volta" um diretório...
          (*pfd) -= 3; 
      }
      // se for apenas um arquivo, só registra.
      else {
         // molde diferente para arquivos:
         let mut straux = format!(
            "{1}{2}{3}{4} \"{0}\"\n",
            nome_pth, espacamento,
            GALHO_VH,GALHO_H,GALHO_H
         );
         // ajusta a string na tela.
         ajusta_string(&mut straux, false);     
         esboco.push_str(straux.as_str());
      }
   }
}

/* caso uma string exceda a tela do terminal
 * a função vai reduzi-lá e implicitar que
 * tal string é mais extensa, continua... */
static mut JA_COMPUTADO:bool = false;
static mut LARGURA:usize = u16::MAX as usize;

fn ajusta_string(s:&mut String, e_diretorio:bool) {
   // obter a largura.
   let largura:usize;
   unsafe {
      if !JA_COMPUTADO {
         /*
         largura = match terminal_size() {
            Some((Width(l), _)) => l as usize,
            None => panic!("erro ao obter LARGURA do terminal"),
         };
         */
         largura = match dimensao() {
            Some((Largura(l), _)) => l as usize,
            None => panic!("erro ao obter LARGURA do terminal"),
         };
         // obtendo largura de vez.
         LARGURA = largura;
         // dá como já calculado tal valor.
         JA_COMPUTADO = true;
      } 
      // apenas atribuir valor JÁ calculado.
      else { largura = LARGURA; }
   }

   // comprimento da string.
   let str_largura = s.len();
   if str_largura > largura {
      //let intervalo = (largura-4)..;
      if e_diretorio 
         { s.replace_range((largura-8)..,"(...):\n"); }
      else 
         {s.replace_range((largura-2)..,"...\n");}
   }
}
   
/* escreva trilha, porém só registros diretórios...
 * arquivos não será ramificados na árvore. */
fn desenha_trilha_dirs(esboco:&mut String, caminho:&Path , pfd:&mut u8) {
   // navegando arquivos e diretórios.
   let lista_no_dir = match read_dir(caminho) {
       Ok(iterador) => iterador,
       Err(_) => panic!("diretório erro:\"{:#?}\"", caminho),
    };
   
   // navegando em seu conteúdo...
   for item in lista_no_dir {
      let item = item.unwrap();

      // possível link-símbolico.
      match item.path().as_path().read_link() {
         //se for link-simbólico, passar ele...
         Ok(sl) => {
         println!("é link-simbólico, burlando...{:#?}",sl); 
         continue;
         },
         Err(_) => () //apenas ignorando...
      };

      // string do caminho.
      let pth_str = {
         item
         .path()
         .into_os_string()
         .into_string()
         .expect("falha ao obter caminho no formato de string!")
      };
      // nome do diretório do caminho.
      let nome_pth = item.file_name().into_string().unwrap();

      // se for um diretório usar de recursividade.
      let espacamento = " ".repeat((*pfd) as usize);
      let pth = Path::new(pth_str.as_str());
      if pth.is_dir() {
          // molde de diretório(dois pontos).
          let mut str_aux = format!(
             "{1}{2}{3}{4} {0}:\n",
             nome_pth,espacamento,
             GALHO_VH,GALHO_H,GALHO_H
         );
         // ajusta a string na tela.
         ajusta_string(&mut str_aux, true);
         esboco.push_str(str_aux.as_str());
         let novo_path = Path::new(pth_str.as_str());
         (*pfd) += 3; // cada chamada recursiva, aumenta a profundidade.
         desenha_trilha_dirs(esboco, novo_path, pfd);
         (*pfd) -= 3; // "volta" um diretório...
      }
   }
}

/* acha o índice da array que representa 
 * abstratamente o galho das ramificações.
 * Caso não haja galho em tal array, o 'Option'
 * com o valor do índice será 'None'. */
fn acha_galho_dobrado(linha:&Vec<char>) -> Option<usize> {
   for indice in 0..linha.len() {
      if linha[indice] == GALHO_VH 
         { return Some(indice); }
   }
   /* nenhum galho a modificar a aparência, então
    * "nada" a retornar. */
   return None;
}

fn preenchendo_galhos(arvore:&mut Vec<Vec<char>>) {
   // dimensão da matriz:
   let max_y = arvore.len();
   // variável mutável para posição móvel da última linha.
   let mut l1;
   
   for l in 0..(max_y-1) {
      // última linha da matriz:
      l1 = (max_y-1)-l;
      // coluna com galho de dobra:
      let c = match acha_galho_dobrado(&arvore[l1]) {
         Some(valor) => valor,
         None => { continue; }
      };

      /* subindo e trocando espaços até encontrar
       * outro galho de dobra.
       * proposições:   */
      let mut p1 = arvore[l1-1][c] != GALHO_VH; 
      let mut p2 = arvore[l1-1][c].is_whitespace();
      let mut p3 = !(
         arvore[l1-1][c+1].is_ascii_alphanumeric()
         || arvore[l1-1][c+1] == '.' 
         || arvore[l1-1][c+1] == '_'
      );

      while p1 && p2 && p3 {
         // troca vácuo por galho vertical.
         arvore[l1-1][c] = GALHO_V;

         l1 -= 1; 
         // atualizando premissas
         p1 = arvore[l1-1][c] != GALHO_VH; 
         p2 = arvore[l1-1][c].is_whitespace();
         p3 = !(arvore[l1-1][c+1].is_ascii_alphanumeric()
               || arvore[l1-1][c+1] == '.');
      }
   }
}

fn troca_galhos_adequadamente(arvore:&mut Vec<Vec<char>>) {
   // dimensão da matriz:
   let max_y = arvore.len();
   let max_x = arvore[0].len();
   
   for l in 0..max_y-1 {
      for c in 0..max_x-1 {
         /* se o galho for dobrado(formato de L) 
          * e, o caractére abaixo for não branco
          * ou alfanumérico, ou seja, não altera
          * os galhos dobrados de ponta. */
         let condicao = {
            arvore[l][c] == GALHO_VH &&
            !(arvore[l+1][c].is_whitespace() ||
            arvore[l+1][c].is_ascii_alphanumeric())
         };
         // seguida a condição... faz troca.
         if condicao 
            { arvore[l][c] = GALHO_VHV; }
      }
   }
}

fn preenche_primeira_coluna(arvore:&mut Vec<Vec<char>>) {
   let inicio = 1;  
   let mut fim = 1;

   // achando última dobradiça.
   for l in inicio..arvore.len() {
      if arvore[l][0] == GALHO_VH {
         if l > fim 
            { fim = l; }
      }
   }

   // mudando... baeado no intervalo confiável filtrado.
   for l in inicio..fim {
      if arvore[l][0] == GALHO_VH {
         arvore[l][0] = GALHO_VHV;
      }
      else if arvore[l][0].is_whitespace() {
         arvore[l][0] = GALHO_V;
      }
   }
}

/** 
 retorna string representado tudo dentro de 
 um dado diretório, ramificando-os em arquivos
 e sub-diretórios. 
*/
pub fn arvore(caminho:&str, mostra_arquivos:bool) -> String {
   // string para concatenar strings representado trilha.
   let mut trilha = String::new();
   // obtendo o nome do diretório raíz.
   let caminho_i = Path::new(caminho);
   let raiz_nome:&str = {
      caminho_i
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
   };

   // colocando raíz no começo...
   trilha.push_str(raiz_nome);
   trilha.push_str(":\n");

   // espaçar cada vez mais, em cada novo sub-diretório.
   let mut profundidade:u8 = 0;
   // raiz, de onde parte a trilhagem...
   let raiz = Path::new(caminho);
   // se estiver configurado para mostrar arquivos...
   if mostra_arquivos 
      { desenha_trilha(&mut trilha, raiz, &mut profundidade); }
   else 
      { desenha_trilha_dirs(&mut trilha, raiz, &mut profundidade); }

   // fazendo ajustes...
   let mut matriz_arv = matriciar_string(trilha.clone());
   preenchendo_galhos(&mut matriz_arv); 
   troca_galhos_adequadamente(&mut matriz_arv);
   preenche_primeira_coluna(&mut matriz_arv);

   // retorna string representando trilha.
   return matriz_para_string(&matriz_arv);
}


// ----------- testando funções --------

#[cfg(test)]
mod tests {
   // biblioteca padrão do Rust.
   use std::path::Path;
   use std::env::var;
   //use constroi_simbolos::imprime;
   use super::*;
   
   // para testes.
   fn imprime(matriz:Vec<Vec<char>>) {
       // pega a linha da matriz.
       for row in matriz {
           // coluna na linha.
           for cell in row { print!("{}", cell); }
           print!("\n");
       }
   }
   
   #[test]
   fn lista_subdirs_de_foto() {
      let caminho = concat!(env!("HOME"), "/Pictures");
      let pth = Path::new(caminho);
      let mut trilha = String::from("");
      let mut p = 0;
      desenha_trilha_dirs(&mut trilha,pth, &mut p);
      println!("caminho:\n{}",trilha);
      assert!(true);
   }

   #[test]
   fn ramifica_ambos_modos() {
      let pth = concat!(env!("HOME"), "/Pictures");
      let primeiro = arvore(pth,true);
      let segundo = arvore(pth, false);

      println!("resultado(1):\n{}\n\nresultado(2):\n{}",primeiro,segundo);
      assert!(true);
   }

   #[test]
   fn coloca_galhos() {
      let caminho = concat!(env!("HOME"), "/Documents/códigos_rust");
      let arv = arvore(caminho,false);
      let mut matriz_arv = matriciar_string(arv);
      preenchendo_galhos(&mut matriz_arv);
      imprime(matriz_arv);
      assert!(true);
   }

   #[test]
   fn conserta_galhos_desajustados() {
      let caminho = var("HOME").unwrap() + "/Documents/códigos_rust";
      let arv = arvore(caminho.as_str(),true);
      let mut matriz_arv = matriciar_string(arv);
      preenchendo_galhos(&mut matriz_arv);
      troca_galhos_adequadamente(&mut matriz_arv);
      imprime(matriz_arv);
      assert!(true);
   }

   #[test]
   fn testando_arvore_implementacao_terminada() {
      let nucleo = match var("HOME") {
         Ok(s) => s,
         Err(_) => { panic!("não existe tal variável!"); },
      };
      let caminho = nucleo.clone() + "/Videos";
      let arv1 = arvore(caminho.as_str(), true);
      let caminho = nucleo.clone() + "/Documents/códigos";
      let arv2 = arvore(caminho.as_str(),true);

      println!("{}\n\n{}\n",arv1, arv2);

      assert!(true);
   }
}
