/*!  
 # Geração de números aleatórios
  Neste código vamos apresentar algumas funções
 que geram, números inteiros e flutuantes, 
 arrays inteiras e intervalos de números
 aleatórios.

  O algoritmo é simples, pega o clock ou 
 tempo no exato momento de execução, como geralmente
 é um número na casa das dezenas de milhares, 
 obtem seu último dígito, que vária muito, e
 soma com outros mais cem números obtidos pelo
 mesmo processo, e, novamente arranca-se o
 valor unitário do número, que sempre varia
 de zero à dez. Em cima desta função que dá
 valores de 0 à 9 de forma randomizada, se
 constrói todo o ferramental de aleatoriaedade.
 
  Tais funções não foram feito para serem gerados
 em grandes escalas randomizadas, apenas um 
 em escalas de milisegundos a geração massissa
 estocástica funciona bem, portanto, um algoritmo
 extremamente lento.
*/

// complementar:
mod metodo_i;
mod metodo_ii;
mod impressao;

// complementar, porém é uma gama de testes:
mod testes_unitarios;

// re-exportando ...
pub use metodo_ii::sortear;
#[cfg(target_os="linux")]
pub use metodo_i::randomico;

// troca valores dentro de uma array genérica.
fn troca<A>(lista: &mut Vec<A>, p1: usize, p2:usize) {
   let remocao = lista.remove(p1);
   lista.insert(p2, remocao);
} 

/// embaralha a array referênciada.
pub fn embaralha<B>(array: &mut Vec<B>) {
   let mut tamanho = array.len();
   let ultimo: u64 = (tamanho - 1) as u64;

   // se houver apenas dois elementos, pode trocar ou não.
   if tamanho == 2 {
      if sortear::bool() 
         { troca(array, 0, 1); }
   } else if tamanho <= 1 {
      // apenas abandona o programa; nada a fazer.
      return ();
   } else {
      // faz o embaralho o "tamanho da lista" vezes.
      while tamanho > 0 {
         let i = sortear::u64(0..=ultimo) as usize;
         let j = sortear::u64(0..=ultimo) as usize;
         if j != i 
            { troca(array, i, j); }
         tamanho -= 1;
      }
   }
}


#[cfg(test)]
mod tests {
   #[test]
   #[should_panic]
   fn intervalo_passado_errado_u16() {
      /* argumento incorreto, pois na base
       * matemática o inteiro -13 é menor
       * que -5, logo o programa será 
       * interrompido! */
      super::sortear::i16(-5..=-13);
   }

   #[test]
   fn testa_embaralha() {
      let mut array = vec![1,2,3,4,5];
      let copia = array.clone();
      let mut p = 0.0;
      for _ in 1..=100 {
         super::embaralha(&mut array);
         println!("{:?}", array);
         if array != copia
            { p += 1.0/100.0; }
      }
      assert!(dbg!(p) > 0.95);
   }

   use std::thread::{JoinHandle, spawn};
   #[test]
   fn e_multi_thread() {
      let mut selecoes: Vec<u8> = Vec::new();
      let mut outra_selecoes: Vec<u8> = Vec::new();

      let thread_i: JoinHandle<Vec<u8>>;
      thread_i = spawn(move || {
         for _ in 1..=30_000 
            { outra_selecoes.push(super::sortear::u8(0..=100)); }
         outra_selecoes
      });

      for _ in 1..=30_000 
         { selecoes.push(super::sortear::u8(0..=100)); }

      match thread_i.join() {
         Ok(resultado) => {
            let iterador = selecoes.iter().zip(resultado);
            for (a, b) in iterador.rev().take(30)
               { println!("{} <==> {}", a, b); }
         } Err(_) => ()
      };

      assert!(false);
   }
}
