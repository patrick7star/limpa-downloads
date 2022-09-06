
/* Complementos de strings(traits)
 * que são úteis durante o desenvolvimento
 * por motivos de organização serão
 * passados para aqui.
 */


/* reescrevendo o método do len da string para 
 * pegar acentuações conhecidas de dois bytes.
 */
pub trait StringExtensao {
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
