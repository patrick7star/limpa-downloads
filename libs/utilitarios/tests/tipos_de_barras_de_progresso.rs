
/* teste conjunto de todos tipos de 
 * progressos criados.
 */
extern crate utilitarios;
use utilitarios::barra_de_progresso::*;
use std::time::{Duration, Instant};
use std::thread::sleep;

/* o primeiro tipo criado, e consequente,
 * o mais simples. */
#[test]
fn progresso_simples() {
   println!("a \"barra de progresso\" mais básica:\n\t'intervalo de quase um milhão'");
   // fim do progresso.
   let total:u64 = 792_382;
   // laço finito.
   for k in 1..(total+1) {
      let bp:String =  progresso(k, total);
      print!("\r{}", bp);
      drop(bp);
   }
   // sem quebra de linha, pois a função que
   // gera a string a mostrar, já cuida disto.
   assert!(true);
}

/* barra de progresso que mostra baseado em
 * dados a dinâmica da computabilidade. Ou 
 * seja, está aqui é um pouco mais complexo
 * em detalhar o progresso. */
#[test]
fn progresso_em_dados() {
   println!("a \"barra de progresso de dados\":");
   for k in 1..360_582 {
      let str_bp_data:String = { progresso_data(k as u64, 360_582+1) };
      print!("\r{}", str_bp_data);
      drop(str_bp_data);
   }
   assert!(true)
}

/* este tipo de barra aceito um logo dinâmico
 * em que o texto fica em movimento se ele não
 * couber inteiramente na tela; isto da direita
 * para esquerda */
#[test]
fn progresso_em_dados_tipo_download() {
   println!("a \"barra de progresso com rótulo\":");
   let mut texto:utilitarios::barra_de_progresso::Logo = {
      let slogan:&str = "GTA IX - New Gangs on the Street, Las Vegas";
      Logo::novo(slogan)
      .unwrap()
   };
   for k in 1..1_360_582+1 {
      // apelido para legibilidade.
      let s:String = texto.to_string();
      // extraindo texto do logo-dinâmico.
      let str_bp_data:String = {
         utilitarios
         ::barra_de_progresso
         ::progresso_data_rotulo(s.as_str(), k as u64, 1_360_582)
      };
      /* movendo logo, no tempo pre-determinado 
       * pelo programação. Acho que é meio segundo,
       * porém não importa, aqui só chama para que
       * se compute a translação do texto. */
      texto.movimenta_letreiro();
      print!("\r{}", str_bp_data);
      drop(str_bp_data);
   }
   assert!(true)
}

/* testando estrutura que processa sobre 
 * impressão para não gerar um monte de 
 * impressão por vez. */
#[test]
fn testa_progresso_percentual() {
   let total:u64 = 1_500_000;
   // instânciando...
   let mut pd = ProgressoPercentual::cria(total);

   println!("visualizando inicial:");
   println!("{}", pd);

   for k in 1..=total {
      // atualizando valor metade de cada maneira.
      if k < total / 2 
         { pd += k; }
      else 
         { pd = pd + k; }

      // suposta impressão contida.
      if let Some(string) = pd.imprime() 
         { print!("{}\n", string); } 
   }
   assert!(true);
}

#[test]
fn testa_progresso_temporal() {
   let total:u64 = 21_500_000;
   // instânciando...
   let mut pd = ProgressoTemporal::cria(total, 300);

   println!("visualizando inicial:");
   println!("{}", pd);

   for k in 1..=total {
       pd += k; 
      // suposta impressão contida.
      if let Some(string) = pd.imprime() 
         { print!("{}\n", string); } 
   }
   assert!(true);
}

#[test]
fn testa_temporizador_progresso() {
   let cronometro = Instant::now();
   let tf = Duration::from_secs(30);
   let mut ti = cronometro.elapsed();
   type Anotacao = fn(&str, Duration, Duration) -> String;
   let funcao:Anotacao = temporizador_progresso;
   while ti < tf { 
      let resultado = funcao("isso é um pequeno teste", ti, tf);
      println!("{}", resultado);
      // registrando nova decorrência ...
      ti = cronometro.elapsed();
      sleep(Duration::from_secs_f32(3.5));
   }
   let resultado = funcao("isso é um pequeno teste", ti, tf);
   println!("{}", resultado);
   // avaliação manual do teste.
   assert!(true);
}

#[test]
fn testa_temporizador_progresso_i() {
   let tf = Duration::from_secs(30);
   let ti = Duration::from_secs(19);
   type Anotacao = fn(&str, Duration, Duration) -> String;
   let funcao:Anotacao = temporizador_progresso;
   let ri = funcao("Quando um estranho chama", ti, tf);
   let rii = funcao("Era Uma Vez uma Chapéuzinho Vermelha",ti, tf);
   let riii = funcao("Roda Mortal, o Terror está de Volta",ti, tf);
   println!("{}", ri);
   println!("{}", rii);
   println!("{}", riii);
   // avaliação manual do teste.
   assert!(true);
}
