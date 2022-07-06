
extern crate utilitarios;
use utilitarios::{
   barra_de_progresso::temporizador_progresso,
   aleatorio::sortear
};

// minha biblioteca:
use crate::item_de_exclusao::Item;

// biblioteca padrão do Rust:
use std::time::{Instant, Duration};
use std::ops::Range;
use std::fmt::{Formatter, Display, Result as Formato};

// um retorno de string diferente do padrão.
pub trait StringDinamica {
   fn to_string(&self) -> String ;
}

/* implementando letreiro aqui, com alguns tweaks.
 * O código têm, com totalidade, o núcleo do código
 * dos `Logo` dos `utilitarios`. */
pub struct Letreiro {
   // para marcar o tempo.
   ti:Instant,
   // o texto que será mostrado.
   rotulo: String,
   // quando da string mostrar.
   capacidade:u8,
   // inicio e fim onde visualizar a string.
   ponta_esquerda:u8,
   ponta_direita:u8,
   // intervalo válido.
   intervalo:Option<Range<usize>>,
   /* pausa da velocidade do atual letreiro, decidido
    * de maneira randômica. */
   pausa: u64,
   // string repetida do nome, para um movimento
   rolo: String
}

// implementando métodos da "estrutura".
impl Letreiro {
   // remove acentuação de string-rolo:
   fn remove_acentuacao(rolo:&mut String) {
      *rolo = rolo.replace("é", "e");
      *rolo = rolo.replace("ê", "e");
      *rolo = rolo.replace("á", "a");
      *rolo = rolo.replace("â", "a");
      *rolo = rolo.replace("ã", "a");
      *rolo = rolo.replace("à", "a");
      *rolo = rolo.replace("ú", "u");
      *rolo = rolo.replace("í", "i");
      *rolo = rolo.replace("ô", "o");
      *rolo = rolo.replace("ó", "o");
      *rolo = rolo.replace("ç", "c");
   }
   fn cria_rolo(rotulo:&str) -> String {
      let mut rolo = String::new();
      for _ in 1..=5 {
         rolo.push(' ');
         rolo.push_str(rotulo);
         rolo.push(' ');
      }
      /* retirando a acentuação, por causa de conflito
       * ainda não tratado. */
      Letreiro::remove_acentuacao(&mut rolo);
      return rolo;
   }
   // criando uma nova instância.
   pub fn novo(label:&str) -> Result<Letreiro, &'static str> {
      if label.len() == 0 {
         Err("não é permitido strings em branco")
      }
      else {
         // apelidos para ajudar na legibilidade ...
         let capacidade:u8 = 30;
         let intervalo = Some(0..capacidade as usize);
         let rolo = Letreiro::cria_rolo(label);
         // encapsulando num Result para dá confiança no resultado.
         Ok(
            Letreiro {
               // iniciando contagem.
               ti: Instant::now(),
               // pegando o rótulo a dimanizar.
               rotulo: label.to_string(),
               // capacidade definida manualmente.
               capacidade, 
               ponta_esquerda: 0,
               ponta_direita: capacidade,
               intervalo, 
               pausa: {
                  let fator:u64 = sortear::u8(0..=5) as u64;
                  let acrescimo:u64 = sortear::u8(100..=255) as u64;
                  if sortear::bool() 
                     { 1_000 + fator*acrescimo }
                  else { 
                     let x = fator*acrescimo;
                     if x > 1_000
                        { x - 1_000 }
                     else 
                        { 1000 - x }
                  }
               },
               rolo, 
            }
         )
      }
   }
   // motor do logo. 
   pub fn movimenta_letreiro(&mut self) {
      // se chegou ao final, resetar posição do LED.
      //if self.ponta_direita == self.rotulo.len() as u8 {
      if self.ponta_direita == self.rolo.len() as u8 {
         self.ponta_direita = self.capacidade;
         self.ponta_esquerda = 0;
      }
      // a cada 1,5seg mover o led 'uma casa'.
      if self.ti.elapsed() > Duration::from_millis(self.pausa) {
         //if self.ponta_direita <= self.rotulo.len() as u8 {
         if self.ponta_direita <= self.rolo.len() as u8 {
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
}

// formatação padrão ao ir a string o tipo `Letreiro`.
impl Display for Letreiro {
   fn fmt(&self, f:&mut Formatter<'_>) -> Formato {
      // apelidos para melhorar a legiblidade.
      let intervalo = self.intervalo.as_ref().unwrap();
      let mut rotulo = self.rotulo.clone();
      Letreiro::remove_acentuacao(&mut rotulo);
      let tamanho_str = rotulo.len() as u8;

      // vendo se a string pode ser contraída ...
      if tamanho_str < self.capacidade
         { write!(f, "{}", rotulo)  }
      else {
         let retangulo = {
            self.rolo
            .get(intervalo.clone())
            .unwrap()
         };
         write!(f, "{}...", retangulo)  
      }
   }
}

// impressão sobre o status do ítem.
impl StringDinamica for Item {
   fn to_string(&self) -> String {
      let duracao = match self.ultimo_acesso.elapsed() {
         Ok(ultimo_acesso) => ultimo_acesso,
         Err(estimativa) => estimativa.duration()
      };
      let barra_de_progresso:String = {
         temporizador_progresso(
            self.letreiro.to_string().as_str(),
            duracao,
            self.validade
         )
      };
      format!("{}", barra_de_progresso)
   }
}
