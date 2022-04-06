
/**! 
  Tenta obter as dimensões do terminal,
 pois tal biblioteca é muito extensa, e
 utiliza de tal funções bastantes, e nisso
 recorre a bibliotecas externas(aliás
 está usando uma agora). Por isso vamos 
 tentar criar tais funções manualmente
 para desutilizar bibliotecas externas 
 que são utilizadas junto com esta, com
 as usadas internamente aqui para uma 
 função tão simples como tal. 

  O que está sendo feito aqui é na verdade
 uma cópia da atual biblioteca "terminal_size",
 só que feita manualmente, pelos meus meios
 encontrados. Não vi o código original como
 é feito. A estrutura deste será igual a
 dele, dentro de uma estrutura e tal. Porém
 sem a computabilidade de pixels.

  Claro que têm o fato de tal implementação
 ser intensamente mais lenta do que as 
 geralmente utilizadas.
*/

// biblioteca padrão do Rust.
use std::process::Command;
use std::str::FromStr;

/** 
 Um enum, pois a biblioteca original tem
 SnakeCase, logo eles devem implementar
 com enum's. 
*/
pub enum TerminalDimensao {
   /* Ambos valores de 16-bits, pois é 
    * muito difícil achar uma tela que 
    * possam conter milhões de caractéres
    * de cima/para o lado. Ou seja, (2^16-1)
    * é mais que o necessário. */
   Largura(u16),
   Altura(u16)
}

pub struct Largura(pub u16);
pub struct Altura(pub u16);


// apelido para melhorar legibilidade.
type Tupla = (TerminalDimensao, TerminalDimensao);
pub type TD = TerminalDimensao;


pub fn terminal_largura() -> Result<TD, &'static str> {
   // executa comando para obter largura primeiramente ...
   let mut resultado:Vec<u8> = {
      match Command::new("tput").arg("cols").output() {
         // retorna array de bytes que é o resultado.
         Ok(r) => r.stdout,
         Err(_) => 
            { return Err("não foi possível obter 'Largura'"); }
      }
   };

   // removendo quebra de linha.
   resultado.pop();
   // transformando em número.
   let mut caracteres:String = String::new();

   for ch in resultado.into_iter()
      { caracteres.push(ch as char); }

   /* converte para um inteiro positivo, e 
    * e registra valor para retorno, posteriormente. */
   let largura = u16::from_str(caracteres.as_str()).unwrap();
   // retornando encapsulado para possível erro.
   Ok(TerminalDimensao::Largura(largura))
}

pub fn terminal_altura() -> Result<TD, &'static str> {
   // executa comando para obter largura primeiramente ...
   let mut resultado:Vec<u8> = {
      match Command::new("tput").arg("lines").output() {
         // retorna array de bytes que é o resultado.
         Ok(r) => r.stdout,
         Err(_) => 
            { return Err("não foi possível obter 'Largura'"); }
      }
   };

   // removendo quebra de linha.
   resultado.pop();
   // transformando em número.
   let mut caracteres:String = String::new();

   for ch in resultado.into_iter()
      { caracteres.push(ch as char); }

   /* converte para um inteiro positivo, e 
    * e registra valor para retorno, posteriormente. */
   let altura = u16::from_str(caracteres.as_str()).unwrap();
   // retornando encapsulado para possível erro.
   Ok(TerminalDimensao::Altura(altura))
}

/// função retorna tupla com enum's de "ambos eixos".
pub fn terminal_dimensao() -> Result<Tupla, &'static str> {
   /* Primeiramente, de modo separado, obtém valores
    * por funções intermediárias, a largura e altura;
    * emite um erro se ocorrer algum. */
   let largura = terminal_largura()?;
   let altura = terminal_altura()?;

   /* retornando numa tupla com, primeiro a largura
    * e segundo a altura do terminal. */
   Ok((largura, altura))
}

/** função retorna tupla com dimensão, porém implementação
 distinta da anterior, encapsulando valores com 
 "structs" ao invés de Enum's. */
pub fn dimensao() -> Option<(Largura, Altura)> {
   /* usando construto acima de auxilio, para 
    * não ter que fazer a mesma coisa de novo.
    */
   let altura:u16 = match terminal_altura() {
      Ok(enumerador) => {
         match enumerador {
            TD::Altura(h) => h,
            _ => { return None; },
         }
      },
      Err(_) => {return None; },
   };
   let largura:u16 = match terminal_largura() {
      Ok(enumerador) => {
         match enumerador {
            TD::Largura(l) => l,
            _ => { return None; },
         }
      },
      Err(_) => {return None; },
   };

   // retorno, porém removendo valores dos enum's.
   return Some((Largura(largura), Altura(altura)));
}


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn testa_terminal_dimensao() {
      let (largura, altura):(u16, u16);
      if let Ok((TD::Largura(l), TD::Altura(h))) = terminal_dimensao()
         { largura = l; altura = h; }
      else { largura = u16::MIN; altura = u16::MAX; }
      assert!(largura > altura);
   }

   #[test]
   fn dimensao_especifica() {
      let (largura, altura):(u16, u16);
      if let Ok((TD::Largura(l), TD::Altura(h))) = terminal_dimensao()
         { largura = l; altura = h; }
      else { largura = u16::MIN; altura = u16::MAX; }
      assert!(largura > altura);
      assert_eq!(largura, 80);
      assert_eq!(altura, 20);
   }

   #[test]
   fn testa_dimensao() {
      let (largura, altura):(u16, u16);
      if let Some((Largura(l), Altura(h))) = dimensao()
         { largura = l; altura = h; }
      else { largura = u16::MIN; altura = u16::MAX; }
      assert!(largura > altura);
   }
}
