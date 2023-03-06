/*!
 # Visualiazação de tabelas
  Um bom modo de organizar dados tabelados. O
 módulo possuí uma estrutura onde você pega 
 todo 'rol' de dados, cede um 'rótulo' a ele
 e toda vez que impresso será visualizado fechado
 por caractéres `Unicode` de uma maneira bem
 formatada no terminal. A estrutura `Coluna`
 que proporciona isso, também aceita a 
 impressão de outras juntas, formando assim uma
 bonita tabela.
*/
// bibliotecas do Rust:
use std::fmt::Display;
use std::string::String;

// extensão deste código:
mod string_complemento;
mod objeto;
mod tabelacao;
mod matriz_texto;
mod revestimento;
mod iterador;
use string_complemento::StringExtensao as StrExt;
// re-exportando ...
pub use objeto::Coluna;
pub use tabelacao::Tabela;


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
         //let compr = str1[0].len_2_bytes() as usize;
         let compr = StrExt::len(str1[0]);
         tabela_str += &"*".repeat(compr).to_string();
      }
      // delimitador do meio.
      tabela_str += LATERAL;
      // célula da segunda coluna.
      if p2 < str2.len()-1 {
         tabela_str += str2[p2];
      }
      else {
         //let compr = str2[0].len_2_bytes() as usize;
         let compr = StrExt::len(str2[0]);
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

/** Pega duas estruturas `Coluna` e junta-as numa 
 formatação string, fazendo o resultado ser uma
 tabela com estas duas `Coluna`s e seus *rol's* 
 de valores. 
*/
pub fn tabelar_dados<I, C>
(coluna_i:Coluna<I>, coluna_ii:Coluna<C>) -> String 
where I: Display + Copy, C: Display + Copy {
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
   let mut linhas: Vec<&str> = {
      copia.as_str()
     .lines()
     .into_iter()
     .collect()
   };

   
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

/// forma uma tabela com três colunas dadas.
pub fn tabelar_tres_colunas <X, Y, Z>
(colunas:(Coluna<X>, Coluna<Y>, Coluna<Z>)) -> String 
where X: Display + Copy, 
      Z: Display + Copy, 
      Y: Display + Copy {
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


// -------- testes do módulo ------------
#[cfg(test)]
mod tests {
   use super::*;

   #[test] 
   fn tipo_generico_teste() {
      let idds = Coluna::nova(
         "idade(anos)",
         vec![12,15, 18, 20,10, 5, 3]
      );
      let pesos = Coluna::nova(
         "massa(kg)",
         vec![60.32, 58.21, 70.32, 
             55.1, 37.1, 10.08]
      );
      let generos = Coluna::nova(
         "gênero(macho/fêmea)", 
         vec!['F','M','M','F','M','M','F','F','F','M']
      );

      println!("{}",pesos);
      println!("{}", idds);
      println!("{}", generos);

      assert!(true);
   }

   #[test]
   fn concatenacao_de_colunas() {
      let idades = Coluna::nova(
         "idade(anos)", 
         vec![
            29, 38, 89, 5, 32, 52, 
            10, 42, 60, 17, 25, 23,20
         ]
      );

      let generos = Coluna::nova(
         "gênero(macho/fêmea)", 
         vec![
            'F','F','F','F','M',
            'M', 'F','M','M','F',
            'F','F','M'
         ]
      );

      println!("{}", tabelar_dados(idades, generos));
      assert!(true);
   }
   
   #[test]
   fn concatenacao_de_colunas_diferentes() {
      let salarios = Coluna::nova(
         "salário(R$)",
         vec![5288.32, 1_000.10, 893.25, 1_839.00]
      );

      let nomes = Coluna::nova(
         "funcionários",
         vec!["Márcia","Luís","João","Caroline",
         "Júlia","Alexandria","Isabela","Alberto"]
      );

      let t = tabelar_dados(nomes, salarios);
      println!("{}",t);

      assert!(true);
   }


   #[test]
   fn tabulucao_dados_identicos() {
      let moedas_magicas = Coluna::nova(
         "moedas mágicas(qtd.)",
         vec![198, 1923, 038, 932, 38_839,
              3, 12, 538, 752, 65, 129]
      );

      let potes = Coluna::nova(
         "qtd. de potes",
         vec![5, 15, 2, 7, 69,
            1, 1, 7, 6, 2, 4]
      );

      println!(
         "como fica:\n{}", 
         tabelar_dados(moedas_magicas, potes)
      );
      assert!(true);
   }

   #[test]
   fn tabular3dados() {
      let moedas_magicas = Coluna::nova(
         "moedas mágicas(qtd.)",
         vec![198, 1923, 038, 932, 38_839,
            3, 12, 538, 752, 65, 129]
      );

      let salarios = Coluna::nova(
         "salário(R$)",
          vec![5288.32, 1_000.10, 893.25, 1_839.00]
      );

      let generos = Coluna::nova(
         "gênero(macho/fêmea)", 
          vec!['F','F','F','F','M','M',
             'F','M','M','F','F','F','M']
      );
      
      let tupla = (salarios, moedas_magicas, generos);
      let tabela = tabelar_tres_colunas(tupla);
      println!("{}", tabela);
   }
}
