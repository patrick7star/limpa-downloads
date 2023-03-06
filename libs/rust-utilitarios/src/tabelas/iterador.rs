

/* Colocando o iterador aqui por 
 * motivo simplesmente... de
 * refatoração. Não só toma muito
 * espaço no outro arquivo, como também
 * tem funções auxiliares que fazem 
 * o mesmo, sem falar nos testes unitários.
 */

// biblioteca padrão:
use std::fmt::Display;
// restante do módulo:
use super::objeto::Coluna;


pub struct ColunaStr {
   //forma_padrao: String,
   /* quantia padrão de linhas da Coluna,
    * ou uma nova preenchido com campos
    * em branco. */
   pub altura: Option<usize>,
   // células e cabeçalho mais espaços vázios.
   iterador: Vec<String>,
   // posição do atual item no iterador.
   posicao: usize,
   // largura da ColunaStr, que é a mesma
   // da Coluna passada na instância.
   pub largura: usize
}

// valor da diferença em absoluto de inteiros:
fn abs(x: usize, y:usize) -> usize {
   if x >= y { x - y }
   else { y - x }
}

fn campo_vago(comprimento: usize) -> String {
   let mut vago = (&" ").repeat(comprimento);
   // computa o meio da string, recuado alguns passos.
   let meio = comprimento/2 + 1 - 3;
   vago.insert_str(meio, "---");
   // remove os extremos por causa dos "recuos".
   vago.pop().unwrap();
   vago.pop().unwrap();
   vago.pop().unwrap();
   return vago;
}

impl ColunaStr {
   // método construtor:
   pub fn nova<X>(coluna: Coluna<X>, aumento: Option<usize>) 
   -> Self 
      where X: Display + Clone 
   {
      let mut iterador: Vec<String> = Vec::new();
      // decompondo sua forma de string em linhas...
      //let versao_str = format!("{}", coluna.clone());
      for linha in coluna.to_string().lines() 
      //for linha in versao_str.lines() 
         { iterador.push(linha.to_string()); }

      match aumento {
         Some(valor) => {
            let lrg = coluna.largura();
            let cv = campo_vago(lrg);
            let diferenca = abs(valor, coluna.linhas());

            for _ in 1..=diferenca 
               { iterador.push(cv.clone()); }
         } None => ()
      };

      Self {
         //forma_padrao, 
         altura: aumento, 
         iterador, posicao: 0, 
         largura: coluna.largura()
      }
   }
   // atualiza a quantia de campos vázios da ColunaStr.
   pub fn atualiza(&mut self, aumento: Option<usize>) {
      match aumento {
         Some(valor) => {
            let l = self.largura;
            let ql = match self.altura {
               Some(valor) => valor,
               None => 0
            };
            let cv = campo_vago(l);
            let diferenca = valor-ql;
            // adicionando a 'diferença' ...
            for _ in 1..=diferenca 
               { self.iterador.push(cv.clone()); }
            // resetando qualquer iteração.
            self.posicao = 0;
         } None => ()
      };
   }
}

// fazendo do objeto um iterador ...
impl Iterator for ColunaStr {
   type Item = String;

   // retorna a próxima célula string da Coluna.
   fn next(&mut self) -> Option<Self::Item> {
      if self.posicao <= self.iterador.len()-1 {
         self.posicao += 1;
         Some(self.iterador[self.posicao-1].clone())
      } else { self.posicao = 0; None }
   }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   //use crate::aleatorio::sortear;
   use super::*;

   #[test]
   fn struct_ColunaStr() {
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

      let altura = Some(generos.linhas());

      let mut strI = ColunaStr::nova(moedas_magicas, altura);
      println!(
         "largura={}\tqtd. de linhas={}",
         strI.largura, strI.altura.unwrap()
      );
      let mut strII = ColunaStr::nova(salarios, altura);
      println!(
         "largura={}\tqtd. de linhas={}",
         strII.largura, strII.altura.unwrap()
      );
      let mut strIII = ColunaStr::nova(generos, altura);
      println!(
         "largura={}\tqtd. de linhas={}",
         strIII.largura, strIII.altura.unwrap()
      );


      for _ in 1..=altura.unwrap() {
         let c1 = strI.next().unwrap();
         let c2 = strII.next().unwrap();
         let c3 = strIII.next().unwrap();
         println!("#{}#{}#{}#", c1, c2, c3);
      }

      // avaliação manual.
      assert!(true);
   }
}
