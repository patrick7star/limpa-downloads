

/* testes unitários serão passados para
 * aqui, pois estão tomando muito espaço
 * no arquivo 'metodo_ii'.
 */

use std::fmt::{
   Display, Formatter as Formato, 
   Result as Forma
};
use std::collections::HashMap;

type Info = HashMap<u8, (u32, f32)>;

#[allow(dead_code)]
struct Pool {
   qtd_e_media: Info,
   positivos: u32,
   limite: u32
}

#[allow(dead_code)]
impl Pool {
   fn total(&self) -> u32 {
      self.qtd_e_media.values()
      .map(|(q, _)| q).sum()
   }
   pub fn adiciona(&mut self, n: i32) {
      // verificando velocidades ...
      // assert_ne!(self.total(), 500_000);
      let t = 1.0 / (self.limite as f32);
      if n.is_negative()
         { self.positivos += 1; }
      match qtd_de_algs(n) {
         0..=10 => {
            let chave: u8 = qtd_de_algs(n) as u8;
            match self.qtd_e_media.get_mut(&chave) {
               Some(mut tupla) =>
                  { tupla.0 += 1; tupla.1 += t; }
               None => ()
            };
         } _ => 
            { panic!("sem valores!"); }
      };
   }
   pub fn novo(limite: u32) -> Self {
      let mut dicio = Info::new();
      for alg in 1u8..=10 
         { dicio.insert(alg, (0u32, 0f32)); }
      Self { qtd_e_media: dicio, positivos: 0, limite }
   }
   pub fn media(&self, qtd_algs: u8) -> f32 { 
      let chave = {
         self.qtd_e_media
         .get(&qtd_algs)
         .unwrap()
      };
      return chave.1;
   }
   pub fn algs(&self, qtd_algs: u8) -> u32 {
      let chave = {
         self.qtd_e_media
         .get(&qtd_algs)
         .unwrap()
      };
      return chave.0;
   }
}

fn qtd_de_algs(n: i32) -> u32
   { ((n.abs() as f32).log10().floor() + 1.0) as u32 }

impl Display for Pool {
   fn fmt(&self, f: &mut Formato<'_>) -> Forma {
      let t = self.limite as f32;
      let percentual_positivo = {
         let p = self.positivos as f32;
         let q = self.limite as f32;
         p / q
      };
      write!(
         f, "\num alg.:{0:>10.7}%({1}/{2})
         \rdois algs.:{3:>9.6}%({4}/{5})
         \rtrês algs.:{6:>8.5}%({7}/{8})
         \rquatro algs.:{9:7.4}%({10}/{11})
         \rcinco algs.:{12:6.3}%({13}/{14})
         \rseis algs.:{15:5.2}%
         \rsete algs.:{16:5.2}%
         \roito algs.:{17:5.2}%
         \rnove algs.:{18:5.2}%
         \rdez algs.:{19:5.2}%
         \rpositivos({20:0.2}%) negativos({21:0.2}%)
         \rtotal: {22}({23})\n",
         // um algarismo(percentual, total, valor-esperado).
         100.0 * self.media(1), self.algs(1), 
         (self.media(1) * t).trunc() as usize,
         // dois algarismos.
         100.0 * self.media(2), self.algs(2),
         (self.media(2) * t).trunc() as usize,
         // três algarismos.
         100.0 * self.media(3), self.algs(3),
         (self.media(3) * t).trunc() as usize,
         // quatro algarismos.
         100.0 * self.media(4), self.algs(4),
         (self.media(4) * t).trunc() as usize,
         // cinco algarismos.
         100.0 * self.media(5), self.algs(5),
         (self.media(5) * t).trunc() as usize,
         // seis algarismos.
         100.0 * self.media(6), 
         100.0 * self.media(7),
         100.0 * self.media(8), 
         100.0 * self.media(9),
         100.0 * self.media(10), 
         percentual_positivo * 100.0,
         (1.0 - percentual_positivo) * 100.0,
         self.total(), self.limite
      )
   }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   use std::collections::HashSet;
   use super::super::metodo_i::randomico;
   use super::super::impressao::PP;
   use std::time::Duration;

   #[test]
   fn GeraTodos_u16() {
      // conjunto com todos inteirios.
      let mut universo: HashSet<u16>;
      universo = HashSet::new();
      // conta quantia de chamadas.
      let mut contador = 0;
      let mut instancia = PP::novo(Duration::from_secs(1));
      let total = (2_i32.pow(16)-1) as usize;
      // sortea até pega todos inteiros possíveis do tipo.
      while universo.len() != total {
         let sorteio = randomico::u16();
         universo.insert(sorteio);
         contador += 1;
         // informação:
         let mensagem = format!(
            "adições até o momento: {}
            \ratual número: {}
            \rchamadas: {}",
            universo.len(), sorteio,
            contador
         );
         instancia.dispara(mensagem.as_str());
      }
      assert!(true);
   }

   #[test]
   #[ignore="consome muito tempo"]
   fn BoaDistribuicao_i32() {
      let mut monitor = PP::novo(Duration::from_secs(15));
      let mut urna = Pool::novo(5_510_000);

      for _ in 1..=5_510_000 {
         urna.adiciona(randomico::i32());
         monitor.dispara(urna.to_string().as_str());
      }
      // última visualizada(para o 'shotscreen').
      println!("{}", urna);
      // avaliação manual.
      assert!(false);
   }

   #[test]
   fn GeraTodosDemandados_usize() {
      for _ in 1..=250 
         { print!("{}, ", randomico::usize(0..=10)); }
      print!("\n");
      // avaliação manual.
      assert!(true);
      let mut ultimo: bool = false;
      let mut primeiro: bool = false;
      for _ in 1..=30_000 {
         let X = randomico::usize(32_327..=40_001);
         if X == 40_001
            { primeiro = true; dbg!(X); }
         else if X == 32_327
            { ultimo = true; dbg!(X); }
      }
      assert!(primeiro && ultimo);
   }

   #[test]
   fn RespeitaIntervalo_isize() {
      // faz 100 vezes para estressar em algum erro.
      for _ in 1..=100_000 {
         assert!(randomico::isize(932..=15_932) >= 932);
         assert!(randomico::isize(-1520..=15) >= -1520);
         assert!(randomico::isize(-311..=-100) >= -311);
      }
   }

   #[test]
   fn ExtremosTambemSorteados_isize() {
      // faz 100 vezes para estressar em algum erro.
      let mut primeiro = false;
      let mut ultimo = false;
      for _ in 1..=100_000 { 
         let X = randomico::isize(32..=15_932); 
         if X == 32 
            { primeiro = true; }
         else if X == 15_932
            { ultimo = true; }
      }
      assert!(primeiro && ultimo);
   }
}
