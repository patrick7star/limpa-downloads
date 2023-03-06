

/* Produz o mesmo que o "módulo principal",
 * porém, enquato lá usa tempo de precisão(
 * que é muito volátil) e faz várias interações
 * para que tal, mesmo que ainda muito volátil,
 * gere entropia. O método usado aqui é pegar
 * isso diramente do sistema, o dispotivo
 * '/dev/random' gera vários bytes aleatórios
 * por nano segundos, o que será feito aqui
 * é só lê tal, na quantia necessária de bytes
 * para formar qualquer valor, e retorna-lô.
 * Apesar de ser um método que usa leitura de 
 * disco, espera-se que seja muito mais veloz
 * que o antigo.
 * Ambos métodos, por problemas de compatilibilidade
 * estarão disponíveis, no entanto, se tal
 * se provar muito mais eficiente que o outro,
 * ficará oficial, e o principal agora, 
 * descontinuado.
 * Por enquanto tal método só funciona no 
 * sistema Linux(talvez Unix).
 */

// biblioteca padrão do Rust:
use std::fs::OpenOptions;
use std::io::Read;
use std::ops::RangeInclusive;

type Bytes = Vec<u8>;
//type IntervaloZ = RangeInclusive<isize>;
type Zplus= RangeInclusive<usize>;
type Zminus= RangeInclusive<isize>;

fn pega_n_bytes(n: usize) -> Bytes {
   // tal gera bytes aleatórios a todo instante.
   let mut arquivo_entropico = {
      OpenOptions::new()
      .read(true)
      .open("/dev/random")
      .unwrap()
   };
   /* buffer de 64-bits(8 bytes), porém 
    * se pode pega apenas o necessário, 
    * claro, sem extrapolar o limite(que será 
    * expandido futuramente). */
   let mut bytes: [u8; 8] = [0; 8];

   // tenta lê "device" ...
   match arquivo_entropico.read(&mut bytes) {
      Ok(_) => (),
      Err(_) =>
         { panic!("não possível lê '/dev/random'"); }
   };

   // array com tamanho demandado.
   return bytes[0..n].to_vec();
}

/** O mesmo que o módulo `sortear`, no entanto foi
 feito para ser mais veloz, e ter um código 
 mais limpo.  Na verdade, ele será o sucessor do
 módulo citado, claro quando estiver mais maduro.
 O outro será descontinuado quando sanado todas 
 compatibilidades que ele oferece. */
pub mod randomico {
   use super::*;

   /// sorteio booleano básico.
   pub fn bool() -> bool {
      let byte = super::pega_n_bytes(1).remove(0);
      match byte {
         0..=127 => true,
         _ => false
      }
   }
   /// sorteio de um byte(u8).
   pub fn u8() -> u8 
      { return pega_n_bytes(1).remove(0); }
   /// sorteia um i8.
   pub fn i8() -> i8 {
      let sorteio = u8() % 128;

      match sorteio {
         1..=127 => {
            if bool() 
               { sorteio as i8 }
            else
               { (-1) * (sorteio  as i8) }
         } 0 => {
            if bool()
               { -128i8 }
            else
               { 0i8 }
         }
         _ => 
            { panic!("não se chega neste, NUNCA!!!"); }
      }
   }
   /// sorteia um i16.
   pub fn i16() -> i16 {
      let mut quatro_bytes= super::pega_n_bytes(2);
      let array = quatro_bytes.drain(0..);
      let mut bytes: [u8; 2] = [0; 2];
      for (i, b) in array.enumerate() 
         { bytes[i] = b; }
      i16::from_le_bytes(bytes)
   }
   /// sorteia um u16.
   pub fn u16() -> u16 {
      let mut quatro_bytes= super::pega_n_bytes(2);
      let array = quatro_bytes.drain(0..);
      let mut bytes: [u8; 2] = [0; 2];
      for (i, b) in array.enumerate() 
         { bytes[i] = b; }
      u16::from_be_bytes(bytes)
   }
   /// sorteia um u32.
   pub fn u32() -> u32 {
      let mut quatro_bytes= super::pega_n_bytes(4);
      let array = quatro_bytes.drain(0..);
      let mut bytes: [u8; 4] = [0; 4];
      for (i, b) in array.enumerate() 
         { bytes[i] = b; }
      u32::from_be_bytes(bytes)
   }
   /// sorteia um i32.
   pub fn i32() -> i32 {
      let mut quatro_bytes= super::pega_n_bytes(4);
      let array = quatro_bytes.drain(0..);
      let mut bytes: [u8; 4] = [0; 4];
      for (i, b) in array.rev().enumerate() 
         { bytes[i] = b; }
      i32::from_be_bytes(bytes)
   }
   /// sorteia um i64.
   pub fn i64() -> i64 {
      let mut oito_bytes = super::pega_n_bytes(8);
      let array = oito_bytes.drain(0..);
      let mut bytes: [u8; 8] = [0; 8];
      for (i, b) in array.enumerate() 
         { bytes[i] = b; }
      i64::from_le_bytes(bytes)
   }
   /// sorteia um u64.
   pub fn u64() -> u64 {
      let mut oito_bytes = super::pega_n_bytes(8);
      let array = oito_bytes.drain(0..);
      let mut bytes: [u8; 8] = [0; 8];
      for (i, b) in array.enumerate() 
         { bytes[i] = b; }
      u64::from_be_bytes(bytes)
   }
   /// sorteia um 'usize' e 'isize'.
   pub fn usize(intervalo: Zplus) -> usize {
      let a = *intervalo.start() as u64;
      let b = *intervalo.end() as u64;
      if a > b
         { return usize((b as usize)..=(a as usize)); }
      (a + u64() % ((b + 1) - a)) as usize
   }
   /* inteiros positivos e negativos. */
   pub fn isize(intervalo: Zminus) -> isize {
      let a = *intervalo.start();
      let b = *intervalo.end();
      let c: isize = {
         if a >= 0 && b >= 0
            { b - a }
         else if a < 0 && b >= 0
            { b + a.abs() }
         else 
            { b.abs() + a.abs() }
      };
      let x = i64().abs() as isize;
      a + x % (c + 1)
   }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;

   #[test]
   fn FuncaoBooleana() {
      let mut percentual: f32 = 0.0;
      let total: f32 = 80_000f32;

      for _ in 1..=(total as usize) {
         if !randomico::bool()
            { percentual += 1.0/total; }
      }
      assert!((dbg!(percentual)-0.50).abs() <= 0.01);
      assert!((percentual-0.50).abs() <= 0.005);
   }

   #[test]
   fn Aleatoriou8() {
      let mut p1alg: f32 = 0.0;
      let mut p2alg: f32 = 0.0;
      let mut p3alg: f32 = 0.0;
      let total: f32 = 80_000f32;

      for _ in 1..=(total as usize) {
         match randomico::u8() {
            0..=9 =>
               { p1alg += 1.0/total; }
            10..=99 =>
               { p2alg += 1.0/total; }
            _ => 
               { p3alg += 1.0/total; }
         };
      }

      println!(
         "\num algarismo:{:>3.2}%
         \rdois algarismos:{:>3.2}%
         \rtrês algarismos:{:>3.2}%
         ", p1alg * 100.0, 
         p2alg * 100.0,
         p3alg * 100.0
      );
      // todos têm que está em tal intervalo.
      assert!((p1alg-0.0390).abs() <= 0.005);
      assert!((p2alg-0.3515).abs() <= 0.005);
      assert!((p3alg-0.6093).abs() <= 0.005);
   }

   #[test]
   fn Aleatorioi8() {
      let mut p1alg: f32 = 0.0;
      let mut p2alg: f32 = 0.0;
      let mut p3alg: f32 = 0.0;
      let total: f32 = 80_000f32;
      let mut negativos: f32 = 0.0;

      for _ in 1..=(total as usize) {
         let valor = randomico::i8();
         if valor < 0
            { negativos += 1.0/total; }
         match valor {
            -9..=9  =>
               { p1alg += 1.0/total; }
            10..=99 | -99..=-10 =>
               { p2alg += 1.0/total; }
            100..=127 | -128..=-100  => 
               { p3alg += 1.0/total; }
         };
      }

      println!(
         "\num algarismo:{:>3.2}%
         \rdois algarismos:{:>3.2}%
         \rtrês algarismos:{:>3.2}%
         \n\r\tpositivos:{:<2.3}%
         \r\tnegativos:{:<2.3}%\n",
         p1alg * 100.0, 
         p2alg * 100.0,
         p3alg * 100.0,
         (1.0-negativos)*100.0,
         negativos * 100.0
      );
      // todos têm que está em tal intervalo.
      assert!((p1alg-0.0742).abs() <= 0.005);
      assert!((p2alg-0.7031).abs() <= 0.005);
      assert!((p3alg-0.2226).abs() <= 0.005);
   }
}
