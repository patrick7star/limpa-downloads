
/** Colocando a coluna aqui, novamente,
por motivos de organização. Tudo fica
no mesmo arquivo dificulta "exponencialmente"
o processo de releitura do que já foi
codificado.
*/

// biblioteca do Rust:
use std::fmt::{Formatter, Display, Result as Resultado};
use super::StrExt;

// Abreviação por motivo de legibilidade.
type Str = &'static str;

/**
 Uma estrutura que representa uma coluna
 numa tabela de dados. Necessário um **rótulo** 
 e uma array representando o **rol** de dados 
 da legenda.
*/
#[derive(Debug, Clone)]
pub struct Coluna <U: ToString + Clone> {
   // legenda do rol de dados.
   rotulo:&'static str,
   // rol de dados.
   rol:Vec<U>,
   // largura máxima da coluna.
   largura: usize,
}

impl <U>Coluna<U> 
where U: ToString + Clone {
   pub fn nova(rotulo: Str, rol: Vec<U>) -> Self {
      let maior_do_rol: usize = {
         rol.iter()
         .map(
            |s| StrExt::len(&s.to_string())
         ).max().unwrap()
      };
      let largura = {
         [maior_do_rol, rotulo.len()]
         .iter().max()
         .unwrap()
         .clone()
      };

      Self { rotulo, rol, largura }
   }
   pub fn linhas(&self) -> usize
      { self.rol.len() + 1 }
   pub fn largura(&self) -> usize
      { return self.largura; }
}

impl<U:Display + Clone> Display for Coluna <U> {
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
         let  ss = calibra_str(
            v.clone().to_string()
            .as_str(), c_max
         );
         s.push_str(ss.as_str());
         s.push('\n');
      }
      // escrevendo no formatdor...
      write!(molde, "{}", s)
   }
}

/* centraliza uma string dado a largura onde
 * ela deve estar envolvida. */
fn calibra_str(s:&str, limite:u8) -> String {
   // diferença entre a slice-string e o máximo aceitável.
   //let d:u8 = limite - s.len_2_bytes();
   let d: u8 = limite - (StrExt::len(s) as u8);
   // verifica se é ímpa?
   let e_impa = d % 2 != 0;
   // string para concatenação.
   let mut outra_str = String::from(s);
   
   // se for ímpa, adiciona 'char' à esquerda para calibrar.
   if e_impa { outra_str.push(' '); }
   
   // se for dois, adiciona apenas uma vez manualmente.
   if d == 0 
      { return String::from(s); }
   if d == 2 {
      outra_str.push(' ');
      outra_str.insert(0,' ');
   }
   // ser for um par maior que doiz adiciona 'd/2' vezes
   // em cada lado.
   else {
      let fim = d / 2;
      for _ in 0..fim {
         outra_str.push(' ');
         outra_str.insert(0,' ');
      }
   }
   // retorna string ajustada.
   return outra_str;
}


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
}
