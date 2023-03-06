
/* cuidado especial com frações de 
 * segundos. A outra função que há no
 * módulo cuida apenas com segundos
 * inteiros. Também traduz o tipo 
 * 'Duration'.
 */


// submúltiplos de tempo conhecidos(fraçoes de segundos).
const MILI_SEG:f64 = 1.0/1000.0;
const MICRO_SEG:f64 = 1.0/1_000_000_f64;
const NANO_SEG: f64 = 1.0/1_000_000_000f64;
const PICO_SEG: f64 = 1.0 / 1_000_000_000_000f64;

/** mesmo que a função original de 
 legibilidade, porém aplicado neste 
 caso para frações de segundos.
*/
pub fn tempo_fracao(t: f64, contracao:bool) -> Option<String> {
   // renomeação da variável a comparar e computar.
   let calculo: f64;
   let sigla: &str;

   if t >= MILI_SEG && t < 1.0  {
      sigla = if contracao {"ms"} else {"milisegundos" };
      calculo = t * 1_000.0 ;
   } else if t >= MICRO_SEG && t < MILI_SEG {
      sigla = if contracao{"μs"} else{"microsegundos"};
      calculo = t * 1_000_000.0;
   } else if t >= NANO_SEG && t < MICRO_SEG {
      sigla = if contracao{"ns"} else{"nanosegundos"};
      calculo = t * 10.0f64.powf(9.0);
   } else if t >= PICO_SEG && t < NANO_SEG {
      sigla = if contracao{"ps"} else{"picosegundos"};
      calculo = t * 10.0f64.powf(12.0);
   } else 
      { return None; }

   // criando formatação.
   Some(format!("{:0.1} {}", calculo, sigla))
} 

use std::time::Duration;
/** Dado um struct 'Duration' ele retorna uma
 string com seu tempo em representação legível.
 Este é uma função bem genérica, pois utiliza
 um tipo de dado que é muito usado, e também
 "cúspido" da maioria das funções do sistema e
 externas também. */
pub fn tempo_generico(t: Duration, curto: bool) -> Option<String> {
   let segundo = Duration::from_secs(1);

   // tratando de frações de segundos.
   if t < segundo
      { tempo_fracao(t.as_secs_f64(), curto) }
   else 
      { Some(super::tempo(t.as_secs(), curto)) }
}


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn amostras_basicas() {
      let amostras: [f64; 11] = [
         0.00129, 0.010292012, 0.000072,
         0.29, 0.00000031, 0.00000000588,
         0.000000075, 0.000000067, 
         0.0000000000019, 0.000093,
         0.000000000000519,
      ];
      for a in amostras.iter() {
         match tempo_fracao(*a, true) {
            Some(traducao) =>
               { println!("{} >> {}", a, traducao); }
            None =>
               { println!("não é possível valores como {}", *a); }
         };
      }
   }

   #[test]
   fn teste_simples() {
      let inteiro = Duration::from_secs(942_392);
      let fracao = Duration::from_secs_f32(0.0000012);
      println!(
         "\t{}\t{}",
         tempo_generico(inteiro, true).unwrap(),
         tempo_generico(fracao, true).unwrap()
      );
   }
}
