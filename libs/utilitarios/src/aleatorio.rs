/* neste código vamos apresentar algumas funções
 * que geram, números inteiros e flutuantes, 
 * arrays inteiras e intervalos de números
 * aleatórios.
 * O algoritmo é simples, pega o clock ou 
 * tempo no exato momento de execução, como geralmente
 * é um número na casa das dezenas de milhares, 
 * obtem seu último dígito, que vária muito, e
 * soma com outros mais cem números obtidos pelo
 * mesmo processo, e, novamente arranca-se o
 * valor unitário do número, que sempre varia
 * de zero à dez. Em cima desta função que dá
 * valores de 0 à 9 de forma randomizada, se
 * constrói todo o ferramental de aleatoriaedade.
 */

// biblioteca Rust:
use std::time::SystemTime;
use std::str::FromStr;
use std::ops::RangeInclusive;


fn char_do_meio(s:String) -> char {
   let mut meio = s.len()/2_usize;
   for ch in s.as_str().chars() {

      if meio == 0 { return ch; }

      meio -= 1;
   }
   return '-';
}

pub fn algarismo_aleatorio() -> u8{
   let tempo = SystemTime::now(); 
   let mut t_nanoseg:usize = 0;

   for _ in 1..100 {
      let tempo_agora = SystemTime::now();
      t_nanoseg += {
         tempo_agora
         .duration_since(tempo)
         .unwrap()
         .as_nanos() as usize
      }
   }

   let s = t_nanoseg.to_string();
   let _inteiro = match u64::from_str(s.as_str()) {
         Ok(valor) => valor,
         Err(_) => {panic!("erro ocorreu na conversão!");}
      };
   let ch = char_do_meio(s);
   return (ch as u8) - 48_u8;
}

fn valor_logico_aleatorio() -> bool {
   // se for maior que cinco retorna verdadeiro.
   if algarismo_aleatorio() >= 5 { true }
   // caso contrário falso.
   else { false}
}

fn numero_0_a_9_faixa(intervalo:RangeInclusive<u8>) -> u8 {
   // proposições:
   let p1 = *intervalo.start() <= 9;
   let p2 = *intervalo.end() <= 9;
   // renomeando atributos para facilitar:
   let (i, f):(u8, u8) = (
      *intervalo.start(), 
      *intervalo.end()
   );

   // caso valores estejam invertidos, apenas troque...
   if i > f { return numero_0_a_9_faixa(f..=i) }
   // caso não haja variação.
   else if i == f { return i; }
   // faixa dada tem que estar entre 0 e 9, corrigindo...
   else if !p1 || !p2 {
      // corrigindo ambos trazendo aos valores válidos.
      if p1 { return numero_0_a_9_faixa(i%10..=f); }
      if p2 { return numero_0_a_9_faixa(i..=f%10); }
   }
   // se for a faixa inteira, apenas retorna a função especializada nisto.
   else if i == 0 && f == 9 
      { return algarismo_aleatorio() }

   // dado um valor inicial.
   let mut inicial = algarismo_aleatorio();
   while !(inicial >=i && inicial <= f) {
      inicial = algarismo_aleatorio();
   }
   
   // retorna número aleatório limitado.
   return inicial;
}

fn numero_i8() -> i8 {
   /* no geral, tal valor tem como máximo
    * 127 e mínimo -128. Vamos aplicar o
    * método apenas para à parte positiva, já
    * que o negativo é a simetria, ou seja, em
    * metade dos casos aplicamos o sinal, assim
    * a distribuição fica igualitária.
    * Outra coisa que temos é, vamos reutilizar
    * a função geradora para 8-bits positivos. 
    * Más aquele vai de 0 à 256, ultrapassa até
    * a parte positiva? Sim, porém apenas aplicando
    * a divisão por dois  em todos, temos o mesmo
    * resultado, e, à parte negativa já respondemos*/
   // se o número tem sinal ou não.
   let sinal = valor_logico_aleatorio();
   // determinando um valor de baseado em 
   // função já criada: "numero_u8".
   let aux_valor:u8 = numero_u8();
   let valor:i8;
   if aux_valor > 127 
      { valor = (aux_valor % 128) as i8; }
   else { valor = aux_valor as i8; }
   // levando primeiro em conta o sinal selecionado randômicamente.
   if sinal {
      /* com o sinal sendo positivo, então vamos 
       * deixar o valor como está. Não têm qualquer
       * relação com o sinal do valor em sí, por 
       * conveção escolhi deste modo. */
      valor
   }
   else {
      /* resolvendo caso de o u8 randômico não 
       * alcançar -128. Se alcançar o 0, metade
       * será reatribuído em -128, outra no valor
       * original. Fazemos isso usando novamente
       * "lançando uma moeda" sobre que tipo será
       * metade-metade de cada lado no final. */
       if valor == 0 { 
         match valor_logico_aleatorio() {
            true => i8::MIN,
            false => 0
         }
      }
      // se não for 127, apenas negativa o valor normalmente.
      else { (-1)*valor }
   }
}

// sortea um valor de um à noventa-e-nove.
fn um_a_noventa_e_nove() -> u8 {
   let tens = numero_0_a_9_faixa(1..=9);
   let ones = numero_0_a_9_faixa(0..=9);
   return tens*10 + ones;
}

fn numero_u8() -> u8 {
   /* distribuição de ciência:
    * => 10 números com 1 algarismo,
    * portando 4% da amostra:
    *    0, 1, 2, 3, 4, ... ,9
    * => 90 números com 2 algarismos,
    * portando 35% da amostra:
    *    10, 11, 12,..., 97, 98, 99
    * => 156 números com 3 algarismos,
    * portanto o restante 61%.
    *    100, 101,102,..., 253, 254, 255.
   */
   let x = um_a_noventa_e_nove();
   let (a1, a2, a3):(u8, u8, u8);
   if x >= 10 && x <= 10+4 
      { return algarismo_aleatorio(); }
   else if x >= 15 && x <= 15+31 { 
      a1 = numero_0_a_9_faixa(1..=9)*10;
      a2 = numero_0_a_9_faixa(0..=9);
      a3 = 0;
      return a1 + a2 + a3;
   }
   else {
      a1 = numero_0_a_9_faixa(1..=2);
      if a1 == 1 {
         a2 = numero_0_a_9_faixa(0..=9);
         a3 = numero_0_a_9_faixa(0..=9);
      }
      else {
         a2 = numero_0_a_9_faixa(0..=5);
         if a2 == 5 {
            a3 = numero_0_a_9_faixa(0..=5);
         }
         else {
            a3 = numero_0_a_9_faixa(0..=5);
         }
      }
   }
   return a1*100 + a2*10 + a3;
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
   /* importando biblioteca importada no módulo
   * externo a este. */
   use super::RangeInclusive;

   /* implementação para valor booleano. Só chama
   * função antigo que faz, isso, por motivos de
   * compatibilidade com códigos que contam 
   * com as funções antigas, e preguiça de remexer
   * em tudo para qualquer coisa não quebrar. */
   pub fn bool() -> bool
      { super::valor_logico_aleatorio() }

   /* O mesmo com inteiros positivos de 8-bits.
   * Apenas chama a função que já fazia, ao invés
   * de mexer na original(retro-compatibilidade).
   * Como a antiga não tive tempo de implemenetar,
   * mas agora tenho... vamos usar intervalos para
   * selecionar uma faixa onde o valor tem que 
   * ser selecionado. */
   pub fn u8(intervalo:RangeInclusive<i16>) -> u8 {
      // apelidos para melhor legibilidade.
      let i = intervalo;
      let a = *i.start();
      let b = *i.end();

      // se não estiver no devido intervalo, dá erro!
      if !(a >= 0 && b <= 255)  
         { panic!("fora do intervalo permitido para o tipo: 0..255"); }

      /* se não estiver dentro do limite, sortear 
      * até que esteja. */
      let mut x:u8 = super::numero_u8();
      while !(x >= (a as u8) && x <= (b as u8)) 
         { x = super::numero_u8(); }

      // retorna número sorteado.
      return x;
   }

   /* Agora para inteiros de 8-bits que permitem
   * números negativos. Mesmo esquema(retrocompatibilidade)
   * e implementação do 'Range' para delimitar
   * o sorteio. */
   pub fn i8(intervalo:RangeInclusive<i16>) -> i8 {
      // alias do interválo para propósito de codificação.
      let i = intervalo;
      let a = *i.start(); 
      let b = *i.end();
      if a >= -128  && b <= 127 {
         let mut x:i8 = super::numero_i8();
         /* se não estiver dentro do limite, sortear 
         * até que esteja. */
         while !(x >= (a as i8) && x <= (b as i8)) 
            { x = super::numero_i8(); }
         return x;
      }
      else 
         { panic!("fora do intervalo para o tipo: -128..127"); }
   }

   // Será futuramente implementado.
   fn _numero_u16() -> u16 { 
      /* Mesmo acima, a distribuição é a
       * seguinte:
       * -- 55,536 números são de 5 algarismos.
       *    ou seja, estamos falando de 84,7% de
       *    de todos números.
       * -- 9,000 números contém 4 algarismos,
       *    que representa aproximadamente 13,7%
       *    de todos os ~65mil números.
       * -- 900 números são de 3 algarismos, que
       *    totaliza percentualmente em 1,37%(
       *    arredondando para 1,4%) na distribuição 
       *    de números.
       * -- 90 contendo apenas 2 algarismos, este
       *    está entre 0,14% dos números.
       * -- 10 contendo apenas 1 algarismos. Quase
       *    nada, só 0,015% dos números. */
       0_u16
   }
}

// TESTES:
#[cfg(test)]
mod tests {
   use crate::aleatorio::*; 

   #[test]
   fn um_simples_algarismo_aleatorio() { 
      println!("==>{}",super::algarismo_aleatorio()); 
   }

   #[test]
   //#[ignore]
   fn mil_numeros_aleatorios() {
      for _i in 1..=1000 { 
         print!("{}, ", super::algarismo_aleatorio());
      }
   }

   #[test]
   //#[ignore]
   fn verificando_faixas() {
      for _i in 1..=5 {
         println!("de 0 até 5: {}", numero_0_a_9_faixa(0..=5))
      }
      println!("\n");
      for _i in 1..=5 {
         println!("de 6 até 9: {}", numero_0_a_9_faixa(6..=9))
      }
      println!("\n");
      for _i in 1..=5 {
         println!("de 0 até 1: {}", numero_0_a_9_faixa(0..=1))
      }
      println!("\n");
      for _i in 1..=5 {
         println!("de 1 até 8: {}", numero_0_a_9_faixa(8..=1))
      }
   }

   const N:usize = 4_000;

   fn lanca_n_i8() -> [i8; N] {
      // 100 mil elementos.
      let mut array:[i8; N] = [0;N];
      // sorteando 100 mil números aleatorios.
      for i in 0..N { array[i] = numero_i8(); }
      return array;
   }
   fn lanca_n_u8() -> [u8; N] {
      // 100 mil elementos.
      let mut array:[u8; N] = [0;N];
      // sorteando 100 mil números aleatorios.
      for i in 0..N { array[i] = numero_u8(); }
      return array;
   }
   
   fn porcentagem_um_algarismo(amostras:&[u8; N]) -> f32 {
      let mut q:u32 = 0;
      for x in amostras.iter() {
         if (0..=9).contains(x) { q += 1; }
      }
      return (q as f32) / (N as f32);
   }
   fn porcentagem_dois_algarismos(amostras:&[u8; N]) -> f32 {
      let mut q:u32 = 0;
      for x in amostras.iter() {
         if (10..=99).contains(x) { q += 1; }
      }
      return (q as f32) / (N as f32);
   }
   fn porcentagem_tres_algarismos(amostras:&[u8; N]) -> f32 {
      let mut q:u32 = 0;
      for x in amostras.iter() {
         if (100..=255).contains(x) { q += 1; }
      }
      return (q as f32) / (N as f32);
   }

   #[test]
   //#[ignore]
   fn teste_u8_randomico() {
      // 10 inteiros de 8 bits.
      for _x in 1..10+1 
         { println!("8 bits: {}",super::numero_u8());}
      println!("\n"); // pula duas linhas(espaçamento).

      // array com 100 mil amostras.
      let valores = lanca_n_u8();
      let p1:f32 = porcentagem_um_algarismo(&valores);
      let p2:f32 = porcentagem_dois_algarismos(&valores);
      let p3:f32 = porcentagem_tres_algarismos(&valores);
      println!("0 à 9: {:0.2}%", p1*100.0);
      println!("10 à 99: {:0.2}%", p2*100.0);
      println!("100 à 255: {:0.2}%", p3*100.0);
      /* a composição tem que ser 100% ou perto disso,
       * com uma margem de erro de 0.04 pts. */
      assert!((p1+p2+p3) >= 0.96 && (p1 + p2 + p3) <= 1.04);
   }
   
   fn porcentagem_negativa(amostras:&[i8; N]) -> f32 {
      let mut contador:u32 = 0;
      for x in amostras.iter() { 
         if *x < 0 { contador += 1; }
      }
      return (contador as f32) / (N as f32);
   }
   #[test]
   fn teste_i8_randomico() {
      let amostra = lanca_n_i8();
      let p = porcentagem_negativa(&amostra);
      let q = 1.0000-p;
      println!(
         "\n\rparte positiva:{:>4.2}%
         \rparte negativa:{:>4.2}%",
         p*100.0, q*100.0
      );
      /* verifica se fica metade-metade, com 
       * precisão de 0.4 em margem de erro. */
      assert!(
         p >= 0.4960 && p <= 0.5040 ||
         q >= 0.4960 && q <= 0.5040 
      );
   }

   #[test]
   fn testa_sortear() {
      let booleano:bool = dbg!(sortear::bool());
      assert!(booleano || !booleano);
      let z_menos= dbg!(sortear::i8(-10..=10));
      assert!(z_menos >= -10 && z_menos <= 10);
      let z_mais= dbg!(sortear::u8(27..=189));
      assert!(z_mais >= 27 && z_mais <= 189);
   }
}
