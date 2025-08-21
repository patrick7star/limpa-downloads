extern crate serde_json;
extern crate utilitarios;

// Bibliotecas externas:
use serde_json::{Deserializer, Map, Value};
use utilitarios::legivel::{interpleta_string_de_tempo};
// Biblioteca padrão do Rust:
use std::fs::{File};
use std::time::{Duration};
use std::path::{Path};
// Módulos/e submódulos do próprio projeto:
use crate::item_de_exclusao::{diretorio_esta_vazio};
use crate::links::{computa_caminho};

type TipoJSON = Map<String, Value>;

static mut TABELA_EXTENSOES: Option<TipoJSON> = None;
const NOME_CONFIG: &str = "definicoes.json";


#[allow(static_mut_refs)]
fn carrega_dicionario_com_definicoes() -> TipoJSON {
   /* Se já tiver sido carregado tais definições, apenas retorna uma cópia
    * do valor. */
   unsafe {
      if TABELA_EXTENSOES.is_some() 
         { return TABELA_EXTENSOES.clone().unwrap().clone() }; 
   }

   let caminho = computa_caminho(NOME_CONFIG);
   println!("{}", caminho.display());
   let arquivo = File::open(caminho).unwrap(); 
   let objeto = Deserializer::from_reader(arquivo); 

   for primeira_entrada in objeto.into_iter::<TipoJSON>() { 
      match primeira_entrada {
         Ok(mapa) => { unsafe { TABELA_EXTENSOES = Some(mapa); } }
         Err(erro) => { panic!("{}", erro); }
      }
   }
   
   unsafe { TABELA_EXTENSOES.clone().unwrap().clone() }
}

fn unwrap_such_value_array(entrada: Value) -> String {
   let msg_erro: &'static str;
   let msg_erro_a: &'static str;

   // Escrevendo mensagens de erros:
   msg_erro = "Tipo de definição inválida no arquivo de configuração JSON";
   msg_erro_a = "Apenas aceita listas ou strings na definição do JSON";

   match entrada {
      Value::Array(lista) => {
         let selecionado = {
            if cfg!(debug_assertions)
               { lista[1].clone() }
            else
               { lista[0].clone() }
         };

         match selecionado {
            Value::String(data) => data,
            _ => { panic!("{}", msg_erro); }
         }
      } Value::String(data) => { data }
      _ => 
         { panic!("{}.", msg_erro_a); }
   }
}

// Computa a devida 'duração' para a devida extensão.
#[allow(dead_code)]
pub fn duracao_para_devida_extensao(extension: &str) -> Duration {
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

/* O mesmo que o acima, porém apenas pega a validade, baseado no tipo de
 * binário, ao invés de defini-lo no próprio código, que é uma total mal
 * prática. Isso também faz muito mais flexível adicionar, mudar ou tirar
 * extensões que não deseja. */
pub fn duracao_para_devida_extensao_por_json(extension: &str) -> Duration {
   let definicoes = carrega_dicionario_com_definicoes();
   let valor: String;
   const PADRAO: &str = "20min";

   /* Quaquer uma que não existe, pega um simples valor padrão de 20 min. */
   if let Some(escolha) = definicoes.get(extension) 
      { valor = unwrap_such_value_array(escolha.clone()); }
   else
      { valor = String::from(PADRAO); }

   interpleta_string_de_tempo(&valor).unwrap()
}

// Computa duração para um diretório, seja qual for seu estado.
#[allow(non_snake_case)]
pub fn duracao_para_diretorio(caminho: &Path) -> Duration {
   const DIR_VAZIA: u64 = 5 * 60;
   let DIA: f32;
   let ALGUNS_DIAS: u64;

   if cfg!(debug_assertions) 
      { DIA = 60.0; } 
   else 
      { DIA = 24.0 * 3600.0; }
   ALGUNS_DIAS = (DIA * 13.9) as u64;

   // criando o ítem e adicionando na lista.
   if diretorio_esta_vazio(caminho)
      { Duration::from_secs(DIR_VAZIA) }
   else 
      { Duration::from_secs(ALGUNS_DIAS) }
}


#[cfg(test)]
mod tests {
   use super::*;


   #[test]
   fn carrega_definicoes_de_validades() {
      let caminho = computa_caminho(NOME_CONFIG);
      let arquivo = File::open(&caminho).unwrap(); 
      let objeto = Deserializer::from_reader(arquivo); 

      for entry in objeto.into_iter::<TipoJSON>() { 
         if let Ok(ref item) = entry
         { 
            for (key, value) in item
               { println!("{} - {:#?}", key, value); }
         } 
      }
   }

   #[test]
   fn funcao_especializada_em_carregar_as_definicoes() {
      let dicio = carrega_dicionario_com_definicoes();

      for chave in dicio.keys() { 
         println!("{} - {:?}", chave, dicio[chave]);
      }
   }
}
