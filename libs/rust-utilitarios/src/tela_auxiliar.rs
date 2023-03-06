use crate::tela::Ponto;

// renomea tipo estático.
type Str = &'static str;
// máximo de items que podem ser empilhados.
const CAPACIDADE: u8 = 20;

pub struct Pilha<A> {
   /* array que armazena os objetos
    * Para tal, com inicialização, no
    * mínimo em vinte, mais é desnecessário
    */
   array: Vec<A>,
   tamanho: u8,
}

// método construtor.
impl <A>Pilha<A> {
   pub fn nova() -> Self {
      let capacidade: usize;
      capacidade = CAPACIDADE as usize;
      Pilha {
         array: Vec::with_capacity(capacidade),
         tamanho: 0,
      }
   }
}

// operações que a pilha realiza:
impl <A>Pilha<A> {
   // verifica se a pilha está vázia.
   pub fn vazia(&self) -> bool 
      { self.tamanho == 0 }
   
   // empilha um item na pilha se não estiver
   // atigindo sua capacidade máxima.
   pub fn empilha(&mut self, item:A) -> Result<(), Str> {
      if self.tamanho == CAPACIDADE
         { return Err("só é possível empilhar isso aí."); }
      
      self.array.push(item);
      // contabiliza o item adicionado.
      self.tamanho += 1;
      return Ok(());
   }

   // remove um item do topo, se houver algum.
   pub fn desempilha(&mut self) -> Option<A> {
      if self.vazia()
         { return None; }
      //descontabiliza item retirado.
      self.tamanho -= 1;
      return self.array.pop();
   }
}

/* Registra uma "mudança" feita na tela,
 * tendo os últimos `Ponto`s que foram 
 * escrito nela. Então é isso que significa
 * uma mudança ocorrida, um aglomerado de 
 * pontos recentementes "printados" na tela.
 */
 pub struct Mudanca {
   /* pilha contendo pontos que foram, 
    * ultimamente, marcados. Também o símbolo
    * que estava alí.*/
   pub pontilhados_escritos: Vec<(Ponto, char)>,
}

// apelido para melhor legibilidade.
impl Mudanca {
   /* funciona junto com o método abaixo, que permite
    * dimensionar a lista de mudanças a desfazer/ou
    * refazer futuramente. */
   pub fn cria_vazio() -> Self 
      { Self { pontilhados_escritos: Vec::new() } }

   /* às vezes têm que adicionar algum que não 
    * foi inicialmente pensado. */
   pub fn incrementa(&mut self, pixel:(Ponto, char)) 
      { self.pontilhados_escritos.push(pixel); }
}