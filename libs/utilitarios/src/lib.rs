
/*!
   Todos códigos que não tiverem elaborações bem 
   complexas, ou seus esboços iniciais e simples, 
   porém bem úteis ficarão aqui. Isto é muito melhor
   que ao invés de criar um `crate` para cada um.
   Como disse antes tais funções e estruturas de cada
   módulo executam coisas muitos simples.
*/

// re-exportando módulos.

/// dado um diretório desenha uma árvore em string
/// baseando nos arquivos e diretórios do atual(raíz)
/// e seus subdirs.
pub mod arvore;

/// talvez o mais complexo do pacote, cria uma estrura
/// para manipular de forma maleável a impressão de
/// texto e desenhos simples no terminal.
pub mod tela;

/// converte valores inteiros e fluantes, que representam
/// grandezas importantes em computação, para valores
/// legíveis, tais na formatação de strings.
pub mod legivel;

/// pega lista e arrays de dados e faz uma tabela delas 
/// para simples impressões.
pub mod tabela_visualizacao;

/// simulação de valores aleatórios.
pub mod aleatorio;

/// transforma strings e inteiros que representam valores
/// decimais/ou binários para números romanos, o inverso
/// também, ou seja, romanos para números decimais/inteiros.
pub mod romanos;

pub mod impressao;

/** variádos tipos de barras de progressos, que informam
 de forma dinâmica como a computabilidade de tais 
 dados abordados está indo. */
pub mod barra_de_progresso;

/** obtem a dimensão do terminal que roda tal lib. 
 Portanto agora, podemos excluir a biblioteca externa
 utilizada para fazer tal, substituindo por esta.
*/
pub mod terminal_dimensao;

/// escreve um número dado por extenso.
pub mod por_extenso;
