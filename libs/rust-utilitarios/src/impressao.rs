/*!
 # Impressão na tela de vários tipos
  Imprime e cria uma string que cuida
 de, dado uma tupla com valores imprimiveis
 faz tal impressão na forma de escada.
*/

// biblioteca do Rust:
use std::fmt::{Formatter, Display, Result as Resultado};
use std::dbg;
use std::string::String;

// biblioteca externa:
use crate::terminal_dimensao::*;


/* reescrevendo o método do len da string para 
 * pegar acentuações conhecidas de dois bytes.
 */
trait StringExtensao {
   /* computa o tamanho de bytes entre strings
    * levando em conta caractéres de 2 bytes. */
   fn len(&self) -> usize;
}

// para slice-strings(stack-strings) `&str`.
impl StringExtensao for str {
   fn len(&self) -> usize {
      // conta a quantia de acentuações comuns.
      let mut qtd:usize = 0;
      for ch in self.chars() {
         if ch == 'á' { qtd += 1; }
         if ch == 'à' { qtd += 1; }
         if ch == 'â' { qtd += 1; }
         if ch == 'ã' { qtd += 1; }
         if ch == 'é' { qtd += 1; }
         if ch == 'ê' { qtd += 1; }
         if ch == 'í' { qtd += 1; }
         if ch == 'ô' { qtd += 1; }
         if ch == 'õ' { qtd += 1; }
         if ch == 'ó' { qtd += 1; }
         if ch == 'ú' { qtd += 1; }
         if ch == 'ç' { qtd += 1; }
      }
      let tamanho = self.len();
      return tamanho - qtd;
   }
}

// para heap-strings `String`.
impl StringExtensao for String {
   fn len(&self) -> usize 
      { self.as_str().len() }
}

/** "Imprime" toda a array com "slice-strings"
 na forma de escadinha, ou seja, cada uma têm
 um recuo até metade da string anterior.
*/
pub fn escadinha(entradas:Vec<&str>) -> String {
   // largura do terminal.
   let largura_tela = match terminal_largura() {
      Ok(Largura(lg)) => lg,
      Err(erro) => 
         { panic!("{}", erro); }
   };

   let mut nova_str:String = String::new();
   let mut acumulado:u16 = 0;
   let mut colidiu_na_parede:bool = false;
   let mut pilha_de_recuos:Vec<u16> = Vec::with_capacity(100);

   for s in entradas.iter() {
      let repeticao = " ".repeat(acumulado as usize);
      let tamanho = StringExtensao::len(*s);
      // adiciona "vácuo".
      nova_str.push_str(&repeticao[..]);
      // adiciona string.
      nova_str.push_str(s);
      // quebra de linha.
      nova_str.push('\n');
      //let meio_str:u16;
      if (tamanho as u16 + acumulado < largura_tela) &&
      !colidiu_na_parede {
         // cotabilizando espaço recuado.
         let meio_str:u16 = (tamanho / 2) as u16;
         acumulado += meio_str;
         /* adiciona aculuma, para quando bater na tela, 
          * o movimento de retrocesso comece. E tal, 
          * dará-se por tirar acumulu adicionado, ou seja
          * o primeiro a entrar será o último a sair,
          * uma pilha! */
         pilha_de_recuos.push(meio_str);
      }
      else {
         /* tira o acumulo colocado no topo da pilha,
          * e subtrai, criando como consequência um
          * movimento inverso. */
         if let Some(q) = pilha_de_recuos.pop() {
            acumulado -= q;
            // marca com colídiu com a parede direita, como certo.
            colidiu_na_parede = true;
         } else 
            { colidiu_na_parede = false }
      }
   }
    return nova_str;
}

/** imprime entradas de forma a preencher toda 
 a janela, onda a saida de texto está saindo. */
pub struct Impressao<'a> {
   // fila que contém um limite.
   fila:Vec<&'a str>,
   // quantidade e colunas a preencher.
   colunas:u8,
   // média de tamanho das strings inseridas.
   largura_media:u8,
   // maior comprimento de string medido.
   maior_comprimento:u8,
}

/// acrescimo para distânciar entradas.
const X:u8 = 3;

impl <'a>Impressao<'a> {
   // método construtor.
   pub fn cria(inicial:Vec<&'a str>) -> Self {
      // computando uma média inicial.
      let media:u8 = { 
         // somando tudo...
         let soma:usize = inicial.iter()
         .map(|s| s.len()).sum();
         // contabilizado o total de objetos.
         let total = inicial.len();
         // efetuando cálculo, e já convertendo...
         (soma / total) as u8
      };
      // computando o comprimento antigo.
      let comprimento:u8 = {
         inicial.iter()
         .map(|s| s.len())
         .max()
         .unwrap() as u8
      };
      // largura do terminal.
      let largura:u8 = match dimensao() {
         Some((Largura(l), Altura(_))) => dbg!(l) as u8,
         None => 0_u8
      };
      #[allow(unused_variables)]
      // instânciando objeto...
      Impressao {
         fila: inicial,
         largura_media: dbg!(media),
         colunas: dbg!(largura/(comprimento + X)),
         maior_comprimento: dbg!(comprimento)
      }
   }
   
   // adiciona uma nova entrada para visualização.
   pub fn adiciona_entrada(&mut self, entrada:&'a str) {
      println!("nova entrada adicionada.");
      self.fila.push(entrada);
      // medindo comprimento e computando atual média.
      self.largura_media = { 
         let soma:usize = self.fila.iter()
         .map(|s| s.len())
         .sum();
         let total = self.fila.len();
         (soma / total) as u8
      };
      // só alterar maior entrada se... for maior.
      if self.maior_comprimento < entrada.len() as u8 {
         self.maior_comprimento = entrada.len() as u8;
      }
   }
}

impl Display for Impressao<'_> {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> Resultado {
      // string para concatenação.
      let mut string = String::new();

      // iterando com fila, o primeiro inserido irá primeiro.
      let mut iterador = self.fila.iter();
      let mut n:u8 = 1;
      // enquanto o iterador não for inteiramente consumido...
      while let Some(s) = iterador.next() {
         // concatenando string...
         string.push_str(s);
         // insere quebra de linha no final da quarta coluna.
         if n % self.colunas == 0 
            { string.push('\n'); }
         else { 
            // computando o espaço adequado.
            let espaco:usize = {
               // comprimento da atual string, também apelido.
               let cs:u8 = s.len() as u8;
               // alias para "maior comprimento".
               let mcs:u8 = self.maior_comprimento;
               /* se for menor, ascrescenta a diferença e
                * o acrescimo constante 'X' básico. */
               if cs < mcs 
                  { ((mcs-cs) + X) as usize }
               // caso contrário só o acréscimo 'X'.
               else { X as usize }
            };
            // adiciona espaçamento.
            string.push_str(&" ".repeat(espaco)); 
         }
         n += 1;
      }
      // colocando quebra de linha final, se necessário.
      string += "\n";

      // escrevendo no output o resultado.
      write!(formatador, "{}", string)
   }
}

// margem e símbolo de inscrita.
const SIMBOLO:&str = "#";
const MARGEM:usize = 3;

/* remove espaços em brancos no começo
 * da slice-string. */
fn remove_espacos_brancos<'b>(texto:&'b str) -> &'b str {
   // localizando o começo do texto em sí.
   let mut marco:usize = usize::MAX;
   // indo até o primeiro caractére não branco.
   for (j, char) in texto.char_indices() {
      /* interromper o laço quando acha-lô, 
       * e 'marcar' de tal. */
      if !char.is_whitespace() {
         marco = j;
         break;
      }
   }
   /* se tiver ocorrido uma alteração 
    * obter faixa específica da string
    * passada. */
   if marco != usize::MAX {
      texto
      .get(marco..)
      .unwrap()
   }
   // retornar string-slice original caso contrário.
   else { texto }
}

/** Pega uma *slice-string* e circunscreve ela
 com caractéres.  */
pub fn circunscrever(texto:&str) -> String {
   // concatenador.
   let mut nova_str:String = String::new();
   // string após filtro.
   // todas linhas existentes.
   let linhas = {
      texto
      .lines()
      .map(|s| remove_espacos_brancos(s))
      .map(|s| 
         if StringExtensao::len(s) > 0 { s } 
         else { "---------" }
      )
   };
   // comprimento da maior string.
   let maior_str:usize = {
      linhas.clone()
      .map(|s| StringExtensao::len(s))
      .max()
      .unwrap()
   };
   // espaço superior.
   let qtd:usize = maior_str + 2*MARGEM + 2;
   // espaço em branco na vertical do texto.
   let margem_superior:String = format!(
      "{ch}{branco}{ch}\n",
      branco = &" ".repeat(qtd-2),
      ch = SIMBOLO
   );
   // barras que fecham o retângulo que circunscreve.
   let barra:&str = &SIMBOLO.repeat(qtd);

   // criando a barra.
   nova_str += barra;
   nova_str += "\n";
   // margem superior.
   nova_str += margem_superior.as_str(); 

   // transformando string e concatenando, linha-por-linha.
   for s in linhas {
      // compensador de váculo.
      let diferenca = maior_str - StringExtensao::len(s);
      // formatando string para concatenação.
      let aux = format!(
         "{caractere}{espaco}{string}{espaco_corrigido}{caractere}\n",
         string = s,
         espaco = &" ".repeat(MARGEM),
         caractere = SIMBOLO,
         espaco_corrigido = &" ".repeat(MARGEM + diferenca)
      );
      // concatenando em sí...
      nova_str += aux.as_str();
   }

   // margem superior.
   nova_str += margem_superior.as_str(); 
   // fechando tudo com barra inferior.
   nova_str += barra;
   nova_str += "\n";
   // retornando nova string que mostra a 
   // original circunscrita.
   return nova_str;
}


#[cfg(test)]
mod tests {
   // importando módulo acima.
   use crate::impressao::*;

   #[test]
   fn teste_basico_escadinha() {
       let strings = vec!["era", "uma","casa", "muito","engraçada"];
       println!("visualizando:\n{}", escadinha(strings));
       assert!(true);
   }

   #[test]
   fn verificando_reverso() {
      let strings:Vec<&str> = vec![
         "era", "uma","casa", "muito","engraçada",
         "vácuo", "original","casarão", "muito", "computador",
         "fio", "ovo","buraco", "ármario", "caixas-de-som",
         "barata", "faca","xícara", "teclado", "quadrado",
         "sem-fio", "ovário", "sofa", "ármario", "porte",
         "heranca", "mágica","cabrito", "muito","engraçada",
         "barata", "canivete","viação", "tubulução", "retângulo",
         "barata", "faca","xicara", "teclado", "quadrado",
         "sem-fio", "ovário", "sofa", "armario", "porte",
         "herança", "mágica","cabrito", "muito","engraçada",
         "barata", "canivete","viaçao", "tubulução", "retângulo",
         "barata", "faca","xicara", "teclado", "quadrado",
         "sem-fio", "ovario", "sofá", "ármario", "porte",
         "herança", "mágica","cabrito", "muito","engraçada",
         "barata", "canívete","viação", "tubulução", "retângulo",
      ];
      println!("visualizando:\n{}", escadinha(strings));
      assert!(true);
   }

   #[test]
   fn teste_impressao_struct_parte_i() {
      let dirs:Vec<&str> = vec![
         "pasta_1", "pasta_2", "pasta_3", "pasta_4",
         "pasta_5", "pasta_6", "pasta_7", "pasta_8",
         "pasta_9", "pasta_10", "pasta_11", "pasta_12",
      ];

      let mut visual = Impressao::cria(dirs);
      println!("{}", visual);

      println!("verificando com fica a adição de mais entradas");
      visual.adiciona_entrada("pasta 14");
      visual.adiciona_entrada("pasta 15");
      visual.adiciona_entrada("pasta 16");
      println!("{}", visual);

      assert!(true);
   }

   #[test]
   fn outside_mudando_impressao() {
      let dirs:Vec<&str> = vec![
         "pasta_1", "pasta_322", "pasta_3", "pasta_4",
         "pasta_5", "pasta_6", "pasta", "pasta_8",
         "pasta_9", "pasta_12120", "pasta_11", "pasta_12",
      ];

      let visual = Impressao::cria(dirs);

      println!("{}", visual);
      assert!(true);
   }

   #[test]
   fn testando_circunscricao_strings_formatadas() {
      let str_teste = "
      \ri see trees of green
      \rred of roses too
      \ri see the blue
      \rfor me and you
      \rwhen i think to myself
      \rwhat wonderful world
      \ri see sky of blue
      \rclouds of white
      \rthe bright bless day";
      let str_bordada = circunscrever(&str_teste);
      println!("{}", str_bordada);

      let str_teste = " 
      \rninety-six thousands three hundreds and twenty-five
      \rtwelve thousands 8 hundreds
      \reighty-four
      \rfour hundreds nine";
      let str_bordada = circunscrever(&str_teste);
      println!("{}", str_bordada);

      let str_teste = {
      "\rrosas são vermelhas
      \rvioletas são azuís
      \reu vejo um coelhinho
      \rcomendo alcazúis." };
      let str_bordada = circunscrever(&str_teste);
      println!("{}", str_bordada);

      assert!(true);
   }
}
