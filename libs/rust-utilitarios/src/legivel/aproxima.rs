
// biblioteca padrão do Rust.
use std::str::FromStr;
// próprio módulo.
use super::{
   SECULO, DECADA, MILENIO, MES,
   DIA, HORA, MINUTO, ANO
};

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

   // converte fração.
   let conversao = super::tempo(calc, false);
   /* arredonda se está quebrada, pois a recursão 
    * do "tempo_detalhado" ainda não foi implementada.  */
   let conversao = arredondando_str(conversao.as_str());

   // concatenando as partes:
   let mut s = String::new();
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
