
/*! 
código que dá utilitário que retorna barra
de progresso, dados uma quantia atual e total
a ser atingida. 
*/


// biblioteca do Rust:
use super::legivel::tamanho;
use std::time::{Duration, Instant};
use std::fmt::{Display, Formatter, Result as Result_fmt};
use std::ops::{Add, AddAssign, Range};

// própria biblioteca:
use crate::legivel;
use super::terminal_dimensao::{TD, terminal_largura};


// símbolo que representa frações das barras e espaços vázios.
const COMPONENTE:&'static str ="#";
const VAZIO:&'static str=".";
const CAPACIDADE:u8 = 50;
const TEXTO_MAX:usize = 30;

/* cria uma proporção de progresso baseado 
 * na porcentagem dada. Então 0% é nada de
 * barra, e, 100% é a barra totalmente preenchida. */
fn cria_barra(percentagem:f32) -> String {
   let mut barra = String::new();
   let conta = (CAPACIDADE as f32 * percentagem) as usize;
   // falta de espaços-vázios.
   let diferenca:usize = 50 - conta;
   // concantena partes da barra.
   barra.push_str(&COMPONENTE.repeat(conta));
   barra.push_str(&VAZIO.repeat(diferenca));
   // retorna a barra formada com sua parte "consumida"
   // e uma parte "vázia", ou algum destes predominante.
   return barra;
}


fn conta_algs(valor:usize) -> u8 {
   let mut d:f32 = valor as f32;  // cópia valor real ...
   let mut contador:u8 = 0; // contador de divisões
   while d > 1.0 {
      // cada divisão por dez, conta um.
      d = d / 10.0;
      contador += 1;
   }
   // retorno a contagem contabilizando um ...
   return contador+1;
}


/** cálculos percentuais e, suas representação
 numérica. */
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
         cria_barra(1.0), 
         percent_100,espaco=qtd_algs
      );
   }

   /* molde da string retornada representando por 
    * inteiro a barra de progresso. */
   let molde:String = format!(
      "{0:>espaco$} de {1} [{2}]{3:>5.1}%",
      qtd_atual, qtd_total, 
      cria_barra(percentagem), 
      percent_100,espaco=qtd_algs
   );
   return molde;
}


/** cálculos percentuais e, suas representação
 numérica. */
pub fn progresso_data(qtd_atual:u64, qtd_total:u64) -> String {
   let percentagem:f32 = (qtd_atual as f32)/(qtd_total as f32);
   let percent_100:f32 = percentagem*100.0;

   // caso de erro.
   if percentagem > 1.0_f32 {
      panic!("os valores de atual supera o total!");
   }
   else if percentagem == 1.00 {
      let total_bytes = tamanho(qtd_total, true);
      return format!(
         "{}/{} [{}] {}%\n",
         total_bytes,
         total_bytes,
         cria_barra(1.0),
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
         qa, qt, cria_barra(percentagem), 
         percent_100,espaco=qtd_algs
      );
      return molde;
   }
}

pub struct Logo<'a> {
   // para marcar o tempo.
   ti:Instant,
   // o texto que será mostrado.
   rotulo:&'a str,
   // quando da string mostrar.
   capacidade:u8,
   // inicio e fim onde visualizar a string.
   ponta_esquerda:u8,
   ponta_direita:u8,
   // intervalo válido.
   intervalo:Option<Range<usize>>,
}
impl <'a> Logo<'a> {
   // criando uma nova instância.
   pub fn novo(label:&str) -> Result<Logo, &'static str> {
      if label.len() == 0 {
         Err("não é permitido strings em branco")
      }
      else {
         Ok(
            Logo {
               // iniciando contagem.
               ti: Instant::now(),
               // pegando o rótulo a dimanizar.
               rotulo: label,
               // capacidade definida manualmente.
               capacidade: 15, // quinze caractéres.
               ponta_esquerda: 0,
               ponta_direita: 15,
               intervalo:Some(0..15),
            }
         )
      }
   }
   // motor do logo. 
   pub fn movimenta_letreiro(&mut self) {
      // se chegou ao final, resetar posição do LED.
      if self.ponta_direita == self.rotulo.len() as u8 {
         self.ponta_direita = self.capacidade;
         self.ponta_esquerda = 0;
      }
      // a cada 1,5seg mover o led 'uma casa'.
      if self.ti.elapsed() > Duration::from_millis(500) {
         if self.ponta_direita <= self.rotulo.len() as u8 {
            // deslocando led...
            self.ponta_esquerda += 1;
            self.ponta_direita += 1;
            // resetando contagem...
            self.ti = Instant::now();
         }
      }
      // definindo novo intervalo.
      self.intervalo = {
         // "renomeação" para melhor legibilidade.
         let pe:usize = self.ponta_esquerda as usize;
         let pd:usize = self.ponta_direita as usize;
         Some(pe..pd)
      };
   }
   // transforma numa slice-string.
   pub fn para_string(&self) -> &'a str {
      match self.intervalo.clone() {
         Some(i) => {
            self.rotulo
            .get(i)
            .unwrap()
         },
         None => self.rotulo,
      }
   }
}
impl Display for Logo<'_> {
   fn fmt(&self, f:&mut Formatter<'_>) -> Result_fmt {
      // apeliando para legibilidade.
      match self.intervalo.clone() {
         Some(i) => {
            write!(
               f, "{}...",
               self.rotulo.get(i)
               .unwrap()
            )
         },
         None => write!(f, "{}", self.rotulo)
      }
   }
}

/** progresso com rótulo dinâmico, então quando
 o rótulo é muito maior do que cabe na tela, ele
 move o rótulo tipo aqueles slogans de neon em
 mercearias e shops. Ele também é de dados, o 
 resto do seu núcleo.
*/
pub fn progresso_data_rotulo<'a>(rotulo:&'a str, qtd_atual:u64, 
qtd_total:u64) -> String {
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
         cria_barra(1.0),
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
         qa, qt, cria_barra(percentagem), 
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
 a 'quantia atual' e '... total'. */
pub struct ProgressoPercentual {
   // quantidade atual passada.
   qtd_atual:u64,
   // o total inicialmente marcado.
   qtd_total:u64,
   // percentual computado.
   percentual:f32,
   // antigo percentual para comparação.
   antigo_percentual:f32,
}

// implementação do método.
impl ProgressoPercentual {
   // criando uma instância...
   pub fn cria(qtd_total:u64) -> Self {
      // criando dado agora...
      ProgressoPercentual {
         qtd_atual: 0,  
         qtd_total: qtd_total,
         percentual: 0.0f32,
         antigo_percentual: 0.0f32
      }
   }

   /* Impressão limitada. Vê se houve um
    * aumento significativo, então dá o 
    * resultado, caso contrário, retorna
    * nenhum dado. */
   pub fn imprime(&mut self) -> Option<String> {
      // diferença de percentuais.
      let diferenca = self.percentual - self.antigo_percentual;
      // verifica se há uma variação de 
      // no mínimo 0,5%.
      if diferenca >= 0.005 {
         // atualiza o 'antigo percentual' com o valor do "atual".
         self.antigo_percentual = self.percentual;
         // criando barra...
         let bp = progresso(self.qtd_atual, self.qtd_total);
         // retornando barra, porém embrulhado.
         return Some(bp);
      }
      // já está terminado, ou acabou de começar.
      else if self.qtd_total == self.qtd_atual
      || self.qtd_atual == 0 {
         // criando barra...
         let bp = progresso(self.qtd_atual, self.qtd_total);
         // retornando barra, porém embrulhado.
         return Some(bp);
      }
      else { None }
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
      self.qtd_atual = valor; 
      // computando novo percentual.
      self.percentual = {
         let x:f32 = self.qtd_atual as f32;
         let y:f32 = self.qtd_total as f32;
         x / y
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
   fn fmt(&self, formatador:&mut Formatter<'_>) -> Result_fmt {
      // nomeando por legibilidade.
      let qa = self.qtd_atual;
      let qt = self.qtd_total;
      // obtendo 'String' baseado no atual/total.
      let barra_de_progresso = progresso(qa, qt);
      // "imprimindo" visualização da atual barra.
      write!(formatador, "{}", barra_de_progresso)
   }
}

pub struct ProgressoTemporal {
   // quantidade atual passada.
   qtd_atual:u64,
   // o total inicialmente marcado.
   qtd_total:u64,
   // cronômetro para registra período de tempo.
   cronometro:Instant,
   // tempo delimitado para impressão em mili-seg.
   tempo:u16
}

// implementação do método.
impl ProgressoTemporal {
   // criando uma instância...
   pub fn cria(qtd_total:u64, tempo_em_miliseg:u16) -> Self {
      // criando dado agora...
      ProgressoTemporal {
         qtd_atual: 0,  
         qtd_total,
         cronometro: Instant::now(),
         tempo: tempo_em_miliseg
      }
   }

   /* Impressão limitada. Vê se houve um
    * um tempo considerável desde a última
    * impressão, se sim, então imprime
    * novamente, se assim for requisitado. */
   pub fn imprime(&mut self) -> Option<String> {
      // tempo passado desde o último registro.
      let tp = self.cronometro.elapsed();
      /* no mínimo, um terço de segundo deve-se 
       * passar para liberar uma nova impressão. */
      if  tp >= Duration::from_millis(self.tempo as u64) {
         // obtendo a barra de progresso simples.
         let bp = progresso(self.qtd_atual, self.qtd_total);
         // reiniciando o crônometro para recontagem de tempo.
         self.cronometro = Instant::now();
         // impressão em sí agora...
         Some(bp)
      }
      // se houver atingindo o máximo possível também.
      else if self.qtd_atual == self.qtd_total {
         // obtendo a barra de progresso simples.
         let bp = progresso(self.qtd_atual, self.qtd_total);
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
      let barra_de_progresso = progresso(qa, qt);
      // "imprimindo" visualização da atual barra.
      write!(formatador, "{}", barra_de_progresso)
   }
}

/* cria uma proporção de progresso baseado 
 * na porcentagem dada. Então 0% é nada de
 * barra, e, 100% é a barra totalmente preenchida. */
fn cria_barra_i(percentagem:f32, capacidade:u8) -> String {
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
         Ok(_enum) => match _enum {
            TD::Largura(l) => l as usize,
            _ => TEXTO_MAX - 1,
         },
         Err(_) => TEXTO_MAX - 1,
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
         cria_barra_i(percentual, 20), 
         p100, restante,
         espaco = recuo - restante.len()
      )
   }
}


#[cfg(test)]
mod tests {
   use super::*;
   use std::thread::sleep;

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

   #[test]
   fn letreiro_dinamico() {
      // instanciando logo dinâmico...
      let texto = "isso e apenas um texto de teste, entao nao entre em panico";
      let mut logo = Logo::novo(texto).unwrap();
      // marcador de tempo.
      let t:Instant = Instant::now();
      while t.elapsed() < Duration::from_secs(15) {
         print!("\r{}", logo.para_string());
         logo.movimenta_letreiro();
      }
      assert!(true);
   }

   #[test]
   fn testando_funcao_que_gira() {
      let texto = "eu adoro suco de caju";
      let mut logo = Logo::novo(texto).unwrap();
      // movimento o texto, dormindo de acordo com
      // o tempo de translação dele(simulando tempo).
      sleep(Duration::from_secs_f32(0.5));
      logo.movimenta_letreiro();
      sleep(Duration::from_secs_f32(0.5));
      logo.movimenta_letreiro();
      sleep(Duration::from_secs_f32(0.5));
      logo.movimenta_letreiro();
      // tirando trecho translado.
      let parte_i = logo.para_string();
      /* o previsto, levando o tempo, e, 
       * lembrando que o "LED" tem 14 locais,
       * tem que cair exatamente como a 
       * frase abaixo.
       */
      let parte_ii = "adoro suco de c";
      // verificando resposta.
      assert_eq!(parte_i, parte_ii);
   }
}
