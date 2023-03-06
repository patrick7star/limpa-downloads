

/* método clássico: onde a volatilidade
 * vem do menor registro de tempo 
 * que pode ser feito.
 */

// biblioteca Rust:
use std::ops::RangeInclusive;
use std::time::SystemTime;

/* função efeitua um "lançamento de moeda" levando
 * em conta repetição de dezenas de vezes da medição
 * do relógio do sistema, isto após ter somado a 
 * medição várias vezes, após isso pega a unidade
 * do número medido porque é a parte mais volátil
 * do número, principalmente depois de somado 
 * várias vezes. */
fn lancamento() -> bool {
   // contador de tempo decorrido.
   let cronometro = SystemTime::now();
   let mut nanoseg: u64 = 0;

   /* soma todos tempos decorridos para que
    * unidades de tal valor, varie alucinadamente,
    * porque é com tal elemento que decidimos 
    * o valor estocástico gerado. */
   for _ in 1..=50 {
      let d = cronometro.elapsed().unwrap();
      nanoseg += d.as_nanos() as u64; 
   }

   // extraíndo unidade do número, pois é muito volátil.
   let unidade:char = {
      nanoseg
      .to_string()
      .pop()
      .unwrap()
   };
   /* metade dos "números" resultam em 'verdadeiro'
    * a outra o inverso. */
   match unidade {
      '0' => false,
      '1' => true,
      '2' => false,
      '3' => true,
      '4' => false,
      '5' => true,
      '6' => false,
      '7' => true,
      '8' => false,
      '9' => true,
      _ => 
         { panic!("não obtido um número!"); }
   }
}


/**
 compila todas funções em uma só, que poder emitir
 quaisquer tipos num só módulo, onde têm funções com
 nomes parecidos dos tipos, assim invokar tal função 
 parece o tipo acompanhado de "()". As funções no
 só chamam as funções acimas que já trabalham na parte
 de gerar tais coisas aleatórias, e algumas também
 implemenetam 'Range' que as funções acimas foram deixas
 de fora nisso. 
*/
pub mod sortear {
   // importando do módulo "pai".
   use super::{RangeInclusive};

   /// gera um valor booleano randômico.
   pub fn bool() -> bool 
      { super::lancamento() }

   /// gera valor inteiro positivo de 8-bits(0 à 255).
   pub fn u8(intervalo:RangeInclusive<u8>) -> u8 {
      // apelidos para melhor legibilidade.
      let a = *intervalo.start();
      let b = *intervalo.end();

      /* se não estiver dentro do limite, sortear 
      * até que esteja. */
      let mut x:u8 = 0;
      // last-endian ...
      for e in 0u32..8u32 {
         // positivo como se fosse o um(1).
         if bool()   
            { x += 2u8.pow(e); }
      }

      // retorna número sorteado.
      if a > 0 && b <= u8::MAX
         { (x % ((b - a) + 1)) + a }
      else if a == 0 && b < u8::MAX
         { x % (b + 1) }
      else { x }
   }

   /// gera um inteiro de 8-bits de modo randômico.
   pub fn i8(intervalo:RangeInclusive<i8>) -> i8 {
      /* o código todo é uma cópia da implementação 
       * do 16-bits, apenas trocando os tipos. Portanto,
       * todos comentários detalhando cada passo, 
       * estão lá. */
      let a = *intervalo.start();
      let b = *intervalo.end();
      if a >= 0 && b <= i8::MAX { 
         let a = a as u8;
         let b = b as u8;
         u8(a..=b) as i8 
      } else if (a < 0 && b <= -1) && a >= i8::MIN  { 
         /* o valor inicial do intervalo tem que
          * seguir a ordem matemática. */
         if a > b {
            let msg_i = "erro matemático no intervalo";
            let msg_ii = format!("{} < {}(Errado!)", a, b);
            let msg_iii = format!("o correto é {} > {}", a, b);
            let msg_iv = format!("logo {}..={} é o correto", b, a);
            panic!(
               "{}: {}, {}, {} a se tentar.", 
               msg_i, msg_ii, 
               msg_iii, msg_iv
            );
         }
         if a == i8::MIN
            { (-1) * i8(b.abs()..=(a+1).abs()) -1 }
         else
            { (-1) * i8(b.abs()..=a.abs()) }
      } else if a < 0 && b >= 0 {
         let x:u8 = {
            if a == i8::MIN { 0 }
            else { a.abs() as u8 }
         };
         let y:u8 = b as u8;
         let t = x + y;
         let c:i8 = (u8(0..=u8::MAX) % t) as i8;
         a + c
      } else { 
         if !bool()
            { i8(-1..=-128) }
         else
            { i8(0..=127) }
      }
   }

   /// gera um inteiro positivo de 16-bits randômico.
   pub fn u16(intervalo:RangeInclusive<u16>) -> u16 { 
      // acumulador de potências.
      let mut soma:u16 = 0;
      // quantia de bits.
      let mut qtd:u32 = 16;
      // apelidos:
      let a = *intervalo.start();
      let b = *intervalo.end();

      // supondo formatação big-endian.
      while qtd > 0 {
         // positivo como se fosse o um(1).
         if bool()   
            {soma += 2u16.pow(qtd-1); }
         // somar zero é irrelevante, mas fica aqui
         // para manter a lógica.
         else {}
         qtd -= 1;
      }

      // corrigindo nos intervalos.
      if a > 0 && b <= u16::MAX
         { (soma % ((b - a) + 1)) + a }
      else { soma }
   }

   /// gera um inteiro de 16-bits de modo randômico.
   pub fn i16(intervalo:RangeInclusive<i16>) -> i16 {
      // apelidos para legibilidade:
      let a = *intervalo.start();
      let b = *intervalo.end();
      /* usando a função que gera um positivo do tipo
       * pois tal valor desta, está contido nele,
       * com uma bela "matemágica" aritimética. */
      // corrigindo nos intervalos.
      if a >= 0 && b <= i16::MAX { 
         let a = a as u16;
         let b = b as u16;
         u16(a..=b) as i16 
      }
      /* um intervalo é negativo, o mesmo, porém
       * módulo dos extremos trocados. */
      else if (a < 0 && b <= -1) && a >= i16::MIN  { 
         /* o valor inicial do intervalo tem que
          * seguir a ordem matemática. */
         if a > b {
            let msg_i = "erro matemático no intervalo";
            let msg_ii = format!("{} < {}(Errado!)", a, b);
            let msg_iii = format!("o correto é {} > {}", a, b);
            let msg_iv = format!("logo {}..={} é o correto", b, a);
            panic!(
               "{}: {}, {}, {} a se tentar.", 
               msg_i, msg_ii, 
               msg_iii, msg_iv
            );
         }
         if a == i16::MIN
            { (-1) * i16(b.abs()..=(a+1).abs()) -1 }
         else
            { (-1) * i16(b.abs()..=a.abs()) }
      }
      // intervalo vária do negativo ao positivo.
      else if a < 0 && b >= 0 {
         /* apelidos para simplificar codificação e 
          * legibilidade do código futuramente. */
         let x:u16 = {
            if a == i16::MIN { 0u16 }
            else { a.abs() as u16 }
         };
         let y:u16 = b as u16;
         // computando o tamanho do intervalo.
         let t = x + y;
         /* computando um valor randômico delimitando
          * o resultado ao intervalo. */
         let c:i16 = (u16(0..=u16::MAX) % t) as i16;
         /* o resultado é um acrescimo de um tamanho
          * entre o tamanho máximo do intervalo, 
          * obtido de forma randômica. */
         return a + c;
      } 
      /* último caso leva em conta todo intervalo
       * distribuindo os positivos e negativos
       * de forma equitativa. */
      else { 
         if !bool()
            { i16(-1..=i16::MIN) }
         else
            { i16(0..=i16::MAX) }
      }
   }

   /// gera um inteiro positivo de 32-bits randômico.
   pub fn u32(intervalo:RangeInclusive<u32>) -> u32 {
      // acumulador de potências.
      let mut soma:u32 = 0;
      // quantia de bits.
      let mut qtd:u32 = 32;
      // apelidos:
      let a = *intervalo.start();
      let b = *intervalo.end();
      // supondo formatação big-endian.
      while qtd > 0 {
         // positivo como se fosse o um(1).
         if bool()   
            {soma += 2_u32.pow(qtd-1); }
         // somar zero é irrelevante, mas fica aqui
         // para manter a lógica.
         else {}
         qtd -= 1;
      }

      // corrigindo nos intervalos.
      if a > 0 && b <= u32::MAX
         { (soma % ((b - a) + 1)) + a }
      else { soma }
   }

   /// gera um inteiro de 32-bits(4 bytes) de modo randômico.
   pub fn i32(intervalo:RangeInclusive<i32>) -> i32 {
      /* o código comentando detalhe por detalhe 
       * sobre o código abaixo está ou na implmenetação
       * do i16 ou i8. Sim, o código é apenas uma cópia
       * deste, mundando a tipagem. */
      let a = *intervalo.start();
      let b = *intervalo.end();

      if a >= 0 && b <= i32::MAX { 
         let a = a as u32;
         let b = b as u32;
         u32(a..=b) as i32 
      }
      else if (a < 0 && b <= -1) && a >= i32::MIN  { 
         if a > b {
            let msg_i = "erro matemático no intervalo";
            let msg_ii = format!("{} < {}(Errado!)", a, b);
            let msg_iii = format!("o correto é {} > {}", a, b);
            let msg_iv = format!("logo {}..={} é o correto", b, a);
            panic!(
               "{}: {}, {}, {} a se tentar.", 
               msg_i, msg_ii, 
               msg_iii, msg_iv
            );
         }

         if a == i32::MIN
            { (-1) * i32(b.abs()..=(a+1).abs()) -1 }
         else
            { (-1) * i32(b.abs()..=a.abs()) }
      }
      else if a < 0 && b >= 0 {
         let x:u32 = {
            if a == i32::MIN { 0u32 }
            else { a.abs() as u32 }
         };
         let y:u32 = b as u32;
         let t = x + y;
         let c:i32 = (u32(0..=u32::MAX) % t) as i32;

         return a + c;
      } 
      else { 
         if !bool()
            { i32(-1..=i32::MIN) }
         else
            { i32(0..=i32::MAX) }
      }
   }
   
   /// gera um inteiro positivo de 64-bits(8 bytes) randômico.
   pub fn u64(intervalo:RangeInclusive<u64>) -> u64 {
      // acumulador de potências.
      let mut soma:u64 = 0;
      // quantia de bits.
      let mut qtd:u32 = 64;
      // apelidos:
      let a = *intervalo.start();
      let b = *intervalo.end();
      // supondo formatação big-endian.
      while qtd > 0 {
         // positivo como se fosse o um(1).
         if bool()   
            {soma += 2u64.pow(qtd-1); }
         qtd -= 1;
      }
      // corrigindo nos intervalos.
      if a > 0 && b <= u64::MAX
         { (soma % ((b - a) + 1)) + a }
      else if a == 0 && b <= u64::MAX
         { soma % (b + 1)  }
      else { soma }
   }

   /// gera um inteiro de 64-bits(8 bytes) de modo randômico.
   pub fn i64(intervalo:RangeInclusive<i64>) -> i64 {
      /* o código comentando detalhe por detalhe 
       * sobre o código abaixo está ou na implmenetação
       * do i16 ou i8. Sim, o código é apenas uma cópia
       * deste, mundando a tipagem. */
      let a = *intervalo.start();
      let b = *intervalo.end();

      if a >= 0 && b <= i64::MAX { 
         let a = a as u64;
         let b = b as u64;
         u64(a..=b) as i64 
      }
      else if (a < 0 && b <= -1) && a >= i64::MIN  { 
         if a > b {
            let msg_i = "erro matemático no intervalo";
            let msg_ii = format!("{} < {}(Errado!)", a, b);
            let msg_iii = format!("o correto é {} > {}", a, b);
            let msg_iv = format!("logo {}..={} é o correto", b, a);
            panic!(
               "{}: {}, {}, {} a se tentar.", 
               msg_i, msg_ii, 
               msg_iii, msg_iv
            );
         }

         if a == i64::MIN
            { (-1) * i64(b.abs()..=(a+1).abs()) -1 }
         else
            { (-1) * i64(b.abs()..=a.abs()) }
      }
      else if a < 0 && b >= 0 {
         let x:u64 = {
            if a == i64::MIN { 0u64 }
            else { a.abs() as u64 }
         };
         let y:u64 = b as u64;
         let t = x + y;
         let c:i64 = (u64(0..=u64::MAX) % t) as i64;

         return a + c;
      } 
      else { 
         if !bool()
            { i64(-1..=i64::MIN) }
         else
            { i64(0..=i64::MAX) }
      }
   }

   /// gera um intero de(quantia variada) bytes, randomicamente.
   pub fn usize(intervalo:RangeInclusive<usize>) -> usize {
      let (a, b) = (
         *intervalo.start() as u64,
         *intervalo.end() as u64
      );
      return u64(a..=b) as usize;
   }
}
