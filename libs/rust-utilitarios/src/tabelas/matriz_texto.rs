
/*!
 Fazendo um objeto que cuida do desenho, como
 funções relacionadas a ele diretamente: 
 concatenação, aparação de espaços em brancos
 e etc.
*/


/* capacidade baseado na dimensão máxima
 * do laptop, que foi primeiramente 
 * codificado. Resolução da tela dele é
 * 1366x768 pixels. */
const MAX_LARGURA: usize = 250;
const MAX_ALTURA: usize = 1000;
const FUNDO: char = ' ';

// para codificação.
type MultiArray = Vec<Vec<char>>;
// para encurtar os parâmetros.
pub type MT = MatrizTexto;


#[derive(Clone)]
pub struct MatrizTexto {
   matriz: Vec<Vec<char>>,
   altura: u16,
   largura: u16,
}

impl MatrizTexto {
   // método construtor:
   pub fn cria(altura: u16, largura: u16) -> Self {
      let mut matriz: MultiArray;
      matriz = Vec::with_capacity(MAX_ALTURA);
      
      for _ in 0..altura {
         let mut linha: Vec<char>;
         linha = Vec::with_capacity(MAX_LARGURA);
         for _ in 0..largura
            { linha.push(FUNDO); }
         matriz.push(linha);
      }
      return MatrizTexto { matriz, altura, largura }
   }

   // muda célula na matriz.
   pub fn set(&mut self, y: u16, x: u16, ch: char) 
      { self.matriz[y as usize][x as usize] = ch; }

   // obtém célula na matriz.
   pub fn get(&self, y: u16, x: u16) -> char 
      { self.matriz[y as usize][x as usize] }

   /* tupla com dimensão da matriz: altura e 
    * largura respectivamente. */
   pub fn dimensao(&self) -> (u16, u16) 
      { (self.altura, self.largura) }
}

// outros métodos mais excentricos.
impl MatrizTexto {
   // apara colunas em brancos à direita.
   #[allow(dead_code)]
   pub fn trim_right(&mut self) {
      for line in self.matriz.iter_mut() 
         { line.pop(); }
   }

   // apara colunas em brancos à esquerda.
   #[allow(dead_code)]
   pub fn trim_left(&mut self) {
      for line in self.matriz.iter_mut() 
         { line.remove(0); }
   }

   // acrescenta 'm' linhas na matriz.
   fn aumenta_altura(&mut self, h: u16) {
      // registro aumento vertical da matriz.
      self.altura += h;

      // forma uma linha com devida largura.
      let mut linha_em_branco: Vec<char>;
      linha_em_branco = Vec::with_capacity(MAX_LARGURA);
      for _ in 0..self.largura 
         { linha_em_branco.push(' '); }
      // adiciona linha-em-branco no topo.
      for _ in 0..h
         { self.matriz.insert(0, linha_em_branco.clone()); }
   }
   
   // acrescenta 'n' colunas.
   fn aumenta_largura(&mut self, l: u16) {
      // registro aumento vertical da matriz.
      self.largura += l;
      for linha in self.matriz.iter_mut() {
         for _ in 0..l 
            { linha.push(FUNDO); }
      }
   }

   // equalisa a menor matrix-texto com a maior.
   fn equaliza_matrizes(mt1:&mut MT, mt2:&mut MT) {
      // altura de ambos.
      let (h1, _) = mt1.dimensao();
      let (h2, _) = mt2.dimensao();
      
      if h1 < h2
         { mt1.aumenta_altura(h2-h1); }
      else if h1 > h2
         { mt2.aumenta_altura(h1-h2); }
   }
}

/* operações das matriz nela própria, ou
 * geradora, que significa que tal operação
 * resulta numa matriz-texto nova. */
impl MatrizTexto {
   /* concatena matriz-texto passado com a própria
    * instância. O argumento(que é uma matriz-texto)
    * será "consumida" dentro do método. */
   #[allow(dead_code)]
   pub fn concatena(&mut self, mut matriz: MT) {
      let (_, l) = matriz.dimensao();
      let largura = self.largura;
      /* coloca algumas das matrizes à nível 
       * da outra. */
      MT::equaliza_matrizes(self, &mut matriz); 
      // redimensiona matriz em "mais 'l'".
      self.aumenta_largura(l);
      for y in 0..self.altura {
         for x in 0..l { 
            let value = matriz.get(y, x);
            self.set(y, largura + x, value);
         }
      }
   }


   /* obtem a referência do objeto e, imprime
    * ele via saída padrão. */
   #[allow(dead_code)]
   pub fn imprime(&self) {
      for row in self.matriz.iter() {
         for cell in row 
            { print!("{}", cell); }
         print!("\n");
      }
   }

}

// apenas métodos estáticos do objeto.
impl MatrizTexto {
   /* transforma uma string -- é necesário que
    * ela seja múltilinha -- numa matriz-texto.
    */
   pub fn to_matriz(string: &str) -> Self {
      let largura: usize = {
         string.lines()
         .map(|linha| linha.chars().count())
         .max().unwrap()
      };
      let altura: u16 = string.lines().count() as u16;
      let mut matriz = MatrizTexto::cria(
         altura as u16, 
         largura as u16
      );

      for (y, linha) in string.lines().enumerate() {
         let mut x: u16 = 0;
         for char in linha.chars() {
            matriz.set(y as u16, x, char);
            x += 1;
         }
      }

      return matriz
   }
   
   /* a mesma função que o método acima, porém com
    * resultante da nova matrix-texto formada. É 
    * um método estático. Os argumentos são referências,
    * e a ordem de concatenação é da esquerda à 
    * direita, mesma da codificação dos parâmetros. */
   #[allow(dead_code)]
   pub fn concatena_matrizes(mt1:&MT, mt2:&MT) -> MT {
      // clona ambos.
      let mut mt1 = mt1.clone();
      let mut mt2 = mt2.clone();
      MT::equaliza_matrizes(&mut mt1, &mut mt2);
      // dimensão após redimensionamento de alguma.
      let ((a, l1), (_, l2)) = (
         mt1.dimensao(), 
         mt2.dimensao()
      );
      // matriz-texto resultante.
      let mut mtr: MatrizTexto;
      mtr = MT::cria(a, l1 + l2);
      for y in 0..a {
         // primeiro parâmetro paran nova matriz-texto.
         for x in 0..l1 
            { mtr.set(y, x, mt1.get(y, x)); }
         // agora, copiando o segundo.
         for x in 0..l2
            { mtr.set(y, x+l1, mt2.get(y, x)); }
      }
      return mtr;
   }
}

// converte a "matriz texto" numa string.
impl ToString for MatrizTexto {
   fn to_string(&self) -> String {
      let l = self.largura as usize;
      let a = self.altura as usize;
      /* quantia de caractéres, mais as
       * quebra-de-linhas. Não está alocado para
       * todo o tamanho, porque alguns caractéres
       * não são ASCII, então custosos
       * redimensionamentos serão feitos, porém
       * algumas unidades de vezes nada
       * estrememanete lento. */
      let mut esboco = String::with_capacity(a * l + l);

      for y in 0..a {
         for x in 0..l 
            { esboco.push(self.matriz[y][x]); }
         // acaba com uma linha, adiciona quebra-de-linha.
         esboco.push('\n');
      }
      return esboco;
   }
}

