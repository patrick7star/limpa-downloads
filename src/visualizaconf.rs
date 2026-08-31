/*! Cuida especialmente da parte de visualização(futuramente até setting)
 * das configurações dadas a programa. Tudo roda no ncurses.
 */
 extern crate pancurses;
 extern crate serde_json;
 extern crate utilitarios;

 use pancurses::{Window, COLOR_PAIR, COLOR_WHITE, COLOR_BLACK, COLOR_YELLOW,
   A_BOLD, A_NORMAL};
 use utilitarios::legivel::{string_to_duration, tempo_legivel_duration};
 // Biblioteca padrão do Rust:
 use std::fs::{File};
 use std::io::{self, BufReader};
 use std::time::{Duration};
 use std::collections::{HashMap};

pub fn pagina_configuracao() {
   let janela = pancurses::initscr();

   // Configurando janela iniciada ...
   pancurses::noecho();
   pancurses::curs_set(0);
   pancurses::start_color();
   pancurses::use_default_colors();

   desenha_cabecalho(&janela);
   if let Ok(lista) = obtem_lista_da_validade_das_extensoes()
      { desenha_lista_de_validades(&janela, lista); }
   janela.refresh();
   pancurses::napms(4000);
   pancurses::endwin();
}

fn desenha_cabecalho(janela: &Window)
{
   const MSG: &str = " Configuracao ";
   let (_, x) = janela.get_max_yx();
   let l = MSG.len() as i32;
   let p = (x - l) / 2;

   // Paleta de cores utilizada.
   pancurses::init_pair(21, COLOR_YELLOW, -1);

   janela.draw_box(0, 0);
   janela.attron(COLOR_PAIR(21) | A_BOLD);
   janela.mvaddstr(0, p, &MSG);
   janela.attroff(COLOR_PAIR(21) | A_BOLD);
}

fn obtem_lista_da_validade_das_extensoes() 
  -> io::Result<Vec<(String, Duration)>>
{
   type Dado = HashMap<String, [String; 2]>;

   let arquivo = File::open("definicoes.json")?;
   let reader = BufReader::new(arquivo);
   let mut output: Vec<(String, Duration)> = vec![];
   let dados = serde_json::from_reader::<_, Dado>(reader);

   if let Ok(ref mapa) = dados
   {
      let mut tempo: Duration; 

      for (chave, valor) in mapa.iter()
      { 
         let extensao = chave.to_string();

         if cfg!(debug_assertions)
            { tempo = string_to_duration(&valor[0]).unwrap(); }
         else
            { tempo = string_to_duration(&valor[1]).unwrap(); }

         output.push((extensao, tempo));
      }
   }
   Ok(output)
}

fn desenha_lista_de_validades
  (janela: &Window, mut input: Vec<(String, Duration)>)
{
   const LINHA: i32 = 4;
   const COLUNA: i32 = 11;
   let mut duracaostr: String;
   let mut fmt: String;

   // Inicia paleta de cores:
   pancurses::init_pair(10, COLOR_WHITE, COLOR_YELLOW);
   pancurses::init_pair(11, COLOR_BLACK, COLOR_WHITE);

   janela.mvaddstr(LINHA - 2, COLUNA, "Todas validades definidas pra extensoes:");
   for (p, (mut extensao, duracao)) in input.drain(..).enumerate() 
   {
      extensao.make_ascii_uppercase();
      fmt = format!("{:^9} ", extensao);
      duracaostr = tempo_legivel_duration(duracao, true); 
      janela.attron(COLOR_PAIR(10) | A_BOLD);
      janela.mvaddstr(LINHA + p as i32, COLUNA, fmt);
      janela.attroff(COLOR_PAIR(10) | A_BOLD);
      fmt = format!("{duracaostr:^12}");
      janela.attron(COLOR_PAIR(11) | A_NORMAL);
      janela.addstr(fmt);
      janela.attroff(COLOR_PAIR(11) | A_NORMAL);
   }
}


#[cfg(test)]
mod tests {

   #[test]
   fn prototipo_da_pagina_de_configuracao()
   {
      super::pagina_configuracao(); 
   }

   #[test]
   fn listagem_dos_tempos_determinados_para_extensoes()
   {
      let lista = super::obtem_lista_da_validade_das_extensoes();
      for (key, value) in lista.unwrap().drain(..)
         { println!("\t- {} {:?}",key, value); }
   }
}

