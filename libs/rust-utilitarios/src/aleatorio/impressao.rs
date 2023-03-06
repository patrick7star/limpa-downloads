

/* cria um print com impressão
 * programada(em segundos). */

use std::time::{Instant, Duration};

#[allow(dead_code)]
pub struct PrintProgramatico {
   cronometro: Instant,
   periodo: Duration,
   contador: usize,
}

#[allow(dead_code)]
impl PrintProgramatico {
   // cria scheduler ...
   pub fn novo(periodo: Duration) -> Self {
      return PrintProgramatico {
         cronometro: Instant::now(),
         periodo,
         contador: 0,
      };
   }
   // mostra mensagem:
   pub fn dispara(&mut self, mensagem: &str) {
      let decorrido = self.cronometro.elapsed();
      let pausa = self.periodo.as_secs();
      if decorrido.as_secs() > pausa {
         println!("\n{}\n", mensagem); 
         // zera contagem.
         self.cronometro = Instant::now();
         self.contador += 1;
      }
   }
   // obtém total de disparo realizados.
   pub fn total_de_disparos(&self) -> usize
      { self.contador }
}

// abreviação:
#[allow(dead_code)]
pub type PP = PrintProgramatico;

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   use std::thread::sleep;

   #[test]
   fn metodoDispara() {
      let mut instancia = PP::novo(Duration::from_secs(5));
      for k in 1..=100 {
         let mensagem = format!("contagem em {}", k);
         instancia.dispara(mensagem.as_str());
         sleep(Duration::from_millis(300));
      }
      assert!(instancia.total_de_disparos() > 0);
   }
}
