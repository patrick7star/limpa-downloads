
/*!
  Criando toda uma estrutura de dados para 
  desenhar formas de texto, ou texto numa 
  tela de terminal. Ela contém tanto o modo 
  de impressão, como uma formatação para `string`,
  então o jeito de visualizar-lá ficará à cargo
  do desenvolvedor implementar.
 */

// própria lib.:
use crate::terminal_dimensao::{Largura, Altura, dimensao};


/** constante com equivalente dos caractéres na tela
 dado uma "polegada"; neste caso a largura da 
 "polegada", que equivale aqui à 7 caractéres.*/
pub const POLEGADA_H:u8 = 7; // colunas
/** aqui a mesma equivalência entre "polegada" e caractéres,
 porém representando a altura de tal; neste caso uma 
 "polegada" equivale a quatro linhas-caractéres. */
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

    // seus "pixels", como eles são caractéres
    // no geral, será uma multiarray destes.
    tela:Vec<Vec<char>>
}

// implementando seus métodos:
impl Tela {
   /// cria a instância da `Tela`; com algumas
   /// configurações ativadas/desativadas como: 
   /// se haverá uma grade(meio pixelada) com 
   /// pontos, ou também, uma borda delimitando
   /// até onde pode-se escrever no objeto.
   pub fn cria(grade:bool, borda:bool) -> Tela {
      // dimensão da tela.
      let mensagem = "não foi possível obter a dimensão da tela!";
      let (col, lin):(u16, u16) = {
         match dimensao() {
            Some((Largura(l), Altura(h))) => (l, h),
            None => { panic!("{}", mensagem); }
         }
      };

      // cria a tela.
      let mut matriz:Vec<Vec<char>> = Vec::new();
      for _p in 1..(lin){
         if grade { matriz.push(vec!['.';col as usize])}
         else { matriz.push(vec![' '; col as usize])}
      }

      // retornando instância...
      let mut objeto = Tela {
         linhas: lin as u8,
         colunas: col as u8,
         tela: matriz,
      };

      // se for exigido, desenhar uma borda na tela. 
      if borda { circunscreve_borda(&mut objeto);}
      return objeto;
   }

   /// escreve uma string à partir de um dado `Ponto`.
   pub fn escreve(&mut self, string:&str, coord:Ponto) {
      let mut i:usize = 0;
      for caracter in string.chars() {
         let (l,c):(usize, usize) = (coord.linha as usize, 
                                    coord.coluna as usize);
         self.tela[l][c+i] = caracter;
         i += 1;
      }
   }

   /** Escreve múltiplas strings(múltiplas quer dizer cinco)
      tudo à partir de uma dada coordenada(`Ponto`). 
         O tanto de strings é determinado em tempo de 
      compilação por uma constante, então pode ser
      alterada para "escrever" mais que está quantia.*/
   pub fn escreve_strs(&mut self, strings:[&str;5], 
                       coord:Ponto) {
      let mut i: usize = 0;
      for s in strings {
         let nova_coord = Ponto{linha:coord.linha+(i as u8), 
                                coluna:coord.coluna}; 
         Tela::escreve(self, s, nova_coord);
         i += 1;
      }
   }

   /// imprime, usando diretamente o sistema
   /// de output padrão do sistema.
   pub fn imprime(&self) 
      { imprime_matriz(self.tela.clone()); }
   
   /// retorna da impressão de tela na forma de string.
   pub fn para_string(&self) -> String {
      // string auxiliar para concatenação
      // da tela em forma de texto.
      let mut tela_str: String = String::new();

      for linha in &self.tela {
         // adiciona caractére na linha.
         for c in linha { tela_str.push(*c);}
         // adiciona quebra-de-linha no texto.
         tela_str.push('\n');
      }

      tela_str // retorna o objeto criado.
   }

   /** Faz um risco na tela de um certo `Ponto`, e 
      também a `Direcao` cedida; com um dado comprimento. */
   pub fn risca(&mut self, coord:Ponto, compr:u8, 
                  simbolo:char, dir:Direcao) {
      // proposições:
      // risco cabe inteiramente dentro da tela.
      let risco_esta_dentro:bool = match dir {
          Direcao::Horizontal => 
            (((coord.coluna as u8)+compr) <= self.colunas as u8),
          Direcao::Vertical => 
            ((coord.linha as u8)+compr <= self.linhas as u8),
          Direcao::Diagonal(_) => {
            let p1 = (coord.linha as u8)+compr <= self.linhas as u8;
            let p2 = (coord.coluna as u8)+compr <= self.colunas as u8;
            p2 && p1
          },
      };
      // verifica se ponto está dentro da tela.
      let esta_na_tela:bool = coord.linha <= self.linhas &&
                              coord.coluna <= self.colunas;

      // se as duas opções vingarem, então...
      if risco_esta_dentro && esta_na_tela {
         // de onde partir a coordenada, e quanto ir...
         let fim = (coord.linha as usize) + (compr as usize) as usize; 
         let inicio = coord.linha as usize;

         for x in inicio..fim {
            let l:usize; let c:usize;
            // baseado na direção, escrever o símbolo.
            match dir {
               Direcao::Horizontal => {
                  l = coord.linha as usize;
                  c = (coord.coluna as usize)+x
               },
               Direcao::Vertical => {
                  l = (coord.linha as usize)+x;
                  c = (coord.coluna as usize)+x
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
            // colocar caractére na posição computada. 
            self.tela[l][c] = simbolo;
         }
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

      // apelidando coordenadas com nomes mais legíveis:
      let (ay,ax):(usize,usize) = (ponto_a.linha.into(), 
                                  ponto_a.coluna.into());
      let (by,bx):(usize, usize) = (ponto_b.linha.into(), 
                                 ponto_b.coluna.into());

      // cantos do perímetro(quadrados).
      self.tela[ay][ax] = CSE;
      self.tela[by][ax] = CIE;
      self.tela[by][bx] = CID;
      self.tela[ay][bx] = CSD; 

      // laterais do quadrilatero.
      for l in ay+1..by {
         self.tela[l][ax] = BV;
         self.tela[l][bx] = BV;
      }

      // bases do quadrilatero.
      for c in ax+1..bx {
         self.tela[ay][c] = BH;
         self.tela[by][c] = BH;
      }
   }

   /// moldura um retângulo dado o ponto e as dimensões.
   pub fn moldura(&mut self, ponto:Ponto, largura:u8, altura:u8) {
      // dá uma borda para o conteúdo interior.
      let ponto_a = Ponto{linha:ponto.linha-1, 
                          coluna:ponto.coluna-1};
      let ponto_b = Ponto{linha:ponto.linha-1+altura+1, 
                          coluna:ponto.coluna+largura+1};
      // usa função que já faz isso, porém para pontos,
      // com os pontos criados, é só preciso chamar-lá.
      self.circunscreve(ponto_a, ponto_b);
   }
}


fn imprime_matriz(matriz:Vec<Vec<char>>) {
   /* imprime matriz; tal é uma multiarray de 'char'. */
   // pecorre cada linha da matriz.
   for linha in &matriz {
       // pecorre cada coluna agora, imprime sem quebra-de-linha.
       for item in linha { print!("{}",item); }
       println!(""); // apenas quebra de linha.
   }
}


fn circunscreve_borda(estrutura:&mut Tela) {
   let (qtd_c,qtd_l):(usize, usize) = (estrutura.colunas as usize,
                              estrutura.linhas as usize);
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


// -------- teste da implementação -----------
#[cfg(test)]
mod tests {
   
   // importando tudo do módulo acima...
   #[test]
   #[ignore]
   fn teste_basico() {
       let  mut monitor:super::Tela = super::Tela::cria(true, false);

       monitor.escreve("hoje é um dia!", super::Ponto{linha:5, coluna:10});
       let lin = 3*super::POLEGADA_V;
       let col = (super::POLEGADA_H as f32/2.0) as u8;
       monitor.escreve("uma frase simples!", super::Ponto{linha:lin,
                                                         coluna:col});
       monitor.imprime();
    }

   #[test]
   #[ignore]
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

      println!("\n\nfeito por impressão padrão:\n{}", monitori.para_string());
   }

   #[test]
   #[ignore]
   fn risca_tela_todas_direcoes() {
      let mut t = super::Tela::cria(false, true);

      let ponto = super::Ponto{linha:5, coluna:15};
      t.risca(ponto, 39,'*', super::Direcao::Horizontal);
      let ponto = super::Ponto{linha:2, coluna:13};
      t.risca(ponto, 17,'#', super::Direcao::Vertical);

      println!("tela rabiscada:\n{}", t.para_string());
   }

   #[test]
   #[ignore]
   fn risca_nao_valido() {
     let mut t = super::Tela::cria(false, true);

     let ponto = super::Ponto{linha:10, coluna:30};
     t.risca(ponto, 30, '&', super::Direcao::Vertical);
     println!("{}", t.para_string());
     assert!(true);
   }

   #[test]
   #[ignore]
   fn ponto_invalido() {
     let mut t = super::Tela::cria(false, true);
     let ponto = super::Ponto{linha:10, coluna:200};
     t.risca(ponto, 13, '&', super::Direcao::Diagonal(super::TipoD::Principal));
     println!("{}", t.para_string());
     assert!(true);
   }

   #[test]
   #[ignore]
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

      println!("{}", t.para_string());
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
      //nova_tela.circunscreve(super::Ponto{linha:6, coluna:29},
      //                   super::Ponto{linha:12, coluna:40});
      nova_tela.moldura(super::Ponto{linha:7, coluna:30}, 10, 5);
      // para esquiparar no debug.
      nova_tela.escreve_strs(["salamandra","côco","barba azul",
                            "abajú","clarice"], 
                            super::Ponto{linha:7, coluna:50});
      println!("{}", nova_tela.para_string());
   }
}
