
// bibliotecas do Rust:
use std::fmt::{Formatter, Display, Result as Resultado};
use std::string::String;

// meus caixote:
use crate::terminal_dimensao::{ Largura, Altura, dimensao };


#[derive(Debug, Clone)]
pub struct Coluna <U: ToString + Clone + Copy> {
   // legenda do rol de dados.
   rotulo:&'static str,

   // rol de dados.
   rol:Vec<U>
}

impl<U:Display + Copy> Display for Coluna <U> {
   fn fmt(&self, molde:&mut Formatter<'_>) -> Resultado {
      // string de concatenação.
      let mut s = String::from("");
      // maior comprimento de caractéres:
      let mut c_max:u8 = self.rotulo.len() as u8;
      
      // busca maior comprimento...
      for x in self.rol.clone() {
         let c = x.to_string().len();
         if c as u8 > c_max {
            c_max = c as u8;
         }
      }

      s.push_str(calibra_str(self.rotulo, c_max).as_str());
      s.push('\n');
      for v in self.rol.clone() {
         let  ss = calibra_str(v.to_string().as_str(), c_max);
         s.push_str(ss.as_str());
         s.push('\n');
      }
      // escrevendo no formatdor...
      write!(molde, "{}", s)
   }
}


// caractéres especiais:
// cantos:
static CANTO_SE:&str = "\u{256d}";
static CANTO_SD:&str = "\u{256e}";
static CANTO_IE:&str = "\u{2570}";
static CANTO_ID:&str = "\u{256f}";
// lateral e horizontais:
static LATERAL:&str = "\u{2502}";
static TRACO:&str = "\u{2500}";
// conectores laterais, pontas e do meio.
static LATERAL_CD:&str = "\u{2524}"; 
static LATERAL_CE:&str = "\u{251c}";
static CRUZ:&str = "\u{253c}";
static TRACO_I:&str = "\u{2534}";
static TRACO_S:&str = "\u{252c}";


trait StringExtensao {
   /* computa o tamanho de bytes entre strings
    * levando em conta caractéres de 2 bytes. */
   fn len_2_bytes(&self) -> u8;
}

impl StringExtensao for str {
   fn len_2_bytes(&self) -> u8 {
      // conta a quantia de acentuações comuns.
      let mut qtd:u8 = 0;
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
      let tamanho:u8 = self.len() as u8;
      return tamanho - qtd;
   }
}
impl StringExtensao for String {
   fn len_2_bytes(&self) -> u8 {
      self.as_str().len_2_bytes()
   }
}

/* centraliza uma string dado a largura onde
 * ela deve estar envolvida. */
fn calibra_str(s:&str, limite:u8) -> String {
   // diferença entre a slice-string e o máximo aceitável.
   let d:u8 = limite - s.len_2_bytes();
   //let d:u8 = limite-(s.len() as u8);
   // verifica se é ímpa?
   let e_impa = d % 2 != 0;
   // string para concatenação.
   let mut outra_str = String::from(s);
   
   // se for ímpa, adiciona 'char' à esquerda para calibrar.
   if e_impa { outra_str.push(' '); }
   
   // se for dois, adiciona apenas uma vez manualmente.
   if d == 0 { return String::from(s); }
   if d == 2 {
      outra_str.push(' ');
      outra_str.insert(0,' ');
   }
   // ser for um par maior que doiz adiciona 'd/2' vezes
   // em cada lado.
   else {
      let fim = d/2;
      for _ in 0..fim {
         outra_str.push(' ');
         outra_str.insert(0,' ');
      }
   }
   // retorna string ajustada.
   return outra_str;
}


/* os inputs de desta função são de forma abstratas
 * linhas de uma tabela, assim o processo aqui tem
 * como objetivo abstrato concatenar tabelas. */
fn entrelacando_strings(str1:&Vec<&str>, str2:&Vec<&str>) -> String {
   // string para concatenação.
   let mut tabela_str = String::from("");
   
   // percorrendo várias linhas e concatenando colunas de cada.
   let mut p1 = 0;
   let mut p2 = 0;
   while p1 < str1.len()-1 || p2 < str2.len()-1 {
      // primeiro delimitador.
      tabela_str += LATERAL;
      // célula da primeira coluna.
      if p1 < str1.len()-1 {
         tabela_str += str1[p1];
      }
      else {
         let compr = str1[0].len_2_bytes() as usize;
         tabela_str += &"*".repeat(compr).to_string();
      }
      // delimitador do meio.
      tabela_str += LATERAL;
      // célula da segunda coluna.
      if p2 < str2.len()-1 {
         tabela_str += str2[p2];
      }
      else {
         let compr = str2[0].len_2_bytes() as usize;
         tabela_str += &"-".repeat(compr).to_string();
      }
      // delimitador do fim e quebra-de-linha.
      tabela_str += format!("{}\n",LATERAL).as_str();

      // separador de linhas.
      p1 += 1;
      p2 += 1;
   }

   // retorno da string representado a tabela.
   return tabela_str;
}

/* pega duas estruturas 'Coluna' e junta-as numa 
 * formatação string, fazendo o resultado ser uma
 * tabela com estas duas 'Coluna's e seus rol's de
 * valores. */
pub fn tabelar_dados<I, C>(coluna_i:Coluna<I>, coluna_ii:Coluna<C>) 
-> String where I: Display + Copy, C: Display + Copy {
   // vetor com as strings ordenadas com base na linhas.
   let str_coluna_i = coluna_i.to_string();
   let str_coluna_ii = coluna_ii.to_string();
   let t1:Vec<&str> = {
      str_coluna_i 
      .as_str()
      .split('\n')
      .collect()
   };
   let t2:Vec<&str> = {
      str_coluna_ii 
      .as_str()
      .split('\n')
      .collect()
   };

   // concatena e retorna strings.
   let mut tabela = entrelacando_strings(&t1, &t2);
   // desenha linhas e bordas.
   circunscreve_borda(&mut tabela);
   return  tabela;
}

fn limites_laterais(s:&String) -> Vec<usize> {
   // lista com índices da localização de tais
   // barras na string.
   let mut lista:Vec<usize> = Vec::new();

   // indo caractére por caractére na primeira
   // linha apenas, que é só o que é preciso.
   for (i,ch) in s.chars().into_iter().enumerate() {

      // adiciona o índice nesta linha.
      let ch_lateral:Vec<char> = LATERAL.chars().collect();
      if ch == ch_lateral[0] { lista.push(i); }

      // parar programa ao encontrar "outra linha".
      if ch == '\n' { break; }
   }
   
   // lista com os índices dos delimitadores laterais.
   return lista;
}

/* conta a quantia de quebra-de-linhas
 * na string, ou seja, conta a quantidade
 * total de linhas. */
fn qtd_de_linhas(s:&String) -> usize {
   let aux:Vec<&str> = {
     s.as_str()
     .lines()
     .into_iter()
     .collect()
   };
   return aux.len();
}

fn circunscreve_borda(s:&mut String) {
   // cruzamentos em T.
   let t_ligamentos = limites_laterais(s);

   // barra de fundo e topo.
   // trabalhando nos conectores laterais inferiores.
   let mut barra_superior = String::from(CANTO_SE);
   let mut barra_inferior = String::from(CANTO_IE);
   let mut barra_meio = String::from(LATERAL_CE);

   // três colunas, então quatro barras laterais.
   if t_ligamentos.len() == 4 {
      let qtd_parte = t_ligamentos[1]-t_ligamentos[0]-1;
      barra_inferior.push_str(&TRACO.repeat(qtd_parte));
      barra_superior.push_str(&TRACO.repeat(qtd_parte));
      barra_meio.push_str(&TRACO.repeat(qtd_parte));

      barra_inferior.push_str(TRACO_I);
      barra_superior.push_str(TRACO_S);
      barra_meio.push_str(CRUZ);

      let qtd_parte = t_ligamentos[2]-t_ligamentos[1]-1;
      barra_inferior.push_str(&TRACO.repeat(qtd_parte));
      barra_superior.push_str(&TRACO.repeat(qtd_parte));
      barra_meio.push_str(&TRACO.repeat(qtd_parte));

      barra_inferior.push_str(TRACO_I);
      barra_superior.push_str(TRACO_S);
      barra_meio.push_str(CRUZ);

      let qtd_parte = t_ligamentos[3]-t_ligamentos[2]-1;
      barra_inferior.push_str(&TRACO.repeat(qtd_parte));
      barra_superior.push_str(&TRACO.repeat(qtd_parte));
      barra_meio.push_str(&TRACO.repeat(qtd_parte));
   }
   // para o caso de duas colunas(então três laterais).
   else if t_ligamentos.len() == 3 {
      let qtd_parte = t_ligamentos[1] - t_ligamentos[0] - 1;
      barra_inferior.push_str(&TRACO.repeat(qtd_parte));
      barra_superior.push_str(&TRACO.repeat(qtd_parte));
      barra_meio.push_str(&TRACO.repeat(qtd_parte));

      barra_inferior.push_str(TRACO_I);
      barra_superior.push_str(TRACO_S);
      barra_meio.push_str(CRUZ);

      let qtd_parte = t_ligamentos[2] - t_ligamentos[1] - 1;
      barra_inferior.push_str(&TRACO.repeat(qtd_parte));
      barra_superior.push_str(&TRACO.repeat(qtd_parte));
      barra_meio.push_str(&TRACO.repeat(qtd_parte));
   }
   // ponta e quebra-de-linha.
   barra_inferior.push_str(CANTO_ID);
   barra_inferior.push('\n');
   barra_superior.push_str(CANTO_SD);
   barra_superior.push('\n');
   barra_meio.push_str(LATERAL_CD);

   // implementando barra superior também.
   s.insert_str(0,barra_superior.as_str());
   s.push_str(barra_inferior.as_str());

   let copia = s.clone();
   let mut linhas:Vec<&str> = copia.as_str()
                             .lines().into_iter()
                             .collect();

   
   // qtd. inicial de linhas. 
   let ql = qtd_de_linhas(s)-2;

   // limpa string para reconstrução.
   s.clear();

   // iterlando ql-2 vezes.
   let mut n = 2;
   let mut contador = 1;
   /* se há n linhas, serão circunscritas, por 
    * baixo 'm-1' linhas. */
   while contador <= ql-1 {
      // circunscreve toda linha.
      linhas.insert(n, barra_meio.as_str());
      // contando cada delimitação de célula.
      contador += 1;
      // indo de dois em dois.
      n += 2; 
   }

   // recriando string, concatenando por 
   // quebra-de-linha as "linhas"!
   for substr in linhas {
      s.push_str(substr);
      s.push('\n');
   }
}

pub fn tabelar_tres_colunas <X, Y, Z>
(colunas:(Coluna<X>, Coluna<Y>, Coluna<Z>)) -> String 
where X: Display + Copy, Z: Display + Copy, Y: Display + Copy {
   // formatações em string de todos objetos.
   let str_cx = colunas.0.to_string();
   let str_cy = colunas.1.to_string();
   let str_cz = colunas.2.to_string();
   
   // divindindo linhas:
   let cx:Vec<&str> = str_cx.as_str().split('\n').collect();
   let cy:Vec<&str> = str_cy.as_str().split('\n').collect(); 
   let cz:Vec<&str> = str_cz.as_str().split('\n').collect();

   // string concatenadora.
   let mut tabela = String::from("");
   // índices de cada coluna.
   let (mut i, mut j, mut k) = (0, 0, 0);

   while i < cx.len()-1 || j < cy.len()-1 || k < cz.len()-1 {
      // demilitador esquerdo.
      tabela += "\u{2502}";
      // verifica se a primeira coluna não se esgotou.
      if i < cx.len()-1 
         { tabela += cx[i]; }
      else 
         { tabela += &"*".repeat(cx[1].len()).to_string(); }

      tabela += "\u{2502}";
      // se a segunda não esgotou-se.
      if j < cy.len()-1
         { tabela += cy[j]; }
      else 
         { tabela += &"*".repeat(cy[1].len()); }

      tabela += "\u{2502}";
      // se a terceira não esgotou-se.
      if k < cz.len()-1 
         { tabela += cz[k]; }
      else
         { tabela += &"-".repeat(cz[1].len()); }
      tabela += "\u{2502}\n";
      
      // contabilizando cada...
      i += 1; j += 1; k += 1;
   }

   // desenhas os limites das colunas e células.
   circunscreve_borda(&mut tabela);
   
   // retorno da tabela concatenada.
   return tabela;
}

pub fn otimiza_tela(tabela_str:String) -> Result<String, &'static str> {
   /* quantia total de linhas da 'tabela_str' e 
    * sua largura máxima. */
   let ql:u16 = qtd_de_linhas(&tabela_str) as u16;
   // dimensão do terminal.
   let (altura, _largura):(u16, u16) = {
      match dimensao() {
         Some((Largura(l), Altura(a))) => (a, l),
         None => (0, 0)
      }
   };
   /* razão, para saber a proejeção em toda
    * tela. */
   #[allow(unused_variables)]
   let razao:u16 = ql / altura;
   #[allow(unused_variables)]
   let tl:u16 = tabela_str.find("\n").unwrap() as u16;

   // verificando se é maior.
   if altura < ql {
      // array contendo 'String' das linhas.
      let mut linhas:Vec<String> = {
         tabela_str
         .lines()
         .map(|s| s.to_string())
         .collect()
      };

      // divide em três ou mais partes.
      if ql / (altura-1) >= 2 {
         /*
         // informação importante para debug.
         let excedente = ql % altura;
         println!(
            "\rexcedente={}
            \rrazão = {}
            \rterminal-largura = {}
            \rqtd. de linhas = {}
            \rterminal-altura = {}
            \rarray-tamanho = {}",
            excedente, ql/altura,
            largura, ql, altura,
            linhas.len()
         ); */
         // realizando a operação 'excedente' vezes. 
         let mut p:usize = 0;
         let h:usize = altura as usize;
         while linhas.len() > h {
            if p > h {
               let remocao:String = linhas.remove(h);
               linhas[p % h + 1] += format!(" {}", remocao).as_str();
            }
            p += 1;
         }
      }
      else {
         // quantia que excede altura da tela do terminal.
         let mut excedente = ql % altura;
         // realizando a operação 'excedente' vezes. 
         for p in 0..(excedente as usize) {
            // onde remover contabilizando o excedente.
            let indice:usize = linhas.len() - (excedente as usize);
            // linha removida para ser colocada lado do topo.
            let remocao = linhas.remove(indice);
            // mais dois para pular cabeçário.
            linhas[p+2] += format!(
               "{espaco}{extra}",
               espaco = " ",
               extra = remocao
            ).as_str();
            // diminuindo linhas que já foram concatenadas.
            excedente -= 1;
         }
      }

      // transformando novamente numa string.
      let mut nova_ts = String::new();
      for s in linhas.iter() {
         nova_ts += s;
         nova_ts += "\n";
      }
      // retornando string construída para preencher tela.
      Ok(nova_ts)
   }
   // se não execeder a dimensão do terminal, apenas
   // retornar o original.
   else { Ok(tabela_str) }
}


// -------- testes do módulo ------------
#[cfg(test)]
mod tests {
   #[test] 
   fn tipo_generico_teste() {
      let idds = super::Coluna{rotulo:"idade(anos)",
      rol:vec![12,15, 18, 20,10, 5, 3]};
      let pesos = super::Coluna{
         rotulo:"massa(kg)",
         rol:vec![60.32, 58.21, 70.32, 
                   55.1, 37.1, 10.08]
      };
      let generos = super::Coluna{
         rotulo:"gênero(macho/fêmea)", 
         rol: vec!['F','M','M','F','M','M','F','F','F','M']
      };

      println!("{}",pesos);
      println!("{}", idds);
      println!("{}", generos);

      assert!(true);
   }

   #[test]
   fn concatenacao_de_colunas() {
      let idades = super::Coluna {
         rotulo:"idade(anos)", 
         rol:vec![29, 38, 89, 5, 32, 52, 10, 42, 60, 17, 25, 23,20]
      };

      let generos = super::Coluna{
         rotulo:"gênero(macho/fêmea)", 
         rol: vec![
            'F','F','F','F','M',
            'M', 'F','M','M','F',
            'F','F','M'
         ]
      };

      println!("{}",super::tabelar_dados(idades, generos));
      assert!(true);
   }
   
   #[test]
   fn concatenacao_de_colunas_diferentes() {
      let salarios = super::Coluna {
         rotulo:"salário(R$)",
         rol: vec![5288.32, 1_000.10, 893.25, 1_839.00]
      };

      let nomes = super::Coluna {
         rotulo:"funcionários",
         rol:vec!["Márcia","Luís","João","Caroline",
         "Júlia","Alexandria","Isabela","Alberto"]
      };

      let t = super::tabelar_dados(nomes, salarios);
      println!("{}",t);

      assert!(true);
   }


   #[test]
   fn tabulucao_dados_identicos() {
      let moedas_magicas = super::Coluna {
         rotulo:"moedas mágicas(qtd.)",
         rol: vec![198, 1923, 038, 932, 38_839,
                  3, 12, 538, 752, 65, 129]
      };

      let potes = super::Coluna {
         rotulo:"qtd. de potes",
         rol: vec![5, 15, 2, 7, 69,
                  1, 1, 7, 6, 2, 4]
      };

      println!("como fica:\n{}", super::tabelar_dados(moedas_magicas, potes));
      assert!(true);
   }

   #[test]
   fn tabular3dados() {
      let moedas_magicas = super::Coluna {
         rotulo:"moedas mágicas(qtd.)",
         rol: vec![198, 1923, 038, 932, 38_839,
                  3, 12, 538, 752, 65, 129]
      };

      let salarios = super::Coluna {
         rotulo:"salário(R$)",
         rol: vec![5288.32, 1_000.10, 893.25, 1_839.00]
      };

      let generos = super::Coluna{
         rotulo:"gênero(macho/fêmea)", 
         rol: vec!['F','F','F','F','M','M',
             'F','M','M','F','F','F','M']
      };
      
      let tupla = (salarios, moedas_magicas, generos);
      let tabela = super::tabelar_tres_colunas(tupla);
      println!("{}", tabela);
   }
   
   #[test]
   fn tabulacao_otimizando_tela() {
      let moedas_magicas = super::Coluna {
         rotulo:"moedas mágicas(qtd.)",
         rol: vec![
            198, 1923,189,419,938, 172,
            038, 932, 38_839, 3, 12, 89, 
            538, 752, 65, 129, 1928, 111,
            59_392, 3_831, 852, 238,3_000,
            89, 71, 582, 83, 81, 721, 53
         ]
      };

      let salarios = super::Coluna {
         rotulo:"salário(R$)",
         rol: vec![
            5288.32, 1_000.10, 893.25, 1_839.00,
            7278.22, 2_300.17, 773.95, 1_211.13,
            4_111.17, 4_356.50, 843.23, 1_171.18,
            5484.34, 1_404.12, 1_183.27, 3_037.30,
            1300.21, 5_312.19, 799.15, 9_832.18,
            3_151.57, 4_586.50, 549.93, 1_281.19,
            162.77, 1_942.43, 2_552.11, 1_521.00,
            895.19, 2_891.09, 512.99
         ]
      };

      let generos = super::Coluna{
         rotulo:"gênero(macho/fêmea)", 
         rol: vec![
            'F','F','F','F','M','M',
            'F','M','M','F','F','F',
            'M','M','F','M','F','F',
            'F','M','M','M','M','M', 
            'F', 'M', 'F', 'F'
         ]
      };
      // testanto, primeiramente, para três colunas.
      let tupla = (
         salarios.clone(), 
         moedas_magicas, 
         generos.clone()
      );
      let tabela = super::otimiza_tela(
         super::tabelar_tres_colunas(tupla)
      );
      // agora, teste para tduas colunas.
      let tabela_i = super::otimiza_tela(
         super::tabelar_dados(generos, salarios)
      );
      // visualização de ambos.
      println!("{}", tabela.unwrap());
      println!("{}", tabela_i.unwrap());
      // testado e confirmado para a quantia dada.
      assert!(true);
   }

   #[test]
   fn tabulacao_otimizando_tela_parte_ii() {
      let moedas_magicas = super::Coluna {
         rotulo:"moedas mágicas(qtd.)",
         rol: vec![
            099, 2223,189, 519, 938, 172,
            539, 932, 339, 623, 12, 89, 
            538, 861, 63, 129, 1778, 111,
            59_392, 4_812, 852, 238,3_101,
            89, 71, 582, 73, 81, 121, 53,
            538, 752, 64, 739, 1888, 119,
            4_392, 3_831, 952, 238,382,
            88, 61, 432, 13, 37, 721, 212,
            5_932,
         ]
      };

      let generos = super::Coluna{
         rotulo:"gênero(macho/fêmea)", 
         rol: vec![
            'F','F','F','F','M','M',
            'F','M','M','F','F','F',
            'M','F','F','M','F','F',
            'F','M','M','F','M','M', 
            'F','M','F','F','M','F',
            'M','M','M','F','F','F',
            'M','M','F','F','M','F',
            'F','M','M','M','F','F',
            'M'
         ]
      };
      // testando com duas colunas.
      let tabela = super::otimiza_tela(
         super::tabelar_dados(generos, moedas_magicas)
      );
      // visualização de ambos.
      println!("{}", tabela.unwrap());
      assert!(true);
   }
}
