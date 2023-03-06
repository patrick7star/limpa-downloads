
use std::fmt::{Display, Formatter, Result as Result_fmt};
use std::time::{Duration, Instant};
use std::ops::Range;

/** É um letreiro dinâmico que dado um string
 a cada determinado tempo, as letras se movem
 da direita para esquerda, ou vice-versa. É 
 um objeto muito útil se todo o nome do progresso
 não cabe totalmente na tela.
*/
#[derive(Clone)]
pub struct Logo<'a> {
   // para marcar o tempo.
   ti:Instant,
   // o texto que será mostrado.
   rotulo:&'a str,
   // quando da string mostrar.
   capacidade:u8,
   // inicio e fim onde visualizar a string.
   ponta_esquerda:u8,
   ponta_direita:u8,
   // intervalo válido.
   intervalo:Option<Range<usize>>,
}

impl <'a> Logo<'a> {
   // criando uma nova instância.
   pub fn novo(label:&str) -> Result<Logo, &'static str> {
      if label.len() == 0 {
         Err("não é permitido strings em branco")
      }
      else {
         Ok(
            Logo {
               // iniciando contagem.
               ti: Instant::now(),
               // pegando o rótulo a dimanizar.
               rotulo: label,
               // capacidade definida manualmente.
               capacidade: 15, // quinze caractéres.
               ponta_esquerda: 0,
               ponta_direita: 15,
               intervalo:Some(0..15),
            }
         )
      }
   }

   // motor do logo. 
   pub fn movimenta_letreiro(&mut self) {
      // se chegou ao final, resetar posição do LED.
      if self.ponta_direita == self.rotulo.len() as u8 {
         self.ponta_direita = self.capacidade;
         self.ponta_esquerda = 0;
      }
      // a cada 1,5seg mover o led 'uma casa'.
      if self.ti.elapsed() > Duration::from_millis(500) {
         if self.ponta_direita <= self.rotulo.len() as u8 {
            // deslocando led...
            self.ponta_esquerda += 1;
            self.ponta_direita += 1;
            // resetando contagem...
            self.ti = Instant::now();
         }
      }
      // definindo novo intervalo.
      self.intervalo = {
         // "renomeação" para melhor legibilidade.
         let pe:usize = self.ponta_esquerda as usize;
         let pd:usize = self.ponta_direita as usize;
         Some(pe..pd)
      };
   }

   // transforma numa slice-string.
   pub fn para_string(&self) -> &'a str {
      match self.intervalo.clone() {
         Some(i) => {
            self.rotulo
            .get(i)
            .unwrap()
         } None => self.rotulo,
      }
   }

   // nova capacidade do logo.
   pub fn nova_capacidade(&mut self, capacidade:u8) {
      self.capacidade = capacidade;
   }
}

impl Display for Logo<'_> {
   fn fmt(&self, f:&mut Formatter<'_>) -> Result_fmt {
      // apeliando para legibilidade.
      match self.intervalo.clone() {
         Some(i) => {
            let string = {
               match self.rotulo.get(i) {
                  Some(r) => r,
                  None => {
                     let max = self.capacidade as usize;
                     let comeco = 0..max;
                     self.rotulo.get(comeco).unwrap()
                  }
               }
            };
            write!(f, "{}...", string)
         } None => write!(f, "{}", self.rotulo)
      }
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use std::thread::sleep;

   #[test]
   fn letreiro_dinamico() {
      // instanciando logo dinâmico...
      let texto = concat!(
         "isso e apenas um texto de teste, ",
         "entao nao entre em panico"
      );
      let mut logo = Logo::novo(texto).unwrap();
      // marcador de tempo.
      let t:Instant = Instant::now();
      while t.elapsed() < Duration::from_secs(15) {
         print!("\r{}", logo.para_string());
         logo.movimenta_letreiro();
      }
      assert!(true);
   }

   #[test]
   fn testando_funcao_que_gira() {
      let texto = "eu adoro suco de caju";
      let mut logo = Logo::novo(texto).unwrap();
      // movimento o texto, dormindo de acordo com
      // o tempo de translação dele(simulando tempo).
      sleep(Duration::from_secs_f32(0.5));
      logo.movimenta_letreiro();
      sleep(Duration::from_secs_f32(0.5));
      logo.movimenta_letreiro();
      sleep(Duration::from_secs_f32(0.5));
      logo.movimenta_letreiro();
      // tirando trecho translado.
      let parte_i = logo.para_string();
      /* o previsto, levando o tempo, e, 
       * lembrando que o "LED" tem 14 locais,
       * tem que cair exatamente como a 
       * frase abaixo.
       */
      let parte_ii = "adoro suco de c";
      // verificando resposta.
      assert_eq!(parte_i, parte_ii);
   }
}
