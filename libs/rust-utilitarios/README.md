# Utilitarios
Quase todas mesmas funções dos `utilitarios_em_python` porém para o Rust🦀. 

módulos:
  - `por_extenso`: pega qualquer número inteiro positivo de 64-bits, e escreve ele em texto.
  - `arvore`: dado um caminho, ele desenha seus subdiretórios e arquivos de forma esquematizada, sendo que se pode escolher se quer apenas sub-diretórios ou arquivos também.
  - `romanos`: escreve números romanos dado um inteiro positivo de 16-bits, dentro do limite é claro. Também transforma uma string representando um romano num inteiro positivo de 16-bits, ou seja, a forma decimal posicional.
  - `legivel`: pega valores de tamanhos(em bytes)/ou tempos(em segundos) grandes e retorna uma string com tais reduziadas a notações científicas, com unidades mais palátaveis.
  - `barra_de_progresso`: tem um monte de "barras de progressos" para variadas situações.
  - `aleatorio`: jeito fácil de gerar números inteiros de 8-bits(não é escalável). Também pode selecionar randomicamente valores de arrays pequenas.
  - `impressao`: listar várias coisas na tela, máximizando o uso da mesma.
  - `tela`: escrever strings, ou realizar vários desenhos na tela, em qualquer posição dada. 
  - `terminal_dimensao`: implementação simples para obter as dimensões do terminal usado.
  - `tabela_visualizacao`: jeito melhor de representar dados de uma tabela no terminal.

### Considerações:
O módulo `tela` só aceita no máximo uma tela de 256(caractéres) de largura/altura, pois é medida em caractéres tal dimensão. No futuro será extendido para uma medida na casa das dezena de milhar.

## Exemplos:
primeiro do módulo `tela`
```rust
let mut t = Tela::cria(false, true);
let p1 = Ponto{linha:5, coluna:20};
let p2 = Ponto{linha:3, coluna:40};

let tipo = TipoD::Principal;
let outro_tipo = TipoD::Secundaria;
let direcao = Direcao::Diagonal(tipo);
let outra_direcao = super::Direcao::Diagonal(outro_tipo);

t.risca(p1, 13, '@', direcao);
t.risca(p2, 14, 'X', outra_direcao);
```

do módulo `aleatorio`
```rust
let x = sortear::u8(0..=15);
let y = sortear::i8(-25..=4);
let z = sortear::i32(1_000..=34_329);

assert_eq!(x, 7);
assert_eq!(y, -3);
assert_eq!(z, 25_312);

// 50% chance de passar!
assert!(sortear::bool());
```
