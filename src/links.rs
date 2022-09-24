
/* cria link simbólico tanto para a versão em
 * debug, quanto para o binário final. */
use std::os::unix::fs::symlink;
use std::env::current_exe;
use std::path::PathBuf;

// complementa link ao executável.
fn computa_caminho(caminho_str:&str) -> PathBuf {
   // à partir do caminho do executável ...
   match current_exe() {
      Ok(mut base) => {
         // remove executável do caminho.
         base.pop(); 
         // sai do subdiretório 'release'.
         base.pop(); 
         // sai do subdiretório 'target'.
         base.pop();
         // complementa com o caminho passado.
         base.push(caminho_str);
         return base;
      } Err(_) =>
         { panic!("não foi possível obter o caminho do executável!"); }
   }
}

pub fn linka_executaveis(nome: &str) {
   // caminho aos executáveis.
   let caminho_str = "target/release/limpa_downloads";
   let executavel = computa_caminho(caminho_str);
   let executavel_debug: PathBuf;
   let caminho_str = "target/debug/limpa_downloads";
   executavel_debug = computa_caminho(caminho_str);

   // seus links simbólicos:
   let ld_link = computa_caminho(nome);
   let mut nome_debug = nome.to_string();
   nome_debug.push_str("_debug");
   let ld_debug_link = computa_caminho(nome_debug.as_str());

   if ld_link.as_path().exists() && 
   ld_link.as_path().is_symlink() {
      if executavel.as_path().exists() 
         { println!("binário do executável existe."); }
   } else {
      print!("criando '{}' ... ", nome);
      match symlink(executavel.as_path(), ld_link.as_path()) {
         Ok(_) => {
            println!("com sucesso.");
         } Err(_) => 
            { println!("executável não existe!"); }
      };
   }

   if ld_debug_link.as_path().exists() && 
   ld_link.as_path().is_symlink() { 
      if executavel_debug.exists() 
         { println!("binário do executável(DEBUG) existe."); }
   } else {
      print!("criando '{}'(debug) ... ", nome_debug);
      match symlink(executavel_debug.as_path(), ld_debug_link.as_path()) {
         Ok(_) => {
            println!("com sucesso.");
         } Err(_) => 
            { println!("executável não existe!"); }
      };
   }
}
