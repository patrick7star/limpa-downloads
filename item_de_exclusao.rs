
// biblioteca padrão do Rust:
use std::path::{Path, PathBuf};
use std::time::{SystemTime, Duration};
use std::ops::Drop;
use std::fs::{remove_dir, remove_file, read_dir};
use std::fmt::{
   Display, Formatter, 
   Result as Formato
};
use std::string::String;

// biblioteca externa:
extern crate utilitarios;
use utilitarios::progresso::temporizador_progresso;
use super::letreiro::Letreiro;

// próprio módulo.
mod remocao_dir;
use remocao_dir::{ self as RD, };


/* Trait para re-implementação do Drop com "saída" na tela. Está versão é 
 * fora do "ncurses", que têm outra na parte gráfica. */
trait DropPadrao 
   { fn drop(&mut self); }

/** Elememento com dados e, principalmente dada de exclusão.  */
pub struct Item {
   // caminho para o item.
   pub caminho: PathBuf,
   // nome do item em sí.
   pub nome: String,
   // tempo restantes de vida(na fila de exclusão).
   pub validade: Duration,
   // o último acesso do itém.
   pub ultimo_acesso: SystemTime,
   // letreiro dinâmico, muito para o modo gráfico.
   pub letreiro: Letreiro
}

impl Item {
   // cria instância.
   pub fn cria(caminho:PathBuf, ultimo_acesso:SystemTime, 
   validade:Duration) -> Self {
      // extraí nomes do caminho.
      let nome:String = {
         caminho.as_path()
         .file_name()
         .unwrap()
         .to_str()
         .unwrap()
         .to_string()
      };
      // letreiro com letras(do nome) em movimento.
      let letreiro:Letreiro = Letreiro::novo(nome.as_str()).unwrap();
      // cria tal objeto.
      Item { caminho, nome, validade, ultimo_acesso, letreiro}
   }

   // verifica se o item já expirou.
   pub fn expirado(&mut self) -> bool {
      /* movimenta letreiro aqui, pois será chamado bem frequentemente 
       * neste bloco. */
      self.letreiro.movimenta_letreiro();

      /* se o último acesso ter excedido a validade dada, então dá o 
       * itém com expirado. */
      match self.ultimo_acesso.elapsed() {
         Ok(acesso) => {
            if self.validade < acesso  { true }
            else { false }
         } Err(estimativa) => {
            let acesso = estimativa.duration();
            if self.validade < acesso  
               { true }
            else { false }
         }
      }
   }

   // tempo restante da validade.
   pub fn tempo_restante(&mut self) -> Duration { 
      if !self.expirado() { 
         match self.ultimo_acesso.elapsed() {
            Ok(acesso) => 
               { return self.validade - acesso; }
            Err(estimativa) => { 
               let acesso = estimativa.duration();
               return self.validade - acesso;
            }
         }
      }
      else { Duration::from_secs(0) }
   }
}

// impressão sobre o status do ítem.
impl Display for Item {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> Formato {
      let duracao = match self.ultimo_acesso.elapsed() {
         Ok(ultimo_acesso) => ultimo_acesso,
         Err(estimativa) => estimativa.duration()
      };
      let barra_de_progresso:String = {
         temporizador_progresso(
            self.nome.as_str(),
            duracao,
            self.validade
         )
      };
      write!(formatador, "{}", barra_de_progresso)
   }
}

// Exclusão do ítem em sí. Deleta arquivo também. Este é sem output!
impl Drop for Item {
   /* Apenas deleta o arquivo, se o "tempo de validade" realmente acabou! */
   fn drop(&mut self) {
      if self.expirado() {
         let caminho = self.caminho.as_path();
         if caminho.is_file() {
            match remove_file(caminho) {
               Ok(_) => (),
               Err(_) => 
                  { panic!("[ERRO!!!] o arquivo ainda continua!"); } 
            };
         }
         else if caminho.is_dir() && diretorio_esta_vazio(caminho)
         // Caso específico para pastas vázias:
            { remove_dir(caminho).unwrap(); }

         /* Diretório com conteúdo(pastas e arquivos, e também subdiretórios
          * com mais arquivos e pastas). */
         remocao_dir::remocao_completa(caminho);
      }
   }
}

impl DropPadrao for Item {
   /* apenas acaba se, e somente se, o item expirou. */
   fn drop(&mut self) {
      print!("==> removendo \"{}\"... ", self.nome); 
      // remoção do arquivo.
      drop(self);
      println!("realização sucedida.");
   }
}

pub struct FilaExclusao {
   // todos ítens da raíz dada.
   pub todos: Vec<Item>,
   // lista de exclusão nas próximas horas.
   pub proximas_exclusao: Vec<Item>,
}

impl FilaExclusao {
   // constante contendo raíz do diretório análisado.
   const RAIZ:&'static str = concat!(env!("HOME"), "/Downloads");

   /// Verifica se não há mais nada analisar e deletar.
   pub fn vazia(&self) -> bool { 
      /* Ambas array-dinâmicas tem que está vázia para a fila como toda, 
       * também assim, ser considerada. */
      self.todos.is_empty() && 
      self.proximas_exclusao.is_empty() 
   }

   // Computa a devida 'duração' para a devida extensão.
   fn duracao_para_devida_extensao(extension: &str) -> Duration {
      if extension == "iso" {
         const MESES:u64 = (2.7*30.0*24.0*3600.0) as u64;
         Duration::from_secs(MESES)
      } else if extension == "zip" {
         const ALGUNS_HORAS:u64 = (4.8 * 3600.0) as u64;
         Duration::from_secs(ALGUNS_HORAS)
      } else if extension == "ttf" || extension == "deb" {
         const ALGUNS_MINUTOS:u64 = (32.3 * 60.0) as u64;
         Duration::from_secs(ALGUNS_MINUTOS)
      } else if extension == "pdf" {
         const ALGUNS_HORAS:u64 = (9.6 * 3600.0) as u64;
         Duration::from_secs(ALGUNS_HORAS)
      } else if extension == "torrent" {
         const ALGUNS_MINUTOS:u64 = (45.2 * 60.0) as u64;
         Duration::from_secs(ALGUNS_MINUTOS)
      } else if extension == "dat" || extension == "djvu" ||
      extension == "toml" {
         const ALGUNS_MINUTOS:u64 = (7.2 * 60.0) as u64;
         Duration::from_secs(ALGUNS_MINUTOS)
      } else if extension == "epub" {
         const ALGUNS_DIAS:u64 = (9.6 * 24.0 * 3600.0) as u64;
         Duration::from_secs(ALGUNS_DIAS)
      } else if extension == "tar" || extension == "gz" {
         const ALGUNS_HORAS:u64 = (3.8 * 3600.0) as u64;
         Duration::from_secs(ALGUNS_HORAS)
      } else {
         const PADRAO:u64 = (5.9 * 3600.0) as u64;
         Duration::from_secs(PADRAO)
      }
   }

   // Computa duração para um diretório, seja qual for seu estado.
   fn duracao_para_diretorio(caminho: &Path) -> Duration {
      const DIA: f32 = 24.0 * 3600.0;
      const ALGUNS_DIAS:u64 = (DIA * 13.9) as u64;

      // criando o ítem e adicionando na lista.
      if diretorio_esta_vazio(caminho)
         { Duration::from_secs(5 * 60) }
      else 
         { Duration::from_secs(ALGUNS_DIAS) }
   }

   /// Visualiza e opera possível exclusão.
   pub fn visualiza(&mut self) {
      /* pondo 'Item's que estão prestes a 
       * ser deletados, na fila de exclusão.
       */
      let mut qtd = self.todos.len();
      while qtd > 0 {
         // na fila de exclusão de hoje.
         let sera_excluido_hoje:bool = {
            let item = self.todos.get_mut(qtd-1).unwrap();
            // tempo restante do ítem.
            let tr = item.tempo_restante();
            // tempo de hoje em segundos.
            let hoje = Duration::from_secs(24*3600);
            tr < hoje 
         };
         if sera_excluido_hoje {
            let item = self.todos.remove(qtd-1);
            self.proximas_exclusao.push(item);
         }
         qtd -= 1;
      }

      // lista de exclusão.
      let mut referencias: Vec<&mut Item>;
      referencias = Vec::with_capacity(qtd);
      /* não se mostra expirados, serão excluídos
       * em seguida, aqui serão colocadas na lista
       * de exclusão. */
      for item in self.proximas_exclusao.iter_mut() { 
         if !item.expirado()
            { println!("{}", item); }
         else
            { referencias.push(item); }
      }
      println!("\n");

      // exclui referências, expiradas, escolhidas.
      for item in referencias.drain(..) {
         if item.expirado() 
            { DropPadrao::drop(item); }
      }
   }

   /// Gera itens baseado no diretório RAÍZ.
   pub fn gera() -> Self {
      let mut lista:Vec<Item> = Vec::new();
      let mut todas_entradas = read_dir(Self::RAIZ).unwrap();

      // Analisando cada objeto no diretório "Downloads" ...
      while let Some(Ok(entry)) = todas_entradas.next() {
         let caminho = entry.path();

         // No caso de se for um diretório.
         if caminho.is_dir() {
            let validade = Self::duracao_para_diretorio(&caminho);
            let auxiliar = SystemTime::now();
            let ua_medio = RD::acesso_medio_dir(&caminho);
            let tempo = Duration::from_secs(ua_medio as u64);
            let ua = {
               auxiliar.checked_add(tempo)
               .expect("falha no ST do caminho passado!")
            };
            let item = Item::cria(caminho, ua, validade);

            lista.push(item);
            // Indo para próxima entrada...
            continue;
         }

         // Caso geral: um arquivo com extensão.
         if let Some(os_str) = caminho.extension() {
            if let Some(ext) = os_str.to_str() {
               let validade = Self::duracao_para_devida_extensao(ext);

               if let Ok(ua) = entry.metadata().unwrap().accessed()
                  { lista.push(Item::cria(caminho, ua, validade)); }
            }
         }
      }
      // criando instância em sí, já retornando ...
      FilaExclusao { todos: lista, proximas_exclusao: Vec::new() }
   }

   /// Há algo na fila de "exclusão diária".
   pub fn ha_exclusao_hoje(&self) -> bool
      { self.proximas_exclusao.len() > 0 }
   
   /// Quantidade total de itens para exclusão próxima.
   pub fn total(&mut self) -> usize { 
      let contagem: usize = {
         self.todos.len() + 
         self.proximas_exclusao.len()
      };
      /* Retirando todos os itens e verificando, pois via iteradores 
       * não conseguir fazer-lô. Retiro cada um e coloco no final para
       * no final tal array fica inalterada. */
      let desconto: usize = {
         let mut qtd = self.proximas_exclusao.len();
         let mut contagem = 0;

         while qtd > 0 {
            let mut item = self.proximas_exclusao.remove(0);
            if item.expirado()
               { contagem += 1; }
            self.proximas_exclusao.push(item);
            qtd -= 1;
         }
         contagem
      };
      // total de itens menos os expirados.
      contagem - desconto 
   }
}

fn diretorio_esta_vazio(caminho:&Path) -> bool {
   /* Tenta percorrer, se conseguir no mínimo um não está vázio. */
   for _ in read_dir(caminho).unwrap() 
      { return false; }
   // se chega até aqui, então está vázio.
   return true;
}


#[cfg(test)]
mod tests {
   // use super::*;
}
