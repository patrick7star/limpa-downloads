
use super::tabelacao::{RECUO, BARRA};
use super::matriz_texto::{MT};
use std::iter::Iterator;

const ESPACO: char = '>';
/* caractéres especiais: */
// cantos:
static CANTO_SE:char = '\u{256d}';
static CANTO_SD:char = '\u{256e}';
static CANTO_IE:char = '\u{2570}';
static CANTO_ID:char = '\u{256f}';
// lateral e horizontais:
static LATERAL:char = '\u{2502}';
static TRACO:char = '\u{2500}';
// conectores laterais, pontas e do meio.
static LATERAL_CD:char = '\u{2524}'; 
static LATERAL_CE:char = '\u{251c}';
static CRUZ:char = '\u{253c}';
static TRACO_I:char = '\u{2534}';
static TRACO_S:char = '\u{252c}';


/* tarefa de revestimento. */
pub fn reveste(tabela_str: String) -> String{
   let referencia: &str = tabela_str.as_str();
   let mut matriz = MT::to_matriz(referencia);
   // primeiro remove margem por espaço em branco.
   remove_margens(&mut matriz);
   // trocas as barras pelas "laterais" e "traços".
   substitui_barras_retas(&mut matriz);
   // trocas cantos das tabelas.
   substitui_cantos(&mut matriz);
   // faz coneções entre "laterals" e "traços".
   coloca_conectores(&mut matriz);
   // retorna tabela string revestida.
   return matriz.to_string();
}

// acha margem especial de debug.
fn acha_margem(matriz: &mut MT) -> Option<Vec<u16>> {
   // posições onde margens começam.
   let mut posicoes: Vec<u16> = Vec::new();
   let (_, largura) = matriz.dimensao();

   // pecorrer todo topo/cabeçalho por algo.
   'geral: for x in 0..largura {
      // se achou algo, análise mais profunda.
      if matriz.get(1, x) == ESPACO {
         let x2 = x + RECUO as u16;
         for x1 in x..x2 {
            // se um estiver fora, então todos estão ...
            if matriz.get(1, x1) != ESPACO
               { continue 'geral; }
         }
         // se chegar até aqui é a posição.
         posicoes.push(x);
      }
   }

   if posicoes.len() > 0  
      { return Some(posicoes); }
   else 
      { return None; }
}

// profundidade da possível margem.
fn qual_profundidade(matriz: &mut MT, x: u16) -> Option<u16> {
   let mut h: Option<u16> = None;
   let (altura, _) = matriz.dimensao();
   for y in 1..altura {
      let c = matriz.get(y, x);
      if c == ESPACO
         { h = Some(y); }
   }
   return h;
}

/* remove margens entre as frações da tabela
 * se houver alguma claro. */
fn remove_margens(matriz: &mut MT) { 
   /* acha todos 'x(posição na matriz)' com 
    * margens na tabela. */
   match acha_margem(matriz) {
      Some(mut posicoes) => {
         for x in posicoes.drain(0..) {
            match qual_profundidade(matriz, x) {
               Some(p) => {
                  for y in 0..=p+1 { 
                     let x2 = RECUO as u16 + x;
                     for x1 in x..x2
                        { matriz.set(y, x1, ' '); }
                  }
               } None => () 
            }
         }
      } None =>
         { println!("sem margem!"); }
   };
}

fn substitui_barras_retas(matriz: &mut MT) {
   let (altura, largura) = matriz.dimensao();
   /* verifica se os caractéres passados
    * são uma barra, todos eles. */
   let sao_barra = {
      |array: Vec<char>| 
      array.iter()
      .all(|c| *c == BARRA) 
   };
   // lista de posições para alteração posterior.
   type Coord = (u16, u16, char);
   let mut lista: Vec<Coord> = Vec::new();

   for y in 0..altura {
      for x in 1..largura-1 {
         // é uma barra horizontal.
         let e_barra_horizontal: bool = {
            sao_barra(vec![
               matriz.get(y, x),
               matriz.get(y, x+1),
               matriz.get(y, x-1)
            ])
         };
         if e_barra_horizontal
            { lista.push((y, x, TRACO)); }
      }
   }
   for y in 1..altura-1 {
      for x in 0..largura {
         // é uma barra horizontal.
         let e_barra_vertical: bool = {
            sao_barra(vec![
               matriz.get(y, x),
               matriz.get(y-1, x),
               matriz.get(y+1, x)
            ])
         };
         if e_barra_vertical 
            { lista.push((y, x, LATERAL)); }
      }
   }
   // alteração em sí.
   for tupla in lista.drain(0..) 
      { matriz.set(tupla.0, tupla.1, tupla.2); }
}

fn substitui_cantos(matriz: &mut MT) {
   let (altura, largura) = matriz.dimensao();
   // últimas posições verticais e horizontais.
   let (h, l) = (altura-1, largura-1);

   for y in 0..altura {
      for x in 0..largura {
         let canto_superior_esquerdo: bool = {
            /* só faz a operação se garantido a 
             * restrição, caso contrário não é 
             * o que está se procurando. */
            if y <= h-1 && x <= l-1 {
               matriz.get(y, x) == BARRA &&
               matriz.get(y+1, x) == LATERAL &&
               matriz.get(y, x+1) == TRACO
            } else { false }
         };
         let canto_superior_direito: bool = {
            /* só faz a operação se garantido a 
             * restrição, caso contrário não é 
             * o que está se procurando. */
            if y <= h-1 && x >= 1 {
               matriz.get(y, x) == BARRA &&
               matriz.get(y, x-1) == TRACO &&
               matriz.get(y+1, x) == LATERAL
            } else { false }
         };
         let canto_inferior_direito: bool = {
            /* só faz a operação se garantido a 
             * restrição, caso contrário não é 
             * o que está se procurando. */
            if y >= 1 && x >= 1 {
               matriz.get(y, x) == BARRA &&
               matriz.get(y, x-1) == TRACO &&
               matriz.get(y-1, x) == LATERAL
            } else { false }
         };
         let canto_inferior_esquerdo: bool = {
            /* só faz a operação se garantido a 
             * restrição, caso contrário não é 
             * o que está se procurando. */
            if y >= 1 && x <= l-1 {
               matriz.get(y, x) == BARRA &&
               matriz.get(y, x+1) == TRACO &&
               matriz.get(y-1, x) == LATERAL
            } else { false }
         };
         /* todos casos são exclusivos, então 
          * acionado um, o outro é inválido. */
         if canto_superior_direito 
            { matriz.set(y, x, CANTO_SD); }
         else if canto_superior_esquerdo 
            { matriz.set(y, x, CANTO_SE); }
         else if canto_inferior_direito
            { matriz.set(y, x, CANTO_ID); }
         else if canto_inferior_esquerdo
            { matriz.set(y, x, CANTO_IE); }
      }
   }
}

fn coloca_conectores(matriz: &mut MT) {
   let (altura, largura) = matriz.dimensao();
   // últimas posições verticais e horizontais.
   let (h, l) = (altura-1, largura-1);

   for y in 0..altura {
      for x in 0..largura {
         let e_um_te_esquerdo: bool = {
            /* só faz a operação se garantido a 
             * restrição, caso contrário não é 
             * o que está se procurando. */
            if (y >= 1 && y <= h-2) && x <= l-1 {
               matriz.get(y, x) == LATERAL &&
               matriz.get(y+1, x) == LATERAL &&
               matriz.get(y-1, x) == LATERAL &&
               matriz.get(y, x+1) == TRACO
            } else { false }
         };
         let e_um_te_direito: bool = {
            /* só faz a operação se garantido a 
             * restrição, caso contrário não é 
             * o que está se procurando. */
            if (y >= 1 && y <= h-2) && x >= 1 {
               matriz.get(y, x) == LATERAL &&
               matriz.get(y+1, x) == LATERAL &&
               matriz.get(y-1, x) == LATERAL &&
               matriz.get(y, x-1) == TRACO
            } else { false }
         };
         let e_um_te_normal: bool = {
            /* só faz a operação se garantido a 
             * restrição, caso contrário não é 
             * o que está se procurando. */
            if y <= h-1 && (x >= 1 && x <= l-1) { 
               matriz.get(y, x) == TRACO &&
               matriz.get(y+1, x) == LATERAL &&
               matriz.get(y, x+1) == TRACO &&
               matriz.get(y, x-1) == TRACO
            } else { false }
         };
         let e_um_te_invertido: bool = {
            /* só faz a operação se garantido a 
             * restrição, caso contrário não é 
             * o que está se procurando. */
            if y >= 1 && (x <=l-1 && x >= 1) {
               matriz.get(y, x) == TRACO &&
               matriz.get(y-1, x) == LATERAL &&
               matriz.get(y, x+1) == TRACO &&
               matriz.get(y, x-1) == TRACO
            } else { false }
         };
         let e_uma_cruz: bool = {
            if (x >= 1 && x <= l-1) && (y >= 1 && y <= h-1) {
               matriz.get(y, x) == LATERAL &&
               matriz.get(y+1, x) == LATERAL &&
               matriz.get(y-1, x) == LATERAL &&
               matriz.get(y, x+1) == TRACO &&
               matriz.get(y, x-1) == TRACO
            } else { false }
         };

         if e_um_te_esquerdo 
            { matriz.set(y, x, LATERAL_CE); }
         else if e_um_te_direito
            { matriz.set(y, x, LATERAL_CD); }
         else if e_um_te_normal
            { matriz.set(y, x, TRACO_S); }
         else if e_um_te_invertido 
            { matriz.set(y, x, TRACO_I); }
         if e_uma_cruz
            { matriz.set(y, x, CRUZ); }
      }
   }
}
