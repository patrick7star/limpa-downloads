/* Testa se o programa permite o lançamento de multiplas instância ao mesmo
 * tempo. Isso deve ser uma exigência, já que, o programa altera dados nos
 * diretórios definidos, e vários programas ao mesmo tempo pode interferir
 * na instância do outro, ao mesmo tempo. Isso pode causar interrupção de 
 * um programa, ou até de ambos. */

use std::process::{Command, Stdio};
use std::time::{Duration};
use std::thread::{sleep as Sleep};


#[test]
fn lancando_duas_instancias() {
   const CAMINHO_LINK: &str = "./limpa-downloads-debug";
   let mut prog_a = Command::new(CAMINHO_LINK);
   let mut prog_b = Command::new(CAMINHO_LINK);

   // Não é definido se o output será sempre "desabilitado".
   prog_a.stdout(Stdio::null());
   prog_b.stdout(Stdio::null());

   let mut processo_a = match prog_a.spawn() {
      Ok(subprocesso) => subprocesso,
      Err(erro) =>
         { panic!("[ERROR] {erro:}"); }
   };

   /* Um tempo menor, provavelmente não dará certo, pois a chamada dos 
    * determinados recursos às vezes ficam numa fila de espera. Então,
    * enquanto um processo não é executado, mesmo dado a ordem de ser o 
    * primeiro, o outro, que será bem mais simples, pode entrar na frente,
    * causando um resultado inesperado. */
   Sleep(Duration::from_millis(300));

   let mut processo_b = match prog_b.spawn() {
      Ok(subprocesso) => subprocesso,
      Err(erro) =>
         { panic!("[ERROR] {erro:}"); }
   };

   let status_a = processo_a.wait().unwrap();
   let status_b = processo_b.wait().unwrap();

   println!(
      "Status(A): {}\nStatus(B): {}", 
      status_a.code().unwrap(),
      status_b.code().unwrap()
   );
   assert!(status_a.success() && !status_b.success());
}
