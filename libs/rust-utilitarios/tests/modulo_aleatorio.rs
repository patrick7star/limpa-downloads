
/*
 * Vamos testar ao máximo que poder como
 * se distribui as amostra de grandes 
 * quantias de valores gerados aleatóriamente
 * para ver se está tudo ok!
*/

extern crate utilitarios;
use utilitarios::aleatorio;
use std::time::{Duration, Instant};
use std::collections::HashSet;
use std::ops::RangeInclusive;
use utilitarios::aleatorio::sortear;
use std::thread;


#[test]
fn distribuicao_do_u8() {
   // amostras aleatórias:
   let mut vinte = 0;
   let mut duzentos_e_trinta_e_dois = 0;
   let mut sessenta_e_sete = 0;

   // total de lançamentos.
   let total = 25_000;
   for _ in 1..=total {
      // sorteio de 0 à 255.
      let numero = aleatorio::sortear::u8(0..=255);

      // contando eventos ...
      if numero == 20 
         { vinte += 1; }
      else if numero == 232
         { duzentos_e_trinta_e_dois += 1; }
      else if numero == 67
         { sessenta_e_sete += 1; }
   }
   
   /* porcentagens de todos tem que alcançar a
    * probabilidade global. */
   let pg:f32 = 1f32 / 256_f32;
   let p20 = (vinte as f32)/(total as f32);
   let p232 = (duzentos_e_trinta_e_dois as f32)/(total as f32);
   let p67 = (sessenta_e_sete as f32)/(total as f32);

   // margem de erro têm que estar em 20%
   let margem = dbg!((pg-p20).abs());
   assert!(margem <= 0.20);
   let margem = dbg!((pg-p232).abs());
   assert!(margem <= 0.20);
   let margem = dbg!((pg-p67).abs());
   assert!(margem.abs() <= 0.20);

   // margem de erro mais estreita: 5%.
   let margem = dbg!((pg-p20).abs());
   assert!(margem <= 0.05);
   let margem = dbg!((pg-p232).abs());
   assert!(margem <= 0.05);
   let margem = dbg!((pg-p67).abs());
   assert!(margem.abs() <= 0.05);

   // margem de$erro AINDA mais estreita: 1%.
   let margem = dbg!((pg-p20).abs());
   assert!(margem <= 0.01);
   let margem = dbg!((pg-p232).abs());
   assert!(margem <= 0.01);
   let margem = dbg!((pg-p67).abs());
   assert!(margem.abs() <= 0.01);
}

#[test]
fn distribuicao_do_i8() {
   // amostras aleatórias:
   let mut oitenta_e_oito = 0;
   let mut menos_quinze = 0;
   let mut cento_e_dois = 0;
   let mut menos_trinta_e_sete = 0;

   // total de lançamentos.
   let total = 105_000;
   for _ in 1..=total {
      // sorteio de 0 à 255.
      let numero = aleatorio::sortear::i8(-128..=127);

      // contando eventos ...
      if numero == 88 
         { oitenta_e_oito += 1; }
      else if numero == -15
         { menos_quinze += 1; }
      else if numero == 102
         { cento_e_dois += 1; }
      else if numero == -37 
         { menos_trinta_e_sete += 1; }
   }
   
   /* porcentagens de todos tem que alcançar a
    * probabilidade global. */
   let pg:f32 = 1f32 / 256_f32;
   let p1 = (oitenta_e_oito as f32)/(total as f32);
   let p2 = (menos_quinze as f32)/(total as f32);
   let p3 = (cento_e_dois as f32)/(total as f32);
   let p4 = (menos_trinta_e_sete as f32)/(total as f32);

   // margem de erro têm que estar em 20%
   for erro in [0.20, 0.05, 0.01, 0.005] {
      let margem = dbg!((pg-p1).abs());
      assert!(margem <= dbg!(erro));
      let margem = dbg!((pg-p2).abs());
      assert!(margem <= erro);
      let margem = dbg!((pg-p3).abs());
      assert!(margem.abs() <= erro);
      let margem = dbg!((pg-p4).abs());
      assert!(margem.abs() <= erro);
   }
}
#[test]
#[ignore]
fn distribuicao_do_i8_parte_ii() {
   // distribuição igual.
   let mut positivo:f32 = 0f32;
   for _ in 1..=105_000 {
      if sortear::i8(-50..=50) > 0
         { positivo += 1f32/5000f32; }
   }
   for erro in [0.10, 0.05, 0.01, 0.005]
      { assert!(dbg!((positivo- 0.50).abs()) < dbg!(erro)); }
}

#[test]
fn distribuicao_bool() {
   let mut p:f32 = 0f32; 
   let total = 100_000;
   for _ in 1..=total {
      if sortear::bool() 
         { p += 1.0 / (total as f32); }
   }
   let verdadeiro = dbg!(p * 100.0);
   for erro in [10.0, 5.0, 1.0, 0.5] {
      let e = (verdadeiro-50.0).abs();
      assert!(e < dbg!(erro));
   }
}

// teste de desempenho de trechos de códigos diferentes.
fn u16_recursao(intervalo:RangeInclusive<u16>) -> u16 { 
   // renomeando ínicio e fim do intervalo ...
   let a = *intervalo.start();
   let b = *intervalo.end();
   // array estática contendo dois únicos bytes.
   let bytes:[u8; 2];

   /* facilitando o processo de computação com
    * números que requerem apenas um byte. */
   if b < 256 
      { bytes = [0, sortear::u8(0..=255)]; }
   else 
      { bytes = [sortear::u8(0..=255), sortear::u8(0..=255)]; }

   /* convertendo do big-endian bytes para um
    * inteiro positivo de 16-bits. */
   let numero = u16::from_be_bytes(bytes);
   
   // ajusta no intervalo.
   fn calibra(n:u16, a:u16, b:u16) -> u16 {
      if n < a 
         { calibra(a + n, a, b) }
      else if n > b 
         { calibra(n - b, a, b) }
      else { return n; }
   }

   return calibra(numero, a, b);
}

fn u16_condicional(intervalo:RangeInclusive<u16>) -> u16 { 
   // renomeando ínicio e fim do intervalo ...
   let a = *intervalo.start();
   let b = *intervalo.end();
   // array estática contendo dois únicos bytes.
   let bytes:[u8; 2];

   /* facilitando o processo de computação com
    * números que requerem apenas um byte. */
   if b < 256 
      { bytes = [0, sortear::u8(0..=255)]; }
   else 
      { bytes = [sortear::u8(0..=255), sortear::u8(0..=255)]; }

   /* convertendo do big-endian bytes para um
    * inteiro positivo de 16-bits. */
   let numero = u16::from_be_bytes(bytes);
   
   // ajusta no intervalo.
   fn calibra(n:u16, a:u16, b:u16) -> u16 {
      let d = b - a;
      if n < a && (n - 0) < d 
         { a + n }
      else if n < a && (n - 0) >= d 
         { a + (n % d) }
      else if n > b && (n - b) < d 
         { b - (n - b) }
      else if n > b && (n - b) >= d 
         { b - ((n - b) % d) }
      else { n }
   }

   return calibra(numero, a, b);
}

#[test]
fn testando_abordagens_diferentes() {
   // primeiro o que usa recursão.
   let mut cronometro = Instant::now();
   for _ in 1..=5000 {
      let x = u16_recursao(663..=666);
      assert!(x >=663 && x <= 666);
   }
   for _ in 1..=5000 {
      let x = u16_recursao(329..=350);
      assert!(x >=329 && x <= 350);
   }
   for _ in 1..=5000 {
      let x = u16_recursao(510..=620);
      assert!(x >=510 && x <= 620);
   }
   for _ in 1..=5000 {
      let x = u16_recursao(1_092..=2_100);
      assert!(x >=1_092 && x <= 2_100);
   }
   for _ in 1..=5000 {
      let x = u16_recursao(11_950..=21_950);
      assert!(x >=11_950 && x <= 21_950);
   }
   for _ in 1..=5000 {
      let x = u16_recursao(12_000..=32_000);
      assert!(x >=12_000 && x <= 32_000);
   }
   let tempo_i = cronometro.elapsed();

   // agora o da condicional ...
   cronometro = Instant::now();
   // mesmos testes:
   for _ in 1..=5000 {
      let x = u16_condicional(663..=666);
      assert!(x >=663 && x <= 666);
   }
   for _ in 1..=5000 {
      let x = u16_condicional(329..=350);
      assert!(x >=329 && x <= 350);
   }
   for _ in 1..=5000 {
      let x = u16_condicional(510..=620);
      assert!(x >=510 && x <= 620);
   }
   for _ in 1..=5000 {
      let x = u16_condicional(1_092..=2_100);
      assert!(x >=1_092 && x <= 2_100);
   }
   for _ in 1..=5000 {
      let x = u16_condicional(11_950..=21_950);
      assert!(x >=11_950 && x <= 21_950);
   }
   for _ in 1..=5000 {
      let x = u16_condicional(12_000..=32_000);
      assert!(x >=12_000 && x <= 32_000);
   }
   let tempo_ii = cronometro.elapsed();
   println!(
      "função com recursão:{:3.2}seg
      \rfunção com condicional:{:3.2}seg",
      tempo_i.as_secs_f32(), 
      tempo_ii.as_secs_f32() 
   );
   assert!(tempo_ii < tempo_i);
}

#[test]
#[ignore]
fn sorteando_todos_valores_de_u8() {
   /* testa todos os possíveis valores 
    * que podem ser sorteados. */
   // valores já sorteados.
   let mut sorteados:HashSet<u8> = HashSet::new();
   // registrar o tempo.
   let mut cronometro = Instant::now();
   // apenas termina quando o mais provável é sorteado.
   while sorteados.len() != 240 {
      // valor randômico gerado levando-se todo espectro.
      let x = sortear::u8(0..=255);
      if sorteados.contains(&x) 
         { continue; }
      else { 
         sorteados.insert(x); 
         // só passa deste ponto com "certa maturidade".
         if sorteados.len() < 230 
            { continue; }
         println!(
            "qtd.: {}\tdecorrido: {}seg", 
            sorteados.len(),
            cronometro.elapsed().as_secs()
         );
         // resetando contagem.
         cronometro = Instant::now();
      }
   }
}

#[test]
fn testa_sortear() {
   dbg!(sortear::u8(0..=u8::MAX)); 
   dbg!(sortear::u64(0..=u64::MAX));   
   dbg!(sortear::u32(0..=u32::MAX));   
   dbg!(sortear::u16(0..=u16::MAX));   
   dbg!(sortear::bool());
   // verificação manual.
   assert!(true);
}

#[test]
fn u32_dentro_da_faixa() {
   for _ in 1..=5_000 {
      let s = aleatorio::sortear::u32(0..=255);
      assert!(s <= 255);
      let s = aleatorio::sortear::u32(1_031..=38_529);
      assert!(s >= 1_031 && s <= 38_529);
      let s = aleatorio::sortear::u32(71_563..=71_692);
      assert!(s >= 71_563 && s <=71_692);
      let s = aleatorio::sortear::u32(2_032_003..=2_032_050);
      assert!(s >= 2_032_003 && s <=2_032_050);
   }
}

#[test]
fn u64_dentro_da_faixa() {
   for _ in 1..=5_000 {
      let s = aleatorio::sortear::u32(20..=145);
      assert!(s >= 20 && s <= 145);
      let i = (u64::MAX - 100)..=u64::MAX;
      let s = aleatorio::sortear::u64(i.clone());
      assert!(s >= *i.start() && s <= *i.end());
      let i:RangeInclusive<u64> = 30_392_109..=93_891_552;
      let s = aleatorio::sortear::u64(i.clone());
      assert!(s >= *i.start() && s <= *i.end());
   }
}
#[test]
fn u16_dentro_da_faixa() {
   // faixa simples codificada!
   for _ in 1..=5_000 {
      let s = aleatorio::sortear::u16(0..=255);
      assert!(s <= 255);
      let s = aleatorio::sortear::u16(1031..=38_529);
      assert!(s >= 1031 && s <= 38_529);
      let s = aleatorio::sortear::u16(1563..=1692);
      assert!(s >= 1563 && s <=1692);
      let s = aleatorio::sortear::u16(32_003..=32_050);
      assert!(s >= 32_003 && s <=32_050);
   }
}

#[test]
fn u8_dentro_da_faixa() {
   // faixa simples codificada!
   for _ in 1..=15_000 {
      let s = aleatorio::sortear::u8(0..=1);
      assert!(s == 0 || s == 1);
      let s = aleatorio::sortear::u8(10..=89);
      assert!(s >= 10 && s <= 89);
      let s = aleatorio::sortear::u8(153..=169);
      assert!(s >= 153 && s <=169);
      let s = aleatorio::sortear::u8(250..=255);
      assert!(s >= 250);
   }
}

#[test]
fn u8_selecao_variada() {
   let mut outro_tempo = false;
   for _ in 1..=100 { 
      let x = dbg!(sortear::u8(0..=10));
      assert!(x <= 10);  
      if outro_tempo { 
         thread::sleep(Duration::from_millis(300)); 
         outro_tempo = false;
      }
      else { 
         thread::sleep(Duration::from_millis(500)); 
         outro_tempo = true;
      }
   }
   // testado e comprovado.
   assert!(true);
   
   // diminuindo o intervalo do sorteio.
   for _ in 1..=100 { 
      let x = dbg!(sortear::u8(187..=203));
      assert!(x >= 187 && x <= 203);  
      if outro_tempo { 
         thread::sleep(Duration::from_micros(300)); 
         outro_tempo = false;
      }
      else { 
         thread::sleep(Duration::from_micros(500)); 
         outro_tempo = true;
      }
   }
   // testado e comprovado.
   assert!(true);

   // diminuindo o intervalo do sorteio.
   for _ in 1..=100 { 
      let x = dbg!(sortear::u8(4..=12));
      assert!(x >= 4 && x <= 12);  
      if outro_tempo { 
         thread::sleep(Duration::from_nanos(300)); 
         outro_tempo = false;
      }
      else { 
         thread::sleep(Duration::from_nanos(500)); 
         outro_tempo = true;
      }
   }
   // testado e comprovado.
   assert!(true);
}
