/* Inicialmente serve apenas para rodar testes, anexando um novo caminho de 
 * busca de bibliotecas. */

fn main() {
   println!("Adicionando caminho da biblioteca estática ...");
   println!("cargo:rustc-link-search=native=./lib");
}
