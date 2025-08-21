/**! Gera a parte de visualização com uma biblioteca semi-gráfica `ncurses`. 
 Que é o mesmo que a impressão no `stdout`, porém de forma dinâmica e 
 animada. 
*/

extern crate pancurses;
extern crate utilitarios;

// Biblioteca externa:
use pancurses::{ 
   endwin, napms, initscr, noecho, curs_set, start_color, init_pair, 
   use_default_colors, Input, Window, COLOR_GREEN, COLOR_RED, COLOR_YELLOW, 
   COLOR_CYAN, A_NORMAL, A_UNDERLINE, A_BOLD, A_REVERSE, COLOR_MAGENTA,
   COLOR_BLUE
};
use utilitarios::{
   impressao::circunscrever,
   legivel::{tempo_legivel_duration, DIA},
   aleatorio::sortear
};
// Módulos do próprio projeto:
use crate::item_de_exclusao::{Item, FilaExclusao};
use super::letreiro::StringDinamica;
use super::notificacoes;
// Biblioteca padrão do Rust:
use std::time::{Duration, Instant};
use std::str::FromStr;

// ID's de todas paletas de cores criadas e utilizadas:
static LONGE:     i16  = 99;
static PERTO:     i16  = 98;
static MEDIO:     i16  = 97;
static LI_COR:    i16  = 96;
static LEH_COR:   i16  = 95;

pub trait Grafico {
   /** O mesmo que o método original, porém de forma dinâmica; com coloração
    * e etc.  */
   fn visualiza_provavel(&mut self);

   fn visualiza_certeza(&mut self);
}

pub trait DropGrafico {
   /** implementação do `Drop`, porém
    com dinâmica de saída no 'ncurses' */
   fn drop(&mut self, janela:&Window);
}

enum Temas {
   Verde(u8),
   Vermelho(u8),
   Amarelo(u8),
   AzulMarinho(u8),
   Violenta(u8),
   // Primeiro é a fonte do texto, seguido do seu fundo.
   Papeis(u8, u8)
}


/* string específica do `Item` e extrai sua porcentagem, que fica 
 * geralmente no final. Retorna tal valor como um float. */
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

/* Desenha na tela do 'ncurses' de modo colorido. */
fn item_visualizacao(janela:&Window, item:&Item) {
   // próxima linha ...
   let l = janela.get_cur_y() + 1;
   let item_str: &String = &StringDinamica::to_string(item);
   let i = item_str.rfind("[").unwrap();
   let f = item_str.rfind("]").unwrap();
   let percentual = extrai_percentual(item_str.as_str());

   janela.mv(l, 0);
   janela.addnstr(item_str, i + 1); 
   janela.attrset(A_BOLD);

   if percentual >= 75.0
      { janela.color_set(94); }
   else if percentual >= 15.0 && percentual < 50.0 
      { janela.color_set(MEDIO); }
   else if percentual < 15.0 
      { janela.color_set(PERTO); }
   else 
      { janela.color_set(LONGE); }

   janela.addnstr(item_str.get(i+1..).unwrap(), f-i-1);
   janela.attroff(A_BOLD);
   janela.color_set(0);
   janela.addnstr(item_str.get(f..).unwrap(), item_str.len() - f);
}

/* Função pega uma slice-string e imprime-a centralizando-a baseado no 
 * seu tamanho. */
fn cabecalho<'a>(string:&'a str, janela:&Window, cor:i16) {
   // quantia de caractéres da string.
   let tamanho:i32 = string.len() as i32;
   // largura total do terminal.
   let largura = janela.get_max_x();
   // espaços em branco da borda esquerda.
   let coluna = (largura - tamanho) / 2 - (1 + 3);
   // movendo cursor ...
   let linha = janela.get_cur_y() + 3;

   // Desenhado na tela ...
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

fn escreve_listas
 (janela:&Window, todos:&mut Vec<Item>, proximas_exclusao:&mut Vec<Item>) 
{
   // primeira lista:
   cabecalho("lista de items", janela, LI_COR);
   // mensagem em caso de está parte da lista está vázia.
   if todos.is_empty() {
      let informacao = circunscrever("nenhum item aqui para lista!");
      let col: i32 = {
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
         "nenhum item a excluir hoje!"
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

/* Implementando métodos privados que complementam o método público 
 * "visualizar", porque é sempre bom tem um código mais limpo e 
 * organizado. 
 */
impl FilaExclusao {
   /* Pondo 'Item's que estão prestes a ser deletados, na fila de exclusão.
    */
   fn reordenacao_dos_items(&mut self) {
      let mut quantia = self.todos.len();
      let lista = &mut self.todos;

      while quantia > 0 {
         // na fila de exclusão de hoje.
         let sera_excluido_hoje:bool = {
            let last = quantia - 1;
            let item = lista.get_mut(last).unwrap();

            item.tempo_restante() < Duration::from_secs_f32(DIA) 
         };

         if sera_excluido_hoje {
            let indice = quantia - 1;
            let item = lista.remove(indice);

            self.proximas_exclusao.push(item);
         }
         quantia -= 1;
      }
   }

   // Realiza limpa de ítems expirados.
   fn limpa_items_expirados(&mut self, janela: Window) -> Window {
      let mut quantidade = self.proximas_exclusao.len();

      while quantidade > 0 {
         let item = {
            self.proximas_exclusao
            .get_mut(quantidade - 1).unwrap()
         };
         if item.expirado() { 
            let mut remocao = {
               self.proximas_exclusao
               .remove(quantidade - 1)
            };
            DropGrafico::drop(&mut remocao, &janela); 
            napms(700);
         }
         quantidade -= 1;
      }
      // devolve janela depois de realizar alguns rascunhos ...
      return janela;
   }

   fn construcao_e_renderizacao(&mut self, janela: &Window,
     ordem_de_saida: Option<&mut bool>, framerate: i32) 
   {
      let linha = janela.get_max_y() - 1;
      let coluna = janela.get_max_x() - 30;

      janela.erase();
      escreve_listas(
         &janela, 
         &mut self.todos, 
         &mut self.proximas_exclusao
      );
      self.constroi_status(janela);

      // Controle do painel em execução ...
      match janela.getch() {
         Some(Input::Character(ch)) => {
            // sair do programa.
            if ch == 's' || ch == 'S' || ch == 'q' || ch == 'Q' { 
               match ordem_de_saida {
                  Some(estado) => 
                     { *estado = true; }
                  None => {}
               }
            }
         } 
         Some(Input::KeyDown) => 
            { janela.mvaddstr(linha, coluna, "para BAIXO!"); }
         Some(Input::KeyUp) => 
            { janela.mvaddstr(linha, coluna, "para CIMA!"); }
         Some(Input::KeyRight | Input::KeyLeft) => 
            { janela.mvaddstr(linha, coluna, "Nao implementado!"); }
         _=> ()
      };

      // Atualiza nova escrita, e deleta ítems que expiraram recentemente.
      janela.refresh(); 
      // Taxa de quadros por segundo da interface.
      napms(framerate);
   }

   fn constroi_status(&mut self, janela: &Window) {
      let linha = janela.get_max_y() - 1;
      let mut cursor = 0;
      const ESPACO: i32 = 2;
      
      if !self.proximas_exclusao.is_empty() { 
         let quantia = self.proximas_exclusao.len();
         let formatacao = format!("Exclusao: {}", quantia);

         janela.mvaddstr(linha, 2, formatacao.as_str()); 
         cursor = ESPACO + formatacao.len() as i32 + ESPACO;
      }

      let quantia = self.todos.len();
      let formatacao = format!("Totos itens: {}", quantia);

      janela.mvaddstr(linha, cursor, &formatacao); 
      cursor += formatacao.len() as i32 + ESPACO;

      if cfg!(debug_assertions) {
         let fmt = " modo debug ";

         janela.attron(A_REVERSE | A_UNDERLINE);
         janela.color_set(98);
         janela.mvaddstr(linha, cursor, fmt); 
         janela.color_set(0);
         janela.attroff(A_REVERSE | A_UNDERLINE);
      }
   }
}

// mostra o contador do tempo restante de exibição do programa gráfico.
fn escreve_temporizador
  (janela: &Window, todo: Duration, contador: &Instant) 
{
   // Contagem regressiva.
   let zero = Duration::new(0, 0);
   let r = todo.checked_sub(contador.elapsed()).unwrap_or(zero);
   let tempo_str = tempo_legivel_duration(r, true);
   let c = tempo_str.len() as i32;

   // desenha na janela referênciada.
   janela.mvaddstr(
      // Coordenadas Y e X:
      janela.get_max_y() - 2,
      janela.get_max_x() - (c + 3),
      // Texto formatado e "traduzido".
      tempo_str.as_str()
   );
}

fn configuracao_da_janela_principal(janela: &Window) {
   // configurando janela.
   noecho();
   curs_set(0);
   start_color();
   use_default_colors();
   janela.keypad(true);
   janela.timeout(400);
}

fn iniciando_todas_paletas_de_cores() {
   // paleta de cores:
   init_pair(99, COLOR_GREEN,    -1);
   init_pair(98, COLOR_RED,      -1);
   init_pair(97, COLOR_YELLOW,   -1);
   init_pair(96, COLOR_CYAN,     -1);
   init_pair(95, COLOR_MAGENTA,  -1);
   init_pair(94, COLOR_BLUE,     -1);
}

impl Grafico for FilaExclusao 
{
   fn visualiza_provavel(&mut self) { 
      let mut janela = initscr();
      
      configuracao_da_janela_principal(&janela);
      iniciando_todas_paletas_de_cores();

       /* Quando está sem itens, às vezes, apenas plota uma notificação,
        * com um aviso de que "não há nada à excluir". */
      if self.vazia() {
         /* Uma em cada dez, mostra a interface. não quero, por 
          * enquanto, desabilitar este antigo 'feature' por completo. */
         if sortear::u8(1..=10) == 5 
            { self.construcao_e_renderizacao(&janela, None, 3500); }
         else {
            /* Finaliza a parte gráfica e lança uma notificação informando
             * a situação sem itens para deletar. */
            endwin();
            notificacoes::avisa_de_diretorio_esta_vazio();
         } 
      }

      let timer = Instant::now();
      let duracao = Duration::from_secs(80);

      while !self.vazia() {
         // reordena ítens de de ambas listas.
         self.reordenacao_dos_items();
         /* Visualizando lista de todos 'Item's. Apaga tudo já escrito na 
          * janela. Imprime ambos tipos de listagens: */
         janela.erase();
         escreve_listas(
            &janela, 
            &mut self.todos, 
            &mut self.proximas_exclusao
         );
         // Controle do painel em execução ...
         match janela.getch() {
            Some(Input::Character(ch)) => {
               // sair do programa.
               if ch == 's' || ch == 'S' || ch == 'q' || ch == 'Q'
                  { break; }
            } Some(Input::KeyDown) => 
               { janela.mvaddstr(0, 0, "para BAIXO!"); }
            Some(Input::KeyUp) => 
               { janela.mvaddstr(0, 0, "para CIMA!"); }
            _ => ()
         };

         if !self.ha_exclusao_hoje() {
            escreve_temporizador(&janela, duracao, &timer);
            // quebra loop se o temporizador "se esgota".
            if timer.elapsed() > duracao { break; }
         }

         // Atualiza nova escrita, e deleta ítems que expiraram recentemente.
         janela.refresh(); 
         janela = self.limpa_items_expirados(janela);

         // Taxa de quadros por segundo da interface.
         napms(500);
      }
      endwin();
   }

   fn visualiza_certeza(&mut self) { 
      let mut janela = initscr();
      let timer = Instant::now();
      let duracao = Duration::from_secs(80);
      let mut abandonar_o_loop: bool = false;
      
      configuracao_da_janela_principal(&janela);
      iniciando_todas_paletas_de_cores();
      self.construcao_e_renderizacao(&janela, None, 3500);

      while !self.vazia() {
         // Reordena ítens de de ambas listas.
         self.reordenacao_dos_items();
         /* Visualizando lista de todos 'Item's. Apaga tudo já escrito na 
          * janela. Imprime ambos tipos de listagens. Ele também muda o
          * estado de permanência do 'loop'; isso, porque tal rotina 
          * controla o 'console' e a entrada de dados da interface. */
          self.construcao_e_renderizacao
            (&janela, Some(&mut abandonar_o_loop), 500);

         if abandonar_o_loop { break; }

         if !self.ha_exclusao_hoje() {
            escreve_temporizador(&janela, duracao, &timer);
            // quebra loop se o temporizador "se esgota".
            if timer.elapsed() > duracao { break; }
         }

         // Deleta ítems que expiraram recentemente.
         janela = self.limpa_items_expirados(janela);
      }
      endwin();
   }
}

// exclusão do ítem em sí.
impl DropGrafico for Item {
   /* apenas acaba se, e somente se, o item expirou. */
   fn drop(&mut self, janela:&Window) {
      if self.expirado() { 
         let largura = janela.get_max_x();
         const MARGEM: i32 = 19i32;
         let comprimento = self.nome.len() as i32;
         let indice = (largura - MARGEM) as usize;
         let fmt: String;

         // nome da string de forma mais conveniênte.
         if comprimento > largura { 
            let nome = self.nome.get(0..indice).unwrap(); 
            fmt = format!("\"{}\"...", nome);
         } else { 
            let nome = self.nome.as_str(); 
            fmt = format!("\"{}\"", nome);
         }
         
         // Apaga toda linha.
         janela.mv(1, 1);
         janela.deleteln();
         janela.insertln();
         janela.addstr("==> removendo ");
         // Adiciona nome do arquivo na linha escrita.
         janela.addstr(fmt.as_str());
         janela.addch('[');
         janela.color_set(99);
         janela.addstr("SUCEDIDO");
         janela.color_set(0);
         janela.addch(']');
         janela.refresh();

         // deletando arquivos e restos em sí.
         let _= self;
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
