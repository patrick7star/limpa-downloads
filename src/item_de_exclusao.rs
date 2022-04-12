
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
use utilitarios::barra_de_progresso::temporizador_progresso;
use utilitarios::terminal_dimensao::{TD, terminal_largura};
use super::letreiro::Letreiro;


/* reescrevendo o método do len da string para 
 * pegar acentuações conhecidas de dois bytes.
 */
trait StringExtensao {
   /* computa o tamanho de bytes entre strings
    * levando em conta caractéres de 2 bytes. */
   fn len(&self) -> usize;
}

/* Trait para re-implementação do Drop com "saída"
 * na tela. Está versão é fora do "ncurses", que 
 * têm outra na parte gráfica. */
trait DropPadrao { fn drop(&mut self); }

// para slice-strings(stack-strings) `&str`.
impl StringExtensao for str {
   fn len(&self) -> usize {
      // conta a quantia de acentuações comuns.
      let mut qtd:usize = 0;
      for ch in self.chars() {
         if ch == 'á' { qtd += 1; }
         if ch == 'à' { qtd += 1; }
         if ch == 'â' { qtd += 1; }
         if ch == 'ã' { qtd += 1; }
         if ch == 'é' { qtd += 1; }
         if ch == 'ê' { qtd += 1; }
         if ch == 'í' { qtd += 1; }
         if ch == 'ô' { qtd += 1; }
         if ch == 'õ' { qtd += 1; }
         if ch == 'ó' { qtd += 1; }
         if ch == 'ú' { qtd += 1; }
         if ch == 'ç' { qtd += 1; }
      }
      let tamanho = self.len();
      return tamanho - qtd;
   }
}

// para heap-strings `String`.
impl StringExtensao for String {
   fn len(&self) -> usize 
      { self.as_str().len() }
}

/** elememento com dados e, principalmente 
 dada de exclusão.  */
//#[derive(Clone)]
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
      /* movimenta letreiro aqui, pois será 
       * chamado bem frequentemente neste bloco. */
      self.letreiro.movimenta_letreiro();

      /* se o último acesso ter excedido a validade
       * dada, então dá o itém com expirado. */
      let acesso = self.ultimo_acesso.elapsed().unwrap();
      if self.validade < acesso  
         { return true; }
      else
         { return false; }
   }

   // tempo restante da validade.
   pub fn tempo_restante(&mut self) -> Duration { 
      if !self.expirado() { 
         let acesso = self.ultimo_acesso.elapsed().unwrap();
         return self.validade - acesso;
      }
      else { Duration::from_secs(0) }
   }
}

// impressão sobre o status do ítem.
impl Display for Item {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> Formato {
      let barra_de_progresso:String = {
         temporizador_progresso(
            self.nome.as_str(),
            self.ultimo_acesso.elapsed().unwrap(),
            self.validade
         )
      };
      write!(formatador, "{}", barra_de_progresso)
   }
}

// exclusão do ítem em sí.
impl Drop for Item {
   // deleta arquivo também. Este é sem output!
   fn drop(&mut self) {
      /* apenas deleta o arquivo, se o "tempo de 
       * validade" realmente acabou! */
      if self.expirado() {
         let caminho = self.caminho.as_path();
         if caminho.is_file() {
            match remove_file(caminho) {
               Ok(_) => (),
               Err(_) => 
                  { panic!("[ERRO!!!] o arquivo ainda continua!"); } 
            };
         }
         // caso específico para pastas vázias:
         else if caminho.is_dir() 
            { remove_dir(caminho).unwrap(); }
      }
   }
}

impl DropPadrao for Item {
   /* apenas acaba se, e somente se, 
    * o item expirou. */
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
   pub proximas_exclusao: Vec<Item>
}

impl FilaExclusao {
   // constante contendo raíz do diretório análisado.
   const RAIZ:&'static str = concat!(env!("HOME"), "/", "Downloads");

   /// verifica se não há mais nada analisar e deletar.
   pub fn vazia(&self) -> bool { 
      /* ambas array-dinâmicas tem que está vázia
       * para a fila como toda, também assim, ser
       * considerada. */
      self.todos.is_empty() && 
      self.proximas_exclusao.is_empty() 
   }

   /// visualiza e opera possível exclusão.
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
            let hoje:Duration = Duration::from_secs(24*3600);
            tr < hoje 
         };
         if sera_excluido_hoje {
            let item = self.todos.remove(qtd-1);
            self.proximas_exclusao.push(item);
         }
         qtd -= 1;
      }

      /* função pega uma slice-string e imprime-a 
       * centralizando-a baseado no seu tamanho. */
      fn imprime_no_centro<'a>(string:&'a str) {
         // quantia de caractéres da string.
         let tamanho = string.len();
         // largura total do terminal.
         let largura:usize = match terminal_largura() {
            Ok(_enum) => match _enum {
               TD::Largura(l) => l as usize,
               _ => 32,
            },
            Err(_) => 32,
         };
         // espaços em branco da borda esquerda.
         let recuo = (largura - tamanho) / 2 - 1;
         println!(
            "{recuo}{}:", 
            string.to_uppercase(), 
            recuo = &" ".repeat(recuo)
         );
      }

      // visualizando lista de todos 'Item's.
      println!("\n");
      imprime_no_centro("lista de items");
      for item in self.todos.iter() 
         { println!("{}", item); }
      imprime_no_centro("exclusão de hoje");
      for item in self.proximas_exclusao.iter() 
         { println!("{}", item); }
      println!("\n");

      let mut qtd = self.proximas_exclusao.len();
      while qtd > 0 {
         let item = self.proximas_exclusao.get_mut(qtd-1).unwrap();
         if item.expirado() 
            { DropPadrao::drop(&mut self.proximas_exclusao.remove(qtd-1)); }
         qtd -= 1;
      }
   }

   /// gera itens baseado no diretório RAÍZ.
   pub fn gera() -> Self {
      // array-dinâmica.
      let mut lista:Vec<Item> = Vec::new();
      // analisando cada objeto no diretório "Downloads".
      for entry in read_dir(FilaExclusao::RAIZ).unwrap() {
         let entrada = entry.unwrap();
         // se for um diretório ignorar ...
         let e_um_diretorio:bool = {
            entrada
            .path()
            .is_dir()
         };
         // no caso de se é um diretório.
         if e_um_diretorio { 
            // trabalhando por enquanto apenas com pastas vázias.
            if diretorio_vazio(entrada.path().as_path()) {
               let validade:Duration;
               const ALGUNS_MINUTOS:u64 = (13.9 * 60.0) as u64;
               validade = Duration::from_secs(ALGUNS_MINUTOS);
               // criando o ítem e adicionando na lista.
               let item = Item::cria(
                  entrada
                  .path(),
                  entrada
                  .metadata()
                  .unwrap()
                  .accessed()
                  .unwrap(),
                  validade
               );
               lista.push(item);
            }
            // próximo ítem do laço ...
            continue;
         }
         
         // a extensão do arquivo.
         let aux_path = entrada.path();
         let extensao:&str = {
            match aux_path.as_path().extension() {
               Some(string) => string.to_str().unwrap(),
               None => { continue; },
            }
         };
         // computando a 'validade' ...
         let validade:Duration;
         if extensao == "iso" {
            const MESES:u64 = (2.7*30.0*24.0*3600.0) as u64;
            validade = Duration::from_secs(MESES);
         } else if extensao == "zip" {
            const ALGUNS_HORAS:u64 = (4.8 * 3600.0) as u64;
            validade = Duration::from_secs(ALGUNS_HORAS);
         } else if extensao == "ttf" || extensao == "deb" {
            const ALGUNS_MINUTOS:u64 = (32.3 * 60.0) as u64;
            validade = Duration::from_secs(ALGUNS_MINUTOS);
         } else if extensao == "pdf" {
            const ALGUNS_HORAS:u64 = (9.6 * 3600.0) as u64;
            validade = Duration::from_secs(ALGUNS_HORAS);
         } else if extensao == "torrent" {
            const ALGUNS_MINUTOS:u64 = (45.2 * 60.0) as u64;
            validade = Duration::from_secs(ALGUNS_MINUTOS);
         } else if extensao == "dat" || extensao == "djvu" ||
         extensao == "toml" {
            const ALGUNS_MINUTOS:u64 = (7.2 * 60.0) as u64;
            validade = Duration::from_secs(ALGUNS_MINUTOS);
         } else if extensao == "epub" {
            const ALGUNS_DIAS:u64 = (9.6 * 24.0 * 3600.0) as u64;
            validade = Duration::from_secs(ALGUNS_DIAS);
         } else if extensao == "tar" || extensao == "gz" {
            const ALGUNS_HORAS:u64 = (3.8 * 3600.0) as u64;
            validade = Duration::from_secs(ALGUNS_HORAS);
         } else {
            const PADRAO:u64 = (5.9 * 3600.0) as u64;
            validade = Duration::from_secs(PADRAO);
         }
         
         // criando o ítem e adicionando na lista.
         let item = Item::cria(
            entrada
            .path(),
            entrada
            .metadata()
            .unwrap()
            .accessed()
            .unwrap(),
            validade
         );
         lista.push(item)
      }
      // criando instância em sí, já retornando ...
      return FilaExclusao {
         todos: lista,
         proximas_exclusao: Vec::new()
      }
   }
}

// verifica se o diretório passado está vázio.
fn diretorio_vazio(caminho:&Path) -> bool {
   /* tenta percorrer, se conseguir no 
    * mínimo um não está vázio. */
   for _ in read_dir(caminho).unwrap() 
      { return false; }
   // se chega até aqui, então está vázio.
   return true;
}


#[cfg(test)]
mod tests {
   use super::*;
   use std::thread;
   use std::path::Path;
   use std::fs::{self,write};

   #[test]
   fn testa_struct_item() {
      let validade = Duration::from_secs(30);
      let caminho = Path::new("the ring I.mp4").to_path_buf();
      write(caminho.clone(), b"nada de dados relevantes!!!").unwrap();
      let ultimo_acesso = SystemTime::now();
      let mut item = Item::cria(caminho, ultimo_acesso, validade);
      while !item.expirado() {
         print!("{}\n", item);
         // pausa para não imprimir continuamente.
         thread::sleep(Duration::from_secs(5));
      }
      // uma avaliação manual, então se 
      // ocorrer como esperado, será mudado.
      DropPadrao::drop(&mut item);
      assert!(true);
   }

   trait TesteFE {
      fn gera() -> Self;
      fn visualiza(&mut self);
   }

   /* implementando uma nova função "gera" com
    * tempos menores para visualizar "expiração"
    * em tempo real. */
   impl TesteFE for FilaExclusao {
      // gera itens baseado no diretório RAÍZ.
      fn gera() -> Self {
         // array-dinâmica.
         let mut lista:Vec<Item> = Vec::new();
         // analisando cada objeto no diretório "Downloads".
         for entry in read_dir("data_teste").unwrap() {
            let entrada = entry.unwrap();
            // se for um diretório ignorar ...
            let e_um_diretorio:bool = {
               entrada
               .path()
               .is_dir()
            };
            if e_um_diretorio { continue; }
            
            // a extensão do arquivo.
            let aux_path = entrada.path();
            let extensao:&str = {
               match aux_path.as_path().extension() {
                  Some(string) => string.to_str().unwrap(),
                  None => { continue; "nem chega aqui!" },
               }
            };
            // computando a 'validade' ...
            let validade:Duration;
            if extensao == "iso" {
               const MESES:u64 = 54;
               validade = Duration::from_secs(MESES);
            } else if extensao == "zip" {
               const ALGUNS_HORAS:u64 = 15;
               validade = Duration::from_secs(ALGUNS_HORAS);
            } else if extensao == "ttf" || extensao == "deb" {
               const ALGUNS_MINUTOS:u64 = 8;
               validade = Duration::from_secs(ALGUNS_MINUTOS);
            } else if extensao == "pdf" {
               const ALGUNS_HORAS:u64 = 20;
               validade = Duration::from_secs(ALGUNS_HORAS);
            } else if extensao == "torrent" {
               const ALGUNS_MINUTOS:u64 = 6;
               validade = Duration::from_secs(ALGUNS_MINUTOS);
            } else if extensao == "dat" || extensao == "djvu" {
               const ALGUNS_MINUTOS:u64 = 5;
               validade = Duration::from_secs(ALGUNS_MINUTOS);
            } else if extensao == "epub" {
               const ALGUNS_DIAS:u64 = 31;
               validade = Duration::from_secs(ALGUNS_DIAS);
            } else if extensao == "tar" || extensao == "gz" {
               const ALGUNS_HORAS:u64 = 16;
               validade = Duration::from_secs(ALGUNS_HORAS);
            } else {
               const PADRAO:u64 = 10;
               validade = Duration::from_secs(PADRAO);
            }
            
            // criando o ítem e adicionando na lista.
            let acesso:SystemTime = SystemTime::now();
            let item = Item::cria(
               entrada.path(),
               acesso,
               validade
            );
            lista.push(item)
         }
         // criando instância em sí, já retornando ...
         return FilaExclusao {
            todos: lista,
            proximas_exclusao: Vec::new()
         }
      }

      fn visualiza(&mut self) {
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
               let hoje:Duration = Duration::from_secs(15);
               tr < hoje 
            };
            if sera_excluido_hoje {
               let item = self.todos.remove(qtd-1);
               self.proximas_exclusao.push(item);
            }
            qtd -= 1;
         }

         /* função pega uma slice-string e imprime-a 
          * centralizando-a baseado no seu tamanho. */
         fn imprime_no_centro<'a>(string:&'a str) {
            // quantia de caractéres da string.
            let tamanho = string.len();
            // largura total do terminal.
            let largura:usize = match terminal_largura() {
               Ok(_enum) => match _enum {
                  TD::Largura(l) => l as usize,
                  _ => 32,
               },
               Err(_) => 32,
            };
            // espaços em branco da borda esquerda.
            let recuo = (largura - tamanho) / 2 - 1;
            println!(
               "{recuo}{}:", 
               string.to_uppercase(), 
               recuo = &" ".repeat(recuo)
            );
         }

         // visualizando lista de todos 'Item's.
         println!("\n");
         imprime_no_centro("lista de items");
         for item in self.todos.iter() 
            { println!("{}", item); }
         imprime_no_centro("exclusão de hoje");
         for item in self.proximas_exclusao.iter() 
            { println!("{}", item); }
         println!("\n");

         let mut qtd = self.proximas_exclusao.len();
         while qtd > 0 {
            let item = self.proximas_exclusao.get_mut(qtd-1).unwrap();
            if item.expirado() 
               { DropPadrao::drop(&mut self.proximas_exclusao.remove(qtd-1)); }
            qtd -= 1;
         }
      }
   }

   fn gera_arquivos_de_teste() {
      let nomes_arquivos = [
         "fonte_i.ttf", "fonte_ii.ttf",
         "texto_i.txt", "texto_ii.txt",
         "registro_i.dat", "registro_ii.dat",
         "livro1.epub", "livro2.epub",
         "livro3.pdf", "OSi.iso", "OSii.iso"
      ];
      for nome in nomes_arquivos {
         let mut caminho = PathBuf::new();
         caminho.push("data_teste");
         caminho.push(nome);
         let mensagem = b"nada de dados relevantes!!!";
         write(caminho.to_path_buf(), mensagem)
         .unwrap();
         thread::sleep(Duration::from_secs(5));
      }
   } 

   #[test]
   fn testa_struct_filaexclusao() {
      // gerando arquivos de teste para exclusão ...
      gera_arquivos_de_teste();
      // executando trecho do código ...
      let mut fe:FilaExclusao = TesteFE::gera();
      for _ in 1..=60 {
         TesteFE::visualiza(&mut fe);
         thread::sleep(Duration::from_secs(1));
      }
      assert!(true);
   }

   #[test]
   fn testa_diretorio_vazio() {
      // criando diretório/e arquivo para testes ...
      let caminho = Path::new("data_teste/pasta_vazia_teste/");
      fs::create_dir_all(caminho).unwrap();   
      assert!(diretorio_vazio(caminho));
      let arq_caminho = caminho.join("arquivo_teste.txt");
      fs::write(
         arq_caminho.as_path(), 
         b"nenhum dado relevante!"
      ).unwrap();
      assert!(!diretorio_vazio(caminho));
      // removendo o diretório e arquivos criados ...
      fs::remove_dir_all(caminho).unwrap();
   }
}
