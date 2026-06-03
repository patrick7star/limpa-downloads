#![allow(unused)]
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
   COLOR_BLUE, COLOR_WHITE, COLOR_BLACK
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
use std::collections::{VecDeque};

// Apelidos para facilitar na codificação.
type ItemPartes<'m> = (&'m str, &'m str, &'m str, &'m str);
// ID's de todas paletas de cores criadas e utilizadas:
static LONGE:     i16  = 99;
static PERTO:     i16  = 98;
static MEDIO:     i16  = 97;
static LI_COR:    i16  = 96;
static LEH_COR:   i16  = 95;
// Número das paletas de cores inicializada.
const VERDE:         i16 = 99;
const VERMELHO:      i16 = 98;
const AMARELO:       i16 = 97;
const AZULMARINHO:   i16 = 96;
const VIOLETA:       i16 = 95;
const AZUL:          i16 = 94;

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
   const MENSAGEM_I: &str = "nenhum item aqui para lista!";
   const MENSAGEM_II: &str ="nenhum item a excluir hoje!";
   let (mut lin, mut col): (i32, i32) = (0, 0);
   let mut informacao = String::new();
   let (col_max, mut x):(i32, i32) = (janela.get_max_x(), 0);

   // primeira lista:
   cabecalho("lista de items", janela, LI_COR);

   // mensagem em caso de está parte da lista está vázia.
   if todos.is_empty() {
      informacao = circunscrever(MENSAGEM_I);
      x = informacao.find("\n").unwrap() as i32;
      col = (col_max - x) / 2;
      lin = janela.get_cur_y();

      for linha in informacao.lines() {
         janela.mvaddstr(lin, col - 3, linha);
         lin += 1;
      }
   }

   for item in todos.iter() 
      { item_visualizacao(&janela, item); }

   cabecalho("exclusao de hoje", janela, LEH_COR);

   // mensagem em caso de está parte da lista está vázia.
   if proximas_exclusao.is_empty() 
   {
      informacao = circunscrever(MENSAGEM_II);
      x = informacao.find("\n").unwrap() as i32;
      col = (col_max - x)  / 2; 
      lin = janela.get_cur_y();

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
      type Fila = VecDeque<String>;

      let linha = janela.get_max_y() - 1;
      let mut cursor = 0;
      let mut displays = Fila::with_capacity(4);
      let espaco = " ".repeat(1);
      
      // Fila de visualizações:
      displays.push_back(format!("Exclusao: {}", self.total()));
      displays.push_back(format!("Todos itens: {}", self.todos.len()));
      displays.push_back(
         String::from(
            if cfg!(debug_assertions) 
               { "modo debug" }
            else { "Normal" }
         )
      );
      displays.push_back(String::from("<S | Q pra sair>"));

      // Começando a desenhar na tela buffer do terminal.
      janela.mv(linha, 2);
      janela.color_set(79);
      janela.addstr(&espaco);
      janela.addstr(&displays.pop_front().unwrap()); 
      janela.addstr(&espaco);

      janela.color_set(77);
      janela.addstr(&espaco);
      janela.addstr(&displays.pop_front().unwrap()); 
      janela.addstr(&espaco);

      janela.color_set(88);
      janela.addstr(&espaco);
      janela.attron(A_UNDERLINE);
      janela.addstr(&displays.pop_front().unwrap()); 
      janela.attroff(A_UNDERLINE);
      janela.addstr(&espaco);

      janela.color_set(76);
      janela.addstr(&espaco);
      janela.addstr(&displays.pop_front().unwrap()); 
      janela.addstr(&espaco);
      janela.color_set(0);
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
   // Paleta de cores com transparência:
   init_pair(99, COLOR_GREEN,    -1);
   init_pair(98, COLOR_RED,      -1);
   init_pair(97, COLOR_YELLOW,   -1);
   init_pair(96, COLOR_CYAN,     -1);
   init_pair(95, COLOR_MAGENTA,  -1);
   init_pair(94, COLOR_BLUE,     -1);

   // Paletas de cores referentes aos planos de fundo.
   init_pair(89, COLOR_WHITE, COLOR_GREEN);
   init_pair(88, COLOR_WHITE, COLOR_RED);
   init_pair(87, COLOR_WHITE, COLOR_YELLOW);
   init_pair(86, COLOR_WHITE, COLOR_CYAN);
   init_pair(85, COLOR_WHITE, COLOR_MAGENTA);
   init_pair(84, COLOR_WHITE, COLOR_BLUE);
   // O mesmo, porém com uma fonte preta.
   init_pair(79, COLOR_BLACK, COLOR_GREEN);
   init_pair(78, COLOR_BLACK, COLOR_RED);
   init_pair(77, COLOR_BLACK, COLOR_YELLOW);
   init_pair(76, COLOR_BLACK, COLOR_CYAN);
   init_pair(75, COLOR_BLACK, COLOR_MAGENTA);
   init_pair(74, COLOR_BLACK, COLOR_BLUE);
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
      let mut abandonar_o_loop = false;
      
      configuracao_da_janela_principal(&janela);
      iniciando_todas_paletas_de_cores();
      self.construcao_e_renderizacao(&janela, None, 3500);

      while !self.vazia() {
         // Busca por novos itens em tempo de execução.
         self.varredura();
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

fn decompoe_em_partes<'a>(entrada: &'a str) -> ItemPartes<'a> 
{
   // Partes dos colchetes(o referêncial aqui são os colchetes da barra):
   let a = entrada.rfind('[').unwrap();   
   let b = entrada.rfind(']').unwrap();   
   let c = entrada.find(' ').unwrap();
   let percentual = entrada.get(b + 1..).unwrap();
   let progresso = entrada.get((a - 1)..(b + 1)).unwrap();
   let nome = entrada.get(0..(c + 1)).unwrap();
   let mut estimado = entrada.get(c..a).unwrap();

   // Retirando espaços branos em ambas pontas.
   estimado = estimado.trim().trim_end();
   (nome, estimado, progresso, percentual)
}

fn define_cor_para_percentual(percentual: f32, janela:&Window) 
{
   if percentual >= 75.0
      { janela.color_set(94); }
   else if percentual >= 15.0 && percentual < 50.0 
      { janela.color_set(MEDIO); }
   else if percentual < 15.0 
      { janela.color_set(PERTO); }
   else 
      { janela.color_set(LONGE); }
}

fn desenha_barra_de_progresso(entrada: &str, janela: &Window, percentual: f32) 
{
   let length = entrada.len();
   let ultimo = length - 1;
   let delimitador_direito = entrada.get(ultimo..).unwrap();
   let delimitador_esquerdo = entrada.get(0..2).unwrap();
   let barras = entrada.get(2..ultimo).unwrap();

   janela.attrset(A_BOLD);
   janela.addstr(delimitador_esquerdo);
   define_cor_para_percentual(percentual, janela);
   janela.addstr(barras);
   janela.color_set(0);
   janela.addstr(delimitador_direito);
   janela.attroff(A_BOLD);
   janela.color_set(0);
}

fn item_visualizacao(janela:&Window, item:&Item) {
   const MARGEM: i32 = 4;
   // próxima linha ...
   let lin = janela.get_cur_y() + 1;
   let col = janela.get_max_x() + 1;
   let item = &StringDinamica::to_string(item);
   let percentual = extrai_percentual(item.as_str());
   let decomposicao = decompoe_em_partes(&item);
   // Dividindo de modo certo, como nomes reconheciveis.
   let nome             = decomposicao.0;
   let estimado         = decomposicao.1;
   let progresso        = decomposicao.2;
   let percentual_fmt   = decomposicao.3;
   let total = (
      estimado.len() + progresso.len() + 
      percentual_fmt.len() 
   ) as i32;

   // Começar a escrever à partir daqui.
   janela.mv(lin, MARGEM);
   janela.attrset(A_BOLD);
   janela.addstr(nome);
   // Move este tanto antes da lateral direita.
   janela.mv(lin, col - total - MARGEM);
   janela.attrset(A_UNDERLINE);
   janela.addstr(estimado);
   janela.attroff(A_UNDERLINE);
   desenha_barra_de_progresso(progresso, janela, percentual);
   janela.addstr(percentual_fmt);
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
