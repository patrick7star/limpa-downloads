// Biblioteca do Rust:
use std::process::Command;
use std::time::Duration;
use std::path::{Path};

// Caminho até o programa que é usado abaixo.
const PROGRAMA: &'static str = "/bin/notify-send";

/* função que realmente puxa uma notificação do sistema, respeitado
 * os parâmetros exigidos. */
fn popup_notificacao(nome: &str, icone: &str, duracao: Duration, 
  mensagem: &str) -> std::io::Result<()>
{
   let miliseg = duracao.as_millis();
   let tempo_de_expiracao = format!("--expire-time={miliseg}");
   let imagem = format!("--icon={icone}");
   let nome_do_programa = format!("--app-name={nome}");
   let argumentos: [&str; 4] = [
      /* a ordem dos argumentos não necessariamente importando, só 
       * que o último tem que ser a mensagem. */
      tempo_de_expiracao.as_str(),
      imagem.as_str(),
      nome_do_programa.as_str(),
      mensagem
   ];

   if o_programa_esta_instalado() {
      // executando comando ...
      Command::new("notify-send")
      .args(argumentos.into_iter())
      .spawn().unwrap().wait()?;
   }

   Ok(())
}

/* faz uma notificação da atual transição aplicada ao sistema. */
pub fn informa_n_itens_removidos(total: usize) {
   let mensagem = format!("foram deletados {} itens!", total);
   // Emitindo notificação...
   let resultado = popup_notificacao(
      // nome do programa e seu ícone:
      "Limpa Downloads",
      "computerjanitor",
      // duração em segundos e sua mensagem:
      Duration::from_secs(25),
      mensagem.as_str()
   );

   match resultado {
      Ok(_) =>
         // Emitindo que a mensagem foi enviada.
         { println!("notificação foi \"plotada\" com sucesso."); }
      Err(erro) =>
         { eprintln!("[Errro] {}", erro); }
   }
}

pub fn avisa_de_diretorio_esta_vazio() {
   popup_notificacao(
      "LimpaDownloads",
      "trashcan_empty",
      Duration::from_secs(20),
      "o diretório está completamente vázio(I)"
   ).unwrap();
}

fn o_programa_esta_instalado() -> bool 
   { Path::new(PROGRAMA).exists() }

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;

   #[test]
   fn exemplo_da_notificacao_requerida() {
      informa_n_itens_removidos(15);
   }

   #[test]
   fn interno_que_lanca_notificacao() {
      assert!(
         popup_notificacao(
            "meu_programa", "checkbox", 
            Duration::from_secs(7), 
            "Darling, you are the only exception!"
         ).is_ok()
      );
      // confirmado visualmente o ouptut.
      assert!(true);
   }
}
