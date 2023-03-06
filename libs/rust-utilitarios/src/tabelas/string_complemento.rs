
/* Para princípios de organização
 * todos trait e implementação padrão,
 * referente a string serão colocados
 * aqui.
 */

pub type Str = &'static str;

pub trait StringExtensao<S> {
   /* maior entre duas strings. */
   fn max(&self, string: &S) -> usize; 

   /* computa o tamanho de bytes entre strings
    * levando em conta caractéres de 2 bytes. */
   fn len(&self) -> usize;

   /* converte uma string de um caractére
    * num char. */
   fn to_char(&self) -> Result<char, Str>;
}

impl StringExtensao<&str> for str {
   fn max(&self, string: &&str) -> usize {
      let a = StringExtensao::len(*string);
      let b = self.len();
      // se for maior, retorna ele.
      if a > b { a }
      // caso contrário, é maior ou igual
      // retornando o outro.
      else { b }
   }
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
      let tamanho:usize = self.len();
      return tamanho - qtd;
   }
   fn to_char(&self) -> Result<char, Str> {
      let tamanho = self.len();
      if tamanho >= 1 && tamanho <= 4 { 
         let mut caracteres = self.chars();
         let char = caracteres.next().unwrap();
         Ok(char)
      } else {
         if tamanho == 0
            { Err("uma string vázia") }
         else 
            { panic!("erro provavelmente desconhecido"); }
      }
   }
}

impl StringExtensao<String> for String {
   fn max(&self, string: &String) -> usize {
      let c_i = StringExtensao::len(string);
      let c_ii = self.len();
      if c_i > c_ii { c_i}
      else { c_ii }
   }

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
      let tamanho:usize = self.len();
      return tamanho - qtd;
   }

   fn to_char(&self) -> Result<char, Str> {
      let tamanho = self.len();
      if tamanho >= 1 && tamanho <= 4 { 
         let mut caracteres = self.chars();
         let char = caracteres.next().unwrap();
         Ok(char)
      } else {
         if tamanho == 0
            { Err("uma string vázia") }
         else 
            { panic!("erro provavelmente desconhecido"); }
      }
   }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;

   #[test]
   fn TesteCasting_to_char() {
      let mut s = String::new();
      for i in 9472..(9472 + 15*7)
         { s.push(char::from_u32(i as u32).unwrap()) }
      println!("{}", s);
      let s1 = String::from("\u{2500}");
      println!("{} ==> {}", s1, s1.to_char().unwrap());
   }
}
