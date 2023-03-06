
/*!
  # Tela para desenho
  Criando toda uma estrutura de dados para 
  desenhar formas de texto, ou texto numa 
  tela de terminal. Ela contém tanto o modo 
  de impressão, como uma formatação para `string`,
  então o jeito de visualizar-lá ficará à cargo
  do desenvolvedor implementar.
*/

// própria lib.:
use crate::terminal_dimensao::{Largura, Altura, dimensao};
// bibloteca padrão do Rust:
use std::string::ToString;
use super::tela_auxiliar::{Mudanca, Pilha};

/** constante com equivalente dos caractéres 
 na tela dado uma "polegada"; neste caso a 
 largura da "polegada", que equivale aqui 
 à 7 caractéres.*/
pub const POLEGADA_H:u8 = 7; // colunas

/** aqui a mesma equivalência entre "polegada" 
 e caractéres, porém representando a altura de 
 tal; neste caso uma "polegada" equivale a quatro 
 linhas-caractéres. */
pub const POLEGADA_V:u8 = 4; // linhas


/** baseado no enum `Direcao` que mostra a direção 
 do risco este no caso, re-direciona a vertical 
 onde será riscado. */
#[derive(Copy, Clone)]
pub enum TipoD {
   Principal,
   Secundaria,
}

/** indica como será o risco, a escrita de string e 
 etc dos métodos de inscrição da estrutura `Tela`. */
#[derive(Copy, Clone)]
pub enum Direcao {
   Horizontal,
   Vertical,
   Diagonal(TipoD),
}

/** estrutura de coordenada para indicar onde na tela.
 será o começo do risco; das strings impressas; 
 o enquadramento e etc... */
#[derive(Copy, Clone)]
pub struct Ponto { linha:u8, coluna:u8 }

/** Estrutura de dados representando a tela em sí. Aqui
 será inscritos todas as coisas, que gerará uma string
 no formato atual do terminal onde será impresso.
 Têm vários métodos demonstrando como inscrever, e o que 
 inscrever nela. */
pub struct Tela {
    // dimensão da tela de terminal.
    linhas:u8, colunas:u8,
    /* seus "pixels", como eles são caractéres
     * no geral, será uma multiarray destes. */
    tela: Vec<Vec<char>>,
    /* Uma pilha contendo todas escritas feitas 
     * na tela, baseado nas escritas pelos métodos
     * abaixo, assim também terar um método de 
     * desfazer e refazer últimas mudanças. */
    pilha_alteracoes: Pilha<Mudanca>
}

// implementando seus métodos:
impl Tela {
   /** cria a instância da `Tela`; com algumas
    configurações ativadas/desativadas como: 
    se haverá uma grade(meio pixelada) com 
    pontos, ou também, uma borda delimitando
    até onde pode-se escrever no objeto. */
   pub fn cria(grade:bool, borda:bool) -> Tela {
      // dimensão da tela.
      let mensagem = "não foi possível obter a dimensão da tela!";
      let (col, lin):(u8, u8) = {
         match dimensao() {
            Some((Largura(l), Altura(h))) => 
               { (l as u8, h as u8) },
            None => { panic!("{}", mensagem); }
         }
      };

      // cria a tela.
      let mut matriz:Vec<Vec<char>> = Vec::new();
      for _ in 1..(lin){
         if grade 
            { matriz.push(vec!['.';col as usize])}
         else 
            { matriz.push(vec![' '; col as usize])}
      }

      // retornando instância...
      let mut objeto = Tela {
         linhas: lin as u8,
         colunas: col as u8,
         tela: matriz,
         pilha_alteracoes: Pilha::nova()
      };

      // se for exigido, desenhar uma borda na tela. 
      if borda { circunscreve_borda(&mut objeto);}
      return objeto;
   }

   /// escreve uma string à partir de um dado `Ponto`.
   pub fn escreve(&mut self, string:&str, coord:Ponto) {
      let mut alteracao = Mudanca::cria_vazio();
      // percorrendo caractéres da string.
      for (i,caracter) in string.chars().enumerate() {
         let (l,c):(usize, usize) = (
            coord.linha as usize, 
            coord.coluna as usize
         );
         // registrando alteração que será feita.
         let coluna = (c + i) as u8;
         let linha = l as u8;
         let p = Ponto { linha, coluna };
         let ch = self.tela[l][c + i];
         alteracao.incrementa((p, ch));
         // alterando tela agora ...
         self.tela[l][c + i] = caracter;
      }
      // salva alteração feita.
      self.pilha_alteracoes.empilha(alteracao).unwrap();
   }

   /** Escreve múltiplas strings(múltiplas quer dizer cinco)
    tudo à partir de uma dada coordenada(`Ponto`). 
      O tanto de strings é determinado em tempo de 
   compilação por uma constante, então pode ser
   alterada para "escrever" mais que está quantia.*/
   pub fn escreve_strs(&mut self, strings:[&str; 5], coord:Ponto) {
      for (l, s) in strings.iter().enumerate() {
         let coord = Ponto{
            linha: coord.linha + (l as u8), 
            coluna: coord.coluna
         }; 
         Tela::escreve(self, s, coord);
      }
      /* mesclando todas as 'alterações' das palavras
       * que foram registradas em uma só. */
      let mut alteracao = {
         self.pilha_alteracoes
         .desempilha()
         .unwrap()
      };
      for _ in 1..=4 {
         let mut a = self.pilha_alteracoes.desempilha().unwrap();
         for pixel in a.pontilhados_escritos.drain(..) 
            { alteracao.incrementa(pixel); }
      }
      self.pilha_alteracoes.empilha(alteracao).unwrap();
   }

   /** Faz um risco na tela de um certo `Ponto`, e 
   também a `Direcao` cedida; com um dado comprimento. */
   pub fn risca(&mut self, coord:Ponto, compr:u8, 
   simbolo:char, dir:Direcao) {
      // proposições:
      // risco cabe inteiramente dentro da tela.
      let risco_esta_dentro:bool = match dir {
          Direcao::Horizontal => {
            let c1:u8 = (coord.coluna as u8) + compr;
            let c2:u8 = self.colunas;
            c1 <= c2
          },
          Direcao::Vertical => {
            let t1 = coord.linha + compr;
            let t2 = self.linhas;
            t1 <= t2
          },
          Direcao::Diagonal(_) => {
            let p1 = (coord.linha + compr) <= self.linhas;
            let p2 = (coord.coluna + compr) <= self.colunas;
            p2 && p1
          },
      };
      // verifica se ponto está dentro da tela.
      let esta_na_tela:bool = {
         coord.linha <= self.linhas &&
         coord.coluna <= self.colunas
      };

      // se as duas opções vingarem, então...
      if risco_esta_dentro && esta_na_tela {
         // de onde partir a coordenada, e quanto ir...
         let fim:usize = {
            (coord.linha as usize) + 
            (compr as usize) as usize
         }; 
         let inicio:usize = coord.linha as usize;

         // alterações realizadas.
         let mut alteracao:Mudanca = Mudanca::cria_vazio();
         for x in inicio..fim {
            let l:usize; let c:usize;
            // baseado na direção, escrever o símbolo.
            match dir {
               Direcao::Horizontal => {
                  l = coord.linha as usize;
                  c = (coord.coluna as usize) + x
               },
               Direcao::Vertical => {
                  l = (coord.linha as usize) + x;
                  c = (coord.coluna as usize) + x
               },
               Direcao::Diagonal(t) => match t {
                  // abordando dois tipos de diagonais...
                  // direita para esquerda(para baixo).
                  TipoD::Principal => {
                     l = coord.linha as usize + x;
                     c = coord.coluna as usize + x;
                  },
                  // esquerda para direita(prá baixo).
                  TipoD::Secundaria => {
                     l = coord.linha as usize + x;
                     c = coord.coluna as usize - x;
                  }
               },
            };
            // registrando posição e estado.
            let p = Ponto { linha: l as u8, coluna: c as u8 };
            let s = self.tela[l][c];
            alteracao.incrementa((p, s));
            // colocar caractére na posição computada. 
            self.tela[l][c] = simbolo;
         }
         // salva mudança, empilhando-a.
         self.pilha_alteracoes.empilha(alteracao).unwrap();
      }
   }

   /// circunscreve uma área, dado dois pontos.
   pub fn circunscreve(&mut self, ponto_a:Ponto, ponto_b:Ponto) {
      // colocando cantos:
      const CSE:char = '\u{250c}';
      const CID:char = '\u{2518}';
      const CIE:char = '\u{2514}';
      const CSD:char = '\u{2510}';
      const BV:char = '\u{2502}';
      const BH:char = '\u{2500}';

      // registro da alteração total.
      let mut alteracao = Mudanca::cria_vazio();
      // apelidando coordenadas com nomes mais legíveis:
      let (ay, ax, by, bx):(usize, usize, usize, usize) = (
         ponto_a.linha.into(), 
         ponto_a.coluna.into(),
         ponto_b.linha.into(), 
         ponto_b.coluna.into()
      );
      /* registrando cantos do perímetro e
       * registrando isso na 'alteração'. */
      let sequencias = [
         (ay, ax, CSE), 
         (by, ax, CIE), 
         (by, bx, CID), 
         (ay, bx, CSD)
      ];
      for tupla in sequencias {
         // o que está antes de sobre-escrever.
         let ea = self.tela[tupla.0][tupla.1];
         // cantos do perímetro(quadrados).
         self.tela[tupla.0][tupla.1] = tupla.2;
         alteracao.incrementa(
            (Ponto {
               linha: tupla.0 as u8, 
               coluna:tupla.1 as u8
            },
            ea)
         );
      }

      // laterais do quadrilatero.
      for l in (ay + 1)..by {
         let antes = self.tela[l][ax];
         let ponto = Ponto {
            linha:l as u8,
            coluna: ax as u8
         };
         alteracao.incrementa((ponto, antes));
         self.tela[l][ax] = BV;
         let antes = self.tela[l][bx];
         let ponto = Ponto {
            linha:l as u8,
            coluna: bx as u8
         };
         alteracao.incrementa((ponto, antes));
         self.tela[l][bx] = BV;
      }

      // bases do quadrilatero.
      for c in (ax + 1)..bx {
         let antes = self.tela[ay][c];
         let ponto = Ponto {
            linha: ay as u8,
            coluna: c as u8
         };
         alteracao.incrementa((ponto, antes));
         self.tela[ay][c] = BH;

         let antes = self.tela[by][c];
         let ponto = Ponto {
            linha:by as u8,
            coluna: c as u8
         };
         alteracao.incrementa((ponto, antes));
         self.tela[by][c] = BH;
      }
      // salva toda alteração, empilhando-a.
      self.pilha_alteracoes.empilha(alteracao).unwrap();
   }

   /// moldura um retângulo dado o ponto e as dimensões.
   pub fn moldura(&mut self, ponto:Ponto, largura:u8, altura:u8) {
      // dá uma borda para o conteúdo interior.
      let ponto_a = Ponto{
         linha: ponto.linha - 1, 
         coluna: ponto.coluna - 1
      };
      let ponto_b = Ponto{
         linha: ponto.linha - 1 + altura + 1, 
         coluna: ponto.coluna + largura + 1
      };
      /* usa função que já faz isso, porém para pontos,
       * com os pontos criados, é só preciso chamar-lá.
       * Como a função que ele usa de auxílio já 
       * registra uma 'alteração', outra não é preciso
       * nem algo como mesclar, pois apenas faz uma 
       * só 'alteração'. */
      self.circunscreve(ponto_a, ponto_b);
   }
   /// desfaz últimos riscos/escritas e mais coisas realizadas.
   pub fn desfazer(&mut self) {
      // tira do topo da pilha, que é ponto alterado ...
      if let Some(md) = self.pilha_alteracoes.desempilha() {
         // trabalhando com cada ponto individual.
         let lista = md.pontilhados_escritos.iter();
         for (Ponto{linha:l, coluna:c}, pixel) in lista {
            // pega coordenadas.
            let x = *c as usize;
            let y = *l as usize;
            // faz lacuna branca novamente.
            self.tela[y][x] = *pixel;
         }
      }
   }
}

/* Pega a `Tela` e escreve uma borda em torno
 * dela. */
fn circunscreve_borda(estrutura:&mut Tela) {
   let (qtd_c,qtd_l):(usize, usize) = (
      estrutura.colunas as usize,
      estrutura.linhas as usize
   );
   // colocando cantos:
   estrutura.tela[0][0] = '\u{250c}';
   estrutura.tela[qtd_l-2][0] = '\u{2514}';
   estrutura.tela[qtd_l-2][qtd_c-1] = '\u{2518}';
   estrutura.tela[0][qtd_c-1] = '\u{2510}';

   // marca os lados:
   for x in 1..qtd_c-1 {
      estrutura.tela[0][x] = '\u{2500}';
      estrutura.tela[qtd_l-2][x] = '\u{2500}';
   }

   // desenha as parte horizontais paralelas.
   for y in 1..qtd_l-2 {
      estrutura.tela[y][0] = '\u{2502}';
      estrutura.tela[y][qtd_c-1]= '\u{2502}';
   }
}

impl ToString for Tela {
   // retorna da impressão de tela na forma de string.
   fn to_string(&self) -> String {
      // string auxiliar para concatenação
      // da tela em forma de texto.
      let mut tela_str: String = String::new();

      for linha in &self.tela {
         // adiciona caractére na linha.
         for c in linha { tela_str.push(*c);}
         // adiciona quebra-de-linha no texto.
         tela_str.push('\n');
      }

      // retorna o objeto criado.
      tela_str 
   }
}

// -------- teste da implementação -----------
#[cfg(test)]
mod tests {
   // importando todo conteúdo acima ...
   use super::*;
   // importando tudo do módulo acima...
   #[test]
   fn teste_basico() {
      let  mut monitor:super::Tela = super::Tela::cria(true, false);

      monitor.escreve(
         "hoje é um dia!", 
         super::Ponto{linha:5, coluna:10}
      );
      let lin = 3*super::POLEGADA_V;
      let col = (super::POLEGADA_H as f32/2.0) as u8;
      monitor.escreve(
         "uma frase simples!", 
         super::Ponto{linha:lin, coluna:col}
      );
       println!("{}",monitor.to_string());
    }

   #[test]
   fn escreve_varias_strings() {
      let mut monitori:super::Tela = super::Tela::cria(false, true);
      let ponto = super::Ponto{linha:3, coluna:10};
      monitori.escreve_strs(["eu","joguei","garrafas",
                           "neles","neste instante"],
                           ponto);
      let ponto = super::Ponto{linha:3, coluna:10};
      monitori.escreve_strs(["meu","nome","não", "é","Jonnhys"],
                              ponto);
      let ponto = super::Ponto{linha:3, coluna:10};
      monitori.escreve_strs(["salamandra","côco","barba azul",
                            "abajú","clarice"], ponto);
      let ponto = super::Ponto{linha:3, coluna:10};
      monitori.escreve_strs(["Meu","Nome","Não", "é","Jonnhys"],
                              ponto);

      println!("\n\nfeito por impressão padrão:\n{}", monitori.to_string());
   }

   #[test]
   fn risca_tela_todas_direcoes() {
      let mut t = super::Tela::cria(false, true);

      let ponto = super::Ponto{linha:5, coluna:15};
      t.risca(ponto, 39,'*', super::Direcao::Horizontal);
      let ponto = super::Ponto{linha:2, coluna:13};
      t.risca(ponto, 17,'#', super::Direcao::Vertical);

      println!("tela rabiscada:\n{}", t.to_string());
   }

   #[test]
   fn risca_nao_valido() {
      let mut t = super::Tela::cria(false, true);
      let ponto = super::Ponto{linha:10, coluna:30};
      t.risca(ponto, 30, '&', super::Direcao::Vertical);
      println!("{}", t.to_string());
      assert!(true);
   }

   #[test]
   fn ponto_invalido() {
      let mut t = super::Tela::cria(false, true);
      let ponto = super::Ponto{linha:10, coluna:200};
      t.risca(
         ponto, 13, '&', 
         super::Direcao::Diagonal(super::TipoD::Principal)
      );
      println!("{}", t.to_string());
      assert!(true);
   }

   #[test]
   fn testando_risco_vertical() {
      let mut t = super::Tela::cria(false, true);
      let ponto = super::Ponto{linha:5, coluna:20};
      let ponto2 = super::Ponto{linha:3, coluna:40};
      let tipo = super::TipoD::Principal;
      let outro_tipo = super::TipoD::Secundaria;
      let direcao = super::Direcao::Diagonal(tipo);
      let outra_direcao = super::Direcao::Diagonal(outro_tipo);

      t.risca(ponto, 13, '@', direcao);
      t.risca(ponto2, 14, 'X', outra_direcao);

      println!("{}", t.to_string());
      assert!(true);
   }

   #[test]
   fn moldura_texto() {
      let mut nova_tela = super::Tela::cria(false, true);
      let ponto = super::Ponto{linha:7, coluna:30};
      nova_tela.escreve_strs(["salamandra","côco","barba azul",
                            "abajú","clarice"], ponto);
      /* o que se pode detectar para um bom ajuste é...
      * um recúo para esquerda em relação a coluna, e,
      * outro para cima em relação a linha. */
      nova_tela.moldura(super::Ponto{linha:7, coluna:30}, 10, 5);
      // para esquiparar no debug.
      nova_tela.escreve_strs(["salamandra","côco","barba azul",
                            "abajú","clarice"], 
                            super::Ponto{linha:7, coluna:50});
      println!("{}", nova_tela.to_string());
   }

   #[test]
   fn testa_metodo_desfazer() {
      let mut nova_tela = Tela::cria(false, true);
      let ponto = Ponto{linha:7, coluna:30};
      let p1 = Ponto{linha:5, coluna:20};
      let p2 = Ponto{linha:3, coluna:40};
      let tipo = TipoD::Principal;
      let outro_tipo = TipoD::Secundaria;
      let direcao = Direcao::Diagonal(tipo);
      let outra_direcao = Direcao::Diagonal(outro_tipo);

      // escreve algumas strings ...
      nova_tela.escreve_strs(
         ["salamandra","côco",
         "barba azul", "abajú",
         "clarice"], 
         ponto
      );
      nova_tela.moldura(Ponto{linha:7, coluna:30}, 10, 5);
      nova_tela.escreve_strs(
         ["salamandra",
         "côco","barba azul",
         "abajú","clarice"], 
         Ponto{linha:7, coluna:50}
      );
      // faz riscos em forma de cruz ...
      nova_tela.risca(p1, 13, '@', direcao);
      nova_tela.risca(p2, 14, '=', outra_direcao);
      //
      nova_tela.circunscreve(
         Ponto{linha: 15, coluna:40},
         Ponto{linha: 20, coluna:59}
      );
      nova_tela.escreve(
         "isso aqui é algo histórico",
         Ponto{linha:1, coluna:10} 
      );
      println!("original:\n{}", nova_tela.to_string());
      // desfazendo cada uma das alterações ...
      nova_tela.desfazer();
      println!("1ª desfeita:\n{}", nova_tela.to_string());
      nova_tela.desfazer();
      println!("2ª desfeita:\n{}", nova_tela.to_string());
      nova_tela.desfazer();
      println!("3ª desfeita:\n{}", nova_tela.to_string());
      nova_tela.desfazer();
      println!("4ª desfeita:\n{}", nova_tela.to_string());
      nova_tela.desfazer();
      println!("5ª desfeita:\n{}", nova_tela.to_string());
      nova_tela.desfazer();
      println!("6ª desfeita:\n{}", nova_tela.to_string());
      nova_tela.desfazer();
      println!("7ª desfeita:\n{}", nova_tela.to_string());
      assert!(true);
   }
}
