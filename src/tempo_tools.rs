

/*! Ferramentas comuns aos demais módulos. */

use std::time::{Duration, Instant};
use std::ops::Deref;


// para registrar o tempo de "forma limpa".
pub struct Cronometro { ti: Instant }

#[allow(dead_code)]
impl Cronometro {
   pub fn cria() -> Self 
      { Self { ti: Instant::now() } }
   pub fn reseta(&mut self)
      { self.ti = Instant::now(); }
   pub fn marca(&self) -> u64
      { self.ti.elapsed().as_millis() as u64 }
}

// temporizador.
pub struct Temporizador { 
   // tempo à cumprir.
   inicio: Duration, 
   // contador.
   cronometro: Instant,
   // informador do estado do temporizador.
   esgotado: bool,
}

#[allow(dead_code)]
impl Temporizador {
   pub fn cria(inicio: Duration) -> Self { 
      Self {
         inicio, 
         cronometro: Instant::now(),
         esgotado: false,
      }
   }
   pub fn decorrido(&mut self) -> u64 {
      let passado = self.cronometro.elapsed();
      // verifica término ...
      if passado >= self.inicio
         { self.esgotado = true; }
      if self.esgotado 
         { return self.inicio.as_secs() as u64; }
      return passado.as_millis() as u64;
   }
   // um milésimo do valor retornado do método acima.
   pub fn decorrido_seg(&mut self) -> u64 
      { self.decorrido() / 1000 }
   // tempo dado inicialmente em segundos.
   pub fn meta(&self) -> u64
      { self.inicio.as_secs() }
}

// informa se o temporizador se esgotou.
impl Deref for Temporizador {
   type Target = bool;
   /* valor lógico dizendo se o 
    * temporizador está/ou não esgotado.
    */
   fn deref(&self) -> &Self::Target 
      { return &self.esgotado; }
}

