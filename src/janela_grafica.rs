
/**! 
 Gera a parte de visualização com uma biblioteca
 semi-gráfica `ncurses`. Que é o mesmo que a
 impressão no `stdout`, porém de forma dinâmica 
 e animada. 
*/

// biblioteca externa:
extern crate pancurses;
use pancurses::{ 
   endwin, napms, initscr, 
   noecho, curs_set, start_color, 
   init_pair, use_default_colors, 
   doupdate, Input, Window, COLOR_GREEN,
   COLOR_RED, COLOR_YELLOW,
   COLOR_CYAN, A_NORMAL, 
   A_UNDERLINE, A_BOLD
};
extern crate utilitarios;
use utilitarios::impressao::circunscrever;

// minha biblioteca:
use crate::item_de_exclusao::{Item, FilaExclusao};
use super::letreiro::StringDinamica;

// biblioteca padrão do Rust:
use std::time::Duration;
use std::str::FromStr;

// ID's de todas paletas de cores criadas e utilizadas:
static LONGE:i16 = 99;
static PERTO:i16 = 98;
static MEDIO:i16 = 97;
static LI_COR:i16 = 96;
static LEH_COR:i16 = 95;


pub trait Grafico {
   /** o mesmo que o método original, porém 
    de forma dinâmica; com coloração e etc.  */
   fn visualiza(&mut self);
}

pub trait DropGrafico {
   /** implementação do `Drop`, porém
    com dinâmica de saída no 'ncurses' */
   fn drop(&mut self, janela:&Window);
}


/* string específica do `Item` e extrai sua
 * porcentagem, que fica geralmente no final.
 * Retorna tal valor como um float. */
fn extrai_percentual(item_str:&str) -> f32 {
   // tamanho da string.
   let tamanho = item_str.len();
   let valor_str = item_str.get(tamanho-6..tamanho-1);
   // convertendo para float e retornando ...
   match f32::from_str(valor_str.unwrap().trim()) {
      Ok(v) => v,
      Err(_) => { 
         dbg!(valor_str); 
         panic!(""); 
      }
   }
}

// colore impressão do `Item`. 
fn item_visualizacao(janela:&Window, item:&Item) {
   // próxima linha ...
   let l = janela.get_cur_y() + 1;
   janela.mv(l, 0);
   //let item_str = &item.to_string();
   let item_str: &String = &StringDinamica::to_string(item.clone());
   let i = item_str.rfind("[").unwrap();
   let f = item_str.rfind("]").unwrap();
   janela.addnstr(item_str, i+1); 
   janela.attrset(A_BOLD);
   let percentual = extrai_percentual(item_str.as_str());
   if percentual >= 15.0 && percentual < 50.0 {
      janela.color_set(MEDIO);
   }
   else if percentual < 15.0 {
      janela.color_set(PERTO);
   }
   else {
      janela.color_set(LONGE);
   }
   janela.addnstr(item_str.get(i+1..).unwrap(), f-i-1);
   janela.attroff(A_BOLD);
   janela.color_set(0);
   janela.addnstr(item_str.get(f..).unwrap(), item_str.len() - f);
}

/* função pega uma slice-string e imprime-a 
 * centralizando-a baseado no seu tamanho. */
fn cabecalho<'a>(string:&'a str, janela:&Window, cor:i16) {
   // quantia de caractéres da string.
   let tamanho:i32 = string.len() as i32;
   // largura total do terminal.
   let largura = janela.get_max_x();
   // espaços em branco da borda esquerda.
   let coluna = (largura - tamanho) / 2 - (1 + 3);
   // movendo cursor ...
   let linha = janela.get_cur_y() + 3;
   // desenhado na tela ...
   janela.attrset(A_UNDERLINE);
   janela.color_set(cor);
   janela.mvaddstr(
      linha, coluna, 
      string.to_uppercase()
   );
   janela.color_set(0);
   janela.attrset(A_NORMAL);
   // movendo cursor uma linha abaixo ...
   janela.mv(janela.get_cur_y() + 1, 0);
}

fn escreve_listas(janela:&Window, todos:&mut Vec<Item>,
proximas_exclusao:&mut Vec<Item>) {
   // primeira lista:
   cabecalho("lista de items", janela, LI_COR);
   // mensagem em caso de está parte da lista está vázia.
   if todos.is_empty() {
      let informacao = circunscrever("nenhum item aqui para lista!");
      let col:i32 = {
         let a = janela.get_max_x();
         let b = informacao.find("\n").unwrap() as i32;
         (a - b) / 2 
      };
      let mut lin = janela.get_cur_y();
      for linha in informacao.lines() {
         janela.mvaddstr(lin, col - 3, linha);
         lin += 1;
      }
   }
   for item in todos.iter() 
      { item_visualizacao(&janela, item); }

   // segunda lista:
   cabecalho("exclusao de hoje", janela, LEH_COR);
   // mensagem em caso de está parte da lista está vázia.
   if proximas_exclusao.is_empty() {
      let informacao = circunscrever(
         "nenhum item ha excluir hoje!"
      );
      let col:i32 = {
         let a = janela.get_max_x();
         let b = informacao.find("\n").unwrap() as i32;
         (a - b) / 2 
      };
      let mut lin = janela.get_cur_y();
      for linha in informacao.lines() {
         janela.mvaddstr(lin, col - 3, linha);
         lin += 1;
      }
   }
   for item in proximas_exclusao.iter() 
      { item_visualizacao(&janela, item); }
}

/* Implementando métodos privados que complementam
 * o método público "visualizar", porque é sempre 
 * bom tem um código mais limpo e organizado. */
impl FilaExclusao {
   /* pondo 'Item's que estão prestes a 
    * ser deletados, na fila de exclusão.
    */
   fn reordenacao_dos_items(&mut self) {
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
   }

   // Realiza limpa de ítems expirados.
   fn limpa_items_expirados(&mut self, janela:Window) -> Window {
      let mut qtd = self.proximas_exclusao.len();
      while qtd > 0 {
         let item = {
            self.proximas_exclusao
            .get_mut(qtd-1).unwrap()
         };
         if item.expirado() { 
            let mut remocao = {
               self.proximas_exclusao
               .remove(qtd-1)
            };
            DropGrafico::drop(&mut remocao, &janela); 
            napms(700);
         }
         qtd -= 1;
      }
      // devolve janela depois de realizar alguns rascunhos ...
      return janela;
   }
}

impl Grafico for FilaExclusao {
   fn visualiza(&mut self) { 
      // janela padrão que se ajusta com o terminal.
      let mut janela = initscr();

      // configurando janela.
      noecho();
      curs_set(0);
      start_color();
      use_default_colors();
      janela.keypad(true);
      janela.timeout(400);

      // paleta de cores:
      init_pair(99, COLOR_GREEN, -1);
      init_pair(98, COLOR_RED, -1);
      init_pair(97, COLOR_YELLOW, -1);
      init_pair(96, COLOR_CYAN, -1);
      init_pair(95, COLOR_RED, -1);

      // se já inicializar vázio, dá um tempo para mostrar
      // a interface específica deste caso.
      if self.vazia() {
         // imprime ambos tipos de listagens:
         escreve_listas(
            &janela, 
            &mut self.todos, 
            &mut self.proximas_exclusao
         );
         // mostra resultado novo da tela.
         janela.refresh();
         // três segundos e meio de espera.
         napms(3_500);
      }

      // rodar ncurses até a fila esváziar.
      while !self.vazia() {
         // reordena ítens de de ambas listas.
         self.reordenacao_dos_items();
         // visualizando lista de todos 'Item's.
         // apaga tudo já escrito na janela.
         janela.erase();
         // imprime ambos tipos de listagens:
         escreve_listas(
            &janela, 
            &mut self.todos, 
            &mut self.proximas_exclusao
         );
         // aceitando alguns comandos...
         match janela.getch() {
            Some(Input::Character(ch)) => {
               // sair do programa.
               if ch == 's'
                  { break; }
            },
            Some(Input::KeyDown) => {
               janela.mvaddstr(0, 0, "para BAIXO!");
            },
            Some(Input::KeyUp) => {
               janela.mvaddstr(0, 0, "para CIMA!");
            },
            _ => (),
         };
         // atualiza nova escrita.
         doupdate();
         // deleta ítems que expiraram recentemente.
         janela = self.limpa_items_expirados(janela);

         // a cadá tanto milisegundos.
         napms(500);
      }
      // terminal tal janela.
      endwin();
   }
}

// exclusão do ítem em sí.
impl DropGrafico for Item {
   /* apenas acaba se, e somente se, 
    * o item expirou. */
   fn drop(&mut self, janela:&Window) {
      if self.expirado() { 
         // movendo para linha debaixo ...
         let linha = janela.get_cur_y();
         // explicitando-se o que vai fazer.
         janela.mvaddstr(linha + 1, 0, "==> removendo");

         // nome da string de forma mais conveniênte.
         let nome:String = {
            //let lt = janela.get_max_x();
            //let c:i32 = self.nome.len() as i32 + 19;
            let (lt, c):(i32, i32) = (
               janela.get_max_x(),
               (self.nome.len() as i32) + 19
            );
            if c > lt { 
               let indice: usize;
               indice = (lt as usize) - 5;
               let parte_str = self.nome.get(0..indice);
               format!("\"{}\"...", parte_str.unwrap()) 
            }
            else 
               { format!("\"{}\"", self.nome) }
         };

         // adiciona nome do arquivo na linha escrita.
         janela.addstr(nome);
         janela.addch('[');
         janela.color_set(99);
         janela.addstr("SUCEDIDO");
         janela.color_set(0);
         janela.addch(']');
         janela.refresh();

         // deletando arquivos e restos em sí.
         drop(self);
      }
   }
}


#[cfg(test)]
mod tests {
   use super::*;
   use std::fs::{create_dir, write};
   use std::time::SystemTime;
   use std::env::temp_dir;

   #[test]
   fn testa_extrai_percentual() {
      // criando arquivos aleatórios.
      let caminho = temp_dir().as_path().join("data_teste");
      // criando diretório primeiramente ...
      create_dir(caminho.as_path()).unwrap();
      write(caminho.as_path().join("fonte.ttf"),b"nada").unwrap();
      write(caminho.as_path().join("teste.dat"), b"nada").unwrap();

      let i = Item::cria(
         caminho.as_path()
         .join("teste.dat")
         .to_path_buf(),
         SystemTime::now(),
         Duration::from_secs(15)
      );
      let _i2 = Item::cria(
         caminho.as_path()
         .join("fonte.ttf.dat")
         .to_path_buf(),
         SystemTime::now(),
         Duration::from_secs(15)
      );

      let progresso = StringDinamica::to_string(&i);
      let p = dbg!(extrai_percentual(progresso.as_str()));
      assert!(p >= 95.5 && p <= 100.0);
   }
}
