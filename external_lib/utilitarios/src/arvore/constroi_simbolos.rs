/*
Imprime texto e retorna strings, sobre qualquer
texto passado, porém na forma gráfica de desenho.
*/


pub fn matriciar_string(string:String) -> Vec<Vec<char>> {
    /* pega uma string que tem quebra de linha 
     * num texto, representado aqui por uma matriz
     * onde cada linha do texto equivale a uma 
     * linha da matriz. */
    // cria uma matriz.
    let mut matriz:Vec<Vec<char>> = Vec::new();

    // iterador que dá várias strings, brotadas
    // da quebra-de-linha.
    for linha in string.lines() {
        // vetor auxiliar que representa linha da 
        // matriz.
        let mut row:Vec<char> = Vec::new();
        // põe cada caractére da string-linha.
        for c in linha.chars() { row.push(c); }
        // põe o vetor na array-de-vetores.
        matriz.push(row);
    }
    // antes do retorno equaliza colunas.
    equaliza_matriz(&mut matriz);
    return matriz;
}


fn equaliza_matriz(matriz:&mut Vec<Vec<char>>) {
    /* obtem a referência de uma matriz, então preenche
     * com espaços em branco até atinger a linha da matriz
     * com maior números de colunas. */
    let qtd_linhas = (*matriz).len();

    // acha linha com mais colunas e, esta quantia.
    let mut max_cols = matriz[0].len();
    for indice in 1..qtd_linhas {
        // contabiliza a quantia de colunas da linha atual.
        let qtd_cols = matriz[indice].len();
        if  max_cols < qtd_cols { max_cols = qtd_cols; }
    }

    /* equaliza todas as "linhas" da matriz baseado
     * na maior, ou seja, a com mais colunas. Serão
     * preenchidas com espaço em branco.
     */
    for i in 0..qtd_linhas {
        while matriz[i].len() < max_cols {
            matriz[i].push(' ');
        }
    }
}


pub fn matriz_para_string(matriz:&Vec<Vec<char>>) -> String {
   /* transforma matriz retângular numa string. */
   // objeto string para concatenar linhas.
   let mut grade_str:String = String::from("");

   // percorrendo linhas.
   for linha in matriz {
      // pecorrendo colunas ou células da linha.
      for celula in linha {
         // adicionando cada símbolo.
         grade_str.push(*celula);
      }
      // adicionando quebra de linha.
      grade_str.push('\n')
   }
   return grade_str;
}
