
/*! 
 # Grandezas mais legíveis 
  Faz conversões de grandezas referentes a 
 dados utilizados em computação, ou outros 
 campos. 
*/

// extensão do módulo:
mod fracao_seg;
mod aproxima;
pub use fracao_seg::*;
pub use aproxima::*;


// múltiplos de tempo(equivalente em seg).
const MINUTO:f32 = 60.0;           // segundos por minuto.
const HORA:f32 = MINUTO*MINUTO;  // segundos por hora.
const DIA:f32 = 24.0*HORA;         // segundos por dia.
const MES:f32 = 30.0*DIA;          // segundos/mês.
const ANO:f32 = 365.0*DIA;         // segundos/ano.
const DECADA:f32 = 10.0*ANO;       // segundos/década.
const SECULO:f32 = 10.0*DECADA;    // segundos por século.
const MILENIO:f32 = 10.0*SECULO;   // seg/milênio.

// múltiplos de tamanho(equivalente em bytes).
const KILO:u64 = 2_u64.pow(10);  // bytes por kB.
const MEGA:u64 = 2_u64.pow(20);  // bytes por MB.
const GIGA:u64 = 2_u64.pow(30);  // bytes por GB.
const TERA:u64 = 2_u64.pow(40);  // bytes por TB.
const PETA:u64 = 2_u64.pow(50);  // bytes por PB.



/** retorna uma string contendo o valor legendado
  porém numa faixa mais legível. */
pub fn tempo(segundos:u64, contracao:bool) -> String {
   // renomeação da variável a comparar e computar.
   let t:f32 = segundos as f32;
   let calculo:f32;
   let sigla:&str;
   if t >= MINUTO && t < HORA {
      sigla = if contracao {"min"} else {"minutos" };
      calculo = t / MINUTO;
   }
   else if t >= HORA && t < DIA {
      sigla = if contracao{"h"} else{"horas"};
      calculo = t / HORA;
   }
   else if t >= DIA && t < MES {
      sigla = "dias";
      calculo = t / DIA;
   }
   else if t >= MES && t < ANO {
      sigla = if contracao{"mês"} else{"meses"};
      calculo = t / MES;
   }
   else if t >= ANO && t < DECADA {
      sigla = "anos";
      calculo = t / ANO;
   }
   else if t >= DECADA && t < SECULO {
      sigla = if contracao{"dec"} else{"décadas"};
      calculo = t / DECADA;
   }
   else if t >= SECULO && t < MILENIO {
      sigla = if contracao{"sec"} else{"séculos"};
      calculo = t / SECULO;
   }
   else if t >= MILENIO && t < 10_f32*MILENIO {
      sigla = "milênios";
      calculo = t / MILENIO;
   }
   else {
      sigla = if contracao{"seg"} else{"segundos"};
      calculo = t;
   }
   format!("{:0.1} {}", calculo, sigla)
} 

/** retorna uma string contendo o tamanho 
  legendado com um múltiplo, porém de forma
  mais legível. */
pub fn tamanho(qtd:u64, contracao:bool) -> String { 
   if qtd >= KILO && qtd < MEGA {
      let sigla = if contracao{"KiB"} else{"kilobytes"};
      format!("{:.1} {}", (qtd as f32 / KILO as f32), sigla)
   }
   else if qtd >= MEGA && qtd < GIGA {
      let sigla = if contracao{"MiB"} else{"megabytes"};
      format!("{:.1} {}", (qtd as f32 /MEGA as f32), sigla)
   }
   else if qtd >= GIGA && qtd < TERA {
      let sigla = if contracao{"GiB"} else{"gigabytes"};
      format!("{:.1} {}", (qtd/GIGA) as f32, sigla)
   }
   else if qtd >= TERA && qtd < PETA {
      let sigla = if contracao{"TiB"} else{"terabytes"};
      format!("{:.1} {}", (qtd/TERA) as f32, sigla)
   }
   else {
      let sigla = if contracao{"B's"} else {"bytes"};
      format!("{:.1} {}", qtd, sigla)
   }
}


#[cfg(test)]
mod tests {
   use crate::legivel::*;

   #[test]   
   fn testa_tamanho_legibilidade() {
      let mut x = 3419; 
      println!("{} ==> {}",x, tamanho(x, false));
      x = 10293; 
      println!("{} ==> {}",x, tamanho(x, false));
      x = 1982419; 
      println!("{} ==> {}",x, tamanho(x, true));
      x = 123048190; 
      println!("{} ==> {}",x, tamanho(x, true));
      x = 1000348293192; 
      println!("{} ==> {}",x, tamanho(x,false));
      x = 193843092384101; 
      println!("{} ==> {}",x, tamanho(x, true));
      assert!(true);
   }

   #[test]
   fn tempo_legibilidade() {
      let mut t:u64 = 36;
      println!("{} ==> {}", t, tempo(t, false));
      t = 152;
      println!("{} ==> {}", t, tempo(t, true));
      t = 552;
      println!("{} ==> {}", t, tempo(t, false));
      t = 9000;
      println!("{} ==> {}", t, tempo(t, false));
      t = 38910;
      println!("{} ==> {}", t, tempo(t, true));
      t = 1039842;
      println!("{} ==> {}", t, tempo(t, false));
      t = 30489123918;
      println!("{} ==> {}", t, tempo(t, true));
      t = 99990192152;
      println!("{} ==> {}", t, tempo(t, true));
      t = 1110238951152;
      println!("{} ==> {}", t, tempo(t, false));
      assert!(true);
   }

   #[test]
   fn testa_tempo_detalhado() {
      assert_eq!(
         super::tempo_detalhado("3.5 horas"), 
         Some(String::from("3 horas 30 minutos"))
      );

      assert_eq!(
         super::tempo_detalhado("4.2 dias"),
         Some(String::from("4 dias 5 horas"))
      );

      assert_eq!(
         super::tempo_detalhado("3.5 meses"), 
         Some(String::from("3 meses 15 dias"))
      );

      assert_eq!(
         super::tempo_detalhado("4.2 anos"),
         Some(String::from("4 anos 2 meses"))
      );


      assert_eq!(
         super::tempo_detalhado("7.9 décadas"),
         Some(String::from("7 décadas 9 anos"))
      );

      assert_eq!(
         super::tempo_detalhado("8.5 séculos"),
         Some(String::from("8 séculos 5 décadas"))
      );

      assert_eq!(
         super::tempo_detalhado("1.5 milênios"),
         Some(String::from("1 milênios 5 séculos"))
      );
   } 
}
