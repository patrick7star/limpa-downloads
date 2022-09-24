
/**! 
  Faz conversões de grandezas referentes a 
  dados utilizados em computação, ou outros 
  campos. 
*/


// biblioteca padrão do Rust.
use std::str::FromStr;


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

// submúltiplos de tempo conhecidos(fraçoes de segundos).
//const MILI_SEG:f64 = 1.0/1000.0;
//const MICRO_SEG:f64 = 1.0/1_000_000_f64;
//const NANO_SEG:f64 = 1.0/1_000_000_000f64;


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

/* arredonda um número inteiro, este
 * do com um byte de tamanho. */
fn arredonda(x:f32) -> u8 {
   let inteiro:u8 = x as u8;
   let fracao:f32 = ((inteiro as f32) - x).abs();
   if fracao < 0.5 { return inteiro;  }
   else { return inteiro + 1; }
}


/* arredonda o valor decimal da string
 * que representa tempo, segue as mesmas
 * regras de arredondamento de um valor comum. */
fn arredondando_str(s:&str) -> String {
   let mut partes = s.split_whitespace();
   
   let valor = f32::from_str(partes.next().unwrap()).unwrap();
   let peso = partes.next().unwrap();
   format!("{} {}", arredonda(valor), peso)
}

/** rescreve de forma mais legível a transformação
 convertendo a parte decimal em inteira rotuláda
 com o múltiplo/submultiplo adequado.
 Apenas funciona com valores decimais com dois
 dígitos significativos:

 # Exemplo:
 ```
   let trasnformacao_1 = tempo_detalhado("3.5 horas")
   let trasnformacao_2 = tempo_detalhado("8.1 minutos")
   let trasnformacao_3 = tempo_detalhado("5.4 meses")

   assert_eq!(transformacao_1, 
            Some(String::from("3 horas 30 minutos")));
   assert_eq!(transformacao_2, 
      Some(String::from("8 minutos 6 segundos")));
   assert_eq!(transformacao_3, 
      Some(String::from("5 meses 8 dias")));
 ``` */
pub fn tempo_detalhado<'a>(tempo_str:&'a str) -> Option<String> {
   let mut partes = tempo_str.split_whitespace();
   
   let valor = partes.next().unwrap();
   let peso = partes.next().unwrap();
   
   let (parte_inteira, parte_fracionaria) = {
      valor.split_once(".")
      .unwrap()
   };

   // partes inteira e fracionária em valores numéricos.
   let mut aux = String::from("0.");
   aux += parte_fracionaria;
   let f = f32::from_str(aux.as_str()).unwrap_or(0.0);

   // se não têm fração, não tem porque processar...
   if f == 0.0 { return None; }

   // converte no peso antecessor do dado.
   let calc:u64;
   if peso.contains("min") 
      { calc = (MINUTO * f) as u64; }
   else if peso.contains("hora")
      { calc = (HORA * f) as u64; }
   else if peso.contains("dia")
      { calc = (DIA * f) as u64 }
   else if peso.contains("mes") 
      { calc = (MES * f) as u64; }
   else if peso.contains("ano")
      { calc = (ANO * f) as u64; }
   else if peso.contains("dec") || peso.contains("déc")
      { calc = (DECADA * f) as u64; }
   else if peso.contains("sec") || peso.contains("séc")
      { calc = (SECULO * f) as u64; }
   else if peso.contains("mil")
      { calc = (MILENIO * f) as u64; }
   else 
      // sem peso, o argumento fica inválido.
      { return None; }
   println!("calc = {}", calc);
   // faz a conversão com a quantia de segundos correta.
   let _nao_contraido:bool = {
      vec!["dias", "horas", "minutos", "séculos",
          "milênios", "segundos", "meses", "anos",
          "semanas", "décadas"].contains(&peso)
   };
   // converte fração.
   let conversao = tempo(calc, false);
   /* arredonda se está quebrada, pois a recursão 
    * do "tempo_detalhado" ainda não foi implementada.  */
   let conversao = arredondando_str(conversao.as_str());

   // concatenando as partes:
   let mut s:String = String::new();
   s += parte_inteira;
   s += " ";
   s += peso;
   s += " ";
   s += conversao.as_str();

   // aparando desnecessário.
   s = s.replace(".0 ", " ");

   // retorna resultado encapsulado.
   return Some(s);
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
