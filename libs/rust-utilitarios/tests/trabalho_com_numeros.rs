

/* tabelação da tradução de milhares de 
 * números que serão representados em:
 * cardinal, por-extenso e romano. Todas
 * ferramentas disponíveis na 'lib'.
 */

extern crate utilitarios;
use utilitarios::{
   por_extenso::escreve_por_extenso,
   romanos::decimal_para_romano,
   tabelas::{Coluna, Tabela},
   aleatorio::randomico
};



#[test]
#[allow(non_snake_case)]
fn tabelaComConversoes() {
   let mut cardinais: Vec<usize> = Vec::new();
   let mut escritos: Vec<String> = Vec::new();
   let mut romanos: Vec<String> = Vec::new();

   // gerando 'rol de dados'.
   for _ in 1..=80 {
      let X = randomico::usize(0..=2000);
      cardinais.push(X);
      let extenso = escreve_por_extenso(X as u64).unwrap();
      escritos.push(extenso);
      romanos.push(decimal_para_romano(X as u16));
   }

   // criando colúnas das tabelas.
   let coluna1 = Coluna::nova("cardinais", cardinais);
   let coluna2 = Coluna::nova("números por-extenso", escritos);
   let coluna3 = Coluna::nova("números romanos", romanos);
   
   // criando a tabela.
   let mut tabela = Tabela::nova(false);
   tabela.adiciona(coluna1);
   tabela.adiciona(coluna3);
   tabela.adiciona(coluna2);
   // visualizando o resultado.
   println!("{}", tabela);

   // avaliação manual.
   assert!(true);
}
