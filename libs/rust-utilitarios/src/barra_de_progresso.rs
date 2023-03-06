
/*! 
 # Progressos para todas situações
 Código que dá utilitário que retorna barra
de progresso, dados uma quantia atual e total
a ser atingida. 
*/


// biblioteca do Rust:
use super::legivel::tamanho;
use std::time::{Duration, Instant};
use std::fmt::{Display, Formatter, Result as Result_fmt};
use std::ops::{Add, AddAssign};

// própria biblioteca:
use crate::legivel;
use super::terminal_dimensao::{Largura, terminal_largura};

// restante do módulo(re-exportando...):
mod letreiro;
mod unitarios;
mod progressosimples;
pub use letreiro::*;
pub use progressosimples::*;

// símbolo que representa frações das barras e espaços vázios.
const COMPONENTE:&'static str ="#";
const VAZIO:&'static str=".";
const CAPACIDADE:u8 = 50;
const TEXTO_MAX:usize = 30;

/// Atalho para `ProgressoTemporal`.
pub type PT = ProgressoTemporal;
type Impressao = Option<String>;


// cálcula número de algarismos de dado número.
fn conta_algs(valor:usize) -> u8 {
   let mut d = valor as f32; 
   // contador de divisões.
   let mut contador: u8 = 0; 

   while d > 1.0 {
      // cada divisão por dez, conta um.
      d = d / 10.0;
      contador += 1;
   }

   // retorno a contagem contabilizando um ...
   return contador + 1;
}

/** Cálculos percentuais e, suas representação
 numérica. Deve ser mais usado para simples
 visualização do progresso, nada dinâmico, pois
 dependendo de como foi escrito pode usar muita
 memória, e o consumo desnecesário de  `CPU` é 
 certo.*/
#[deprecated(since="1.3.5", 
note="usado agora apenas internamente")]
pub fn progresso(qtd_atual:u64, qtd_total:u64) -> String {
   let percentagem:f32 = (qtd_atual as f32)/(qtd_total as f32);
   let percent_100:f32 = percentagem*100.0;
   // qtd. de algarismos que será alcançado o valor atual.
   let qtd_algs:usize = (conta_algs(qtd_total as usize)) as usize;

   // caso de erro.
   if percentagem > 1.0_f32 {
      panic!("os valores de atual supera o total!");
   }
   else if percentagem == 1_f32 {
      return format!(
         "{0:>espaco$} de {1} [{2}]{3:>5.1}%",
         qtd_atual, qtd_total, 
         cria_barra(1.0, CAPACIDADE), 
         percent_100,espaco=qtd_algs
      );
   }

   /* molde da string retornada representando por 
    * inteiro a barra de progresso. */
   let molde:String = format!(
      "{0:>espaco$} de {1} [{2}]{3:>5.1}%",
      qtd_atual, qtd_total, 
      cria_barra(percentagem, CAPACIDADE), 
      percent_100,espaco=qtd_algs
   );
   return molde;
}


/** Cálculos percentuais e, suas representação
 numérica. Mesmo que o acima, o consumo de 
 `CPU` é dispedício, não use em **loops**.
 */
#[deprecated(since="1.3.5", 
note="usado agora apenas internamente")]
pub fn progresso_data(qtd_atual:u64, qtd_total:u64) -> String {
   let percentagem:f32 = (qtd_atual as f32)/(qtd_total as f32);
   let percent_100:f32 = percentagem*100.0;

   // caso de erro.
   if percentagem > 1.0_f32 
      { panic!("os valores de atual supera o total!"); }
   else if percentagem == 1.00 {
      let total_bytes = tamanho(qtd_total, true);
      return format!(
         "{}/{} [{}] {}%\n",
         total_bytes,
         total_bytes,
         cria_barra(1.0, CAPACIDADE),
         100.0
      );
   }
   else {
      // strings dos valores.
      let qa = tamanho(qtd_atual, true);
      let qt = tamanho(qtd_total, true);
      let qtd_algs = qt.len() as usize;
      let molde:String = format!(
         "{0:>espaco$}/{1} [{2}]{3:>5.1}%",
         qa, qt, cria_barra(percentagem, CAPACIDADE), 
         percent_100,espaco=qtd_algs
      );
      return molde;
   }
}


/** progresso com rótulo dinâmico, então quando
 o rótulo é muito maior do que cabe na tela, ele
 move o rótulo tipo aqueles slogans de neon em
 mercearias e shops. Ele também é de dados, o 
 resto do seu núcleo.
*/
#[deprecated(
  since="1.3.5", 
  note="confuso, e raramente utilizado"
)]
pub fn progresso_data_rotulo<'a>(rotulo:&'a str, 
  qtd_atual:u64, qtd_total:u64) -> String 
{
   // cálculando a porcetagem.
   let percentagem:f32 = (qtd_atual as f32)/(qtd_total as f32);
   // caso de erro.
   if percentagem > 1.0_f32 {
      panic!("os valores de atual supera o total!");
   }
   else if percentagem == 1f32 {
      let total_bytes = tamanho(qtd_total, true);
      return format!(
         "[{}] {}/{} [{}] {}%\n",
         rotulo,
         total_bytes,
         total_bytes,
         cria_barra(1.0, CAPACIDADE),
         100.0
      );
   }
   else {
      // strings dos valores.
      let qa = tamanho(qtd_atual, true);
      let qt = tamanho(qtd_total, true);
      let qtd_algs = qt.len() as usize;
      let molde:String = format!(
         "[{4}] {0:>espaco$}/{1} [{2}]{3:>5.1}%",
         qa, qt, cria_barra(percentagem, CAPACIDADE), 
         percentagem*100.0,
         rotulo,
         espaco=qtd_algs,
      );
      return molde;
   }
}

/** Um tipo de progresso que leva em conta,
 na hora de imprimir, a quantia de progresso
 ocorrido, e não apenas dá uma impressão dados
 a *quantia atual* e *total*. Tal estrutura é
 perfeita para uma grande entrada de dados.*/
pub struct ProgressoPercentual {
   // quantidade atual passada.
   qtd_atual:u64,
   // o total inicialmente marcado.
   qtd_total:u64,
   // percentual computado.
   percentual:f32,
   // antigo percentual para comparação.
   antigo_percentual:f32,
   // status da barra:
   pub esgotado: bool
}

// implementação do método.
impl ProgressoPercentual {
   // criando uma instância...
   pub fn cria(total:u64) -> Self {
      // criando dado agora...
      ProgressoPercentual {
         qtd_atual: 0,  
         qtd_total: total,
         percentual: 0.0f32,
         antigo_percentual: 0.0f32,
         esgotado: false
      }
   }

   /* Impressão limitada. Vê se houve um
    * aumento significativo, então dá o 
    * resultado, caso contrário, retorna
    * nenhum dado. */
   pub fn imprime(&mut self) -> Impressao {
      // diferença de percentuais.
      let diferenca = self.percentual - self.antigo_percentual;

      // verifica se há uma variação de 
      // no mínimo 0,5%.
      if diferenca >= 0.005 {
         // atualiza o 'antigo percentual' com o valor do "atual".
         self.antigo_percentual = self.percentual;
         // criando barra...
         let bp = progresso_i(self.qtd_atual, self.qtd_total);
         // retornando barra, porém embrulhado.
         return Some(bp);
      } else if self.qtd_atual == 0 {
         // não chamada nenhuma vez?
         let bp = progresso_i(self.qtd_atual, self.qtd_total);
         return Some(bp);
      } else if self.esgotado {
         Some(progresso_i(self.qtd_atual, self.qtd_total))
      } else { None }
   }
}

/* implementando adição para capturar 
 * novo valor 'qtd_atual'. */
impl Add<u64> for ProgressoPercentual {
   // tipo de retorno.
   type Output = ProgressoPercentual;
   /* pega um valor, e atualiza os atributos 
    *'qtd_atual' e dependentes destes para 
    * resultado final. Para isto, retorna
    * a própria instância. */
   fn add(mut self, qtd_atual:u64) -> Self::Output {
      // atualizando valores...
      self.qtd_atual = qtd_atual;
      // computando percentual.
      self.percentual = {
         let x:f32 = self.qtd_atual as f32;
         let y:f32 = self.qtd_total as f32;
         x / y
      };
      // como dito, própria instância passada.
      self
   }
}

// agora com o atribuição, faz a mesma coisa
// do que o acima, porém sem retorno. No 
// 'background' é atualização de valor.
impl AddAssign<u64> for ProgressoPercentual {
   // implementação da atualização de dados.
   fn add_assign(&mut self, valor:u64) { 
      // atualizando valores...
      if valor >= self.qtd_total { 
         self.esgotado = true; 
         self.qtd_atual = self.qtd_total; 
      } else 
         { self.qtd_atual = valor; }
      // computando novo percentual.
      self.percentual = {
         let x:f32 = self.qtd_atual as f32;
         let y:f32 = self.qtd_total as f32;
         if x > y { 1.0f32 }
         else { x / y }
      };
   }
}

/* visualização do atual estado da barra
 * se for requisitado. Seria como usar 
 * a função 'progresso' acima, não tem diferença
 * não há limite para a quantia de vezes
 * que a chama. */
impl Display for ProgressoPercentual {
   // implementando visualização em sí.
   fn fmt(&self, formatador:&mut Formatter<'_>) 
     -> Result_fmt 
   {
      // nomeando por legibilidade.
      let qa = self.qtd_atual;
      let qt = self.qtd_total;
      // obtendo 'String' baseado no atual/total.
      let barra_de_progresso = progresso_i(qa, qt);
      // "imprimindo" visualização da atual barra.
      write!(formatador, "{}", barra_de_progresso)
   }
}

/**
 Uma estrutura que mostra uma barra de progresso,
  configurado os milisegundos de disparo. Portanto,
  ele só forma a barra passado total de **ms**, se for 
  configurado com um valor maior ou igual à *1seg*, ou
  na faixas dos micro segundos, tal estrutura não
  se forma, para o programa. Tal restrição é necessária.
  Tal estrututa, assim com a percentual, são perfeitos
  para entrada de dados, assim não desaceleram o 
  programa, por ficar gerando a todo *ciclo* uma
  nova string formantando a barra.
*/
pub struct ProgressoTemporal {
   // quantidade atual passada.
   qtd_atual: u64,
   // o total inicialmente marcado.
   qtd_total: u64,
   // cronômetro para registra período de tempo.
   cronometro:Instant,
   // tempo delimitado para impressão em mili-seg.
   tempo: Duration
}

use progressosimples::progresso as progresso_i;
// implementação do método.
impl ProgressoTemporal {
   // criando uma instância...
   pub fn cria(qtd_total:u64, milisegs: Duration) -> Self {
      // criando dado agora...
      ProgressoTemporal {
         qtd_atual: 0,  
         qtd_total,
         cronometro: Instant::now(),
         tempo: milisegs
      }
   }

   /* Impressão limitada. Vê se houve um
    * um tempo considerável desde a última
    * impressão, se sim, então imprime
    * novamente, se assim for requisitado. */
   pub fn imprime(&mut self) -> Impressao {
      // tempo passado desde o último registro.
      let tp = self.cronometro.elapsed();
      /* no mínimo, um terço de segundo deve-se 
       * passar para liberar uma nova impressão. */
      if  tp >= self.tempo {
         // obtendo a barra de progresso simples.
         let bp = progresso_i(self.qtd_atual, self.qtd_total);
         // reiniciando o crônometro para recontagem de tempo.
         self.cronometro = Instant::now();
         // impressão em sí agora...
         Some(bp)
      }
      // se houver atingindo o máximo possível também.
      else if self.qtd_atual == self.qtd_total {
         // obtendo a barra de progresso simples.
         let bp = progresso_i(self.qtd_atual, self.qtd_total);
         // reiniciando o crônometro para recontagem de tempo.
         self.cronometro = Instant::now();
         // impressão em sí agora...
         Some(bp)
      }
      else { None }
   }
}

/* agora com o atribuição, faz a mesma coisa
 * do que o acima, porém sem retorno. No 
 * 'background' é atualização de valor. */
impl AddAssign<u64> for ProgressoTemporal {
   // implementação da atualização de dados.
   fn add_assign(&mut self, valor:u64) { 
      // atualizando valores...
      self.qtd_atual = valor; 
   }
}

/* visualização do atual estado da barra
 * se for requisitado. Seria como usar 
 * a função 'progresso' acima, não tem diferença
 * não há limite para a quantia de vezes
 * que a chama. */
impl Display for ProgressoTemporal {
   // implementando visualização em sí.
   fn fmt(&self, formatador:&mut Formatter<'_>) -> Result_fmt {
      // nomeando por legibilidade.
      let qa = self.qtd_atual;
      let qt = self.qtd_total;
      // obtendo 'String' baseado no atual/total.
      let barra_de_progresso = progresso_i(qa, qt);
      // "imprimindo" visualização da atual barra.
      write!(formatador, "{}", barra_de_progresso)
   }
}

/* cria uma proporção de progresso baseado 
 * na porcentagem dada. Então 0% é nada de
 * barra, e, 100% é a barra totalmente preenchida. */
fn cria_barra(percentagem:f32, capacidade:u8) -> String {
   let mut barra = String::new();
   let conta = (capacidade as f32 * percentagem) as usize;
   // falta de espaços-vázios.
   let diferenca:usize = (capacidade as usize) - conta;
   // concantena partes da barra.
   barra.push_str(&COMPONENTE.repeat(conta));
   barra.push_str(&VAZIO.repeat(diferenca));
   // retorna a barra formada com sua parte "consumida"
   // e uma parte "vázia", ou algum destes predominante.
   return barra;
}

/** Cria uma barra tempora, que mostra o progresso
 regressivo, também mostra uma contagem legível.
 O nome, dependendo da dimensão da tela, pode ser 
 contraído para se ajustar, ou virar um "slogan"
 dinâmico que mostra toda legenda, mas para isso 
 move-se.
*/
pub fn temporizador_progresso(rotulo:&str, 
tempo_atual:Duration, tempo_total:Duration) -> String {
   // total baseado na validade dada.
   let total = tempo_total.as_secs();
   // quantia restanteste.
   let qtd:u64 = {
      let ta = tempo_atual .as_secs();
      if ta > total { 0 }
      else { total - ta }
   };
   let percentual:f32 = (qtd as f32)/(total as f32);
   let p100:f32 = percentual * 100.0;

   // caso de erro.
   if percentual > 1.0_f32 {
      panic!("os valores de atual supera o total!");
   }
   else {
      // reduzindo nome se necessário.
      /* molde da string retornada representando por 
       * inteiro a barra de progresso. */
      let string:String = {
         if rotulo.len() > TEXTO_MAX { 
            rotulo
            .get(0..TEXTO_MAX)
            .unwrap()
            .to_string() + "..." 
         }
         else 
            { rotulo.to_string() }
      };
      let largura:usize = match terminal_largura() {
         Ok(Largura(l)) => l as usize,
         Err(_) => TEXTO_MAX - 1
      };
      let recuo:usize = largura - TEXTO_MAX - 3;
      let restante:String = {
         if qtd == 0
            { "imediatamente".to_string() }
         else 
            { legivel::tempo(qtd, false) }
      };

      // criando string formatada para retorno ...
      format!(
         "{0:<espaco$} {3} [{1}]{2:>5.1}%",
         string, 
         cria_barra(percentual, 20), 
         p100, restante,
         espaco = recuo - restante.len()
      )
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn teste_progresso_com_rotulo() {
      let rotulo:&str = "isso e um teste basico, sem panico";
      let mut logo:Logo = Logo::novo(rotulo).unwrap();
      for k in 1..(600_000+1) { 
         let bp = progresso_data_rotulo(
            logo.para_string(), 
            k, 600_000
         );
         print!("\r{}",bp); 
         logo.movimenta_letreiro();
      }
      assert!(true);
   }
}
