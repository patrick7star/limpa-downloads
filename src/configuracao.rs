

/*
 *   O mesmo de onde este foi copiado, lê um
 * arquivo que têm algumas configurações e
 * especificações, que não são decididas mais
 * por codificação, mas sim, em tempo de 
 * execução(talvez isso possa desacelerar 
 * um pouco o programa[dependendo de quantas
 * destas estamos falando]). Como foi feita
 * específicamente para a aplicação que foi 
 * copiada, precisamos dá alguns retoques,
 * tanto para já generaliza-lô -- já que 
 * ser copiada para cá, implica isso -- como
 * também para esta aplicação específica.
 *
 *   Vamos explicar o que é exatamente isso:
 * É algo parecido com o JSON, XML e outros
 * serializadores de dados, não tanto robusto
 * quanto(não chega nem perto), mas algo suficiente
 * para o que a aplicação precisa. Por que não 
 * usar os já robustos? Bem, eu queria fazer
 * algo para mim, com a minha sintaxe. Consegui
 * e funcionou muito bem, agora estou expandido.
 * No futuro irei migrar definitamente para
 * XML ou JSON, ou os dois, claro que não no
 * mesmo projeto; porém, tal dia não é hoje.
 */

// biblioteca padrão:
use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;
use std::fs::read_to_string;
use std::path::{PathBuf, Path};

pub const ARQUIVO_DE:&str = concat!( 
   env!("RUST_CODES"),
   "/limpa-downloads",
   "/configuração.conf"
);


type Cabecalho = HashMap<String, Vec<String>>;
/* separa um cabeçalho, e as linhas ligadas à ele,
 * por meio de um dicionário. */
fn todas_configuracoes(conteudo: String) -> Cabecalho {
   let mut cabecalho_detectado = false;
   /* dicionário que com 'chave de cabeçalho',
    * coleta linhas baixo dele. */
   let mut mapa = Cabecalho::new();
   let mut chave: String = "".to_string();

   for l in conteudo.lines().filter(|s| !s.contains(&"#")) {
      // se char um novo cabeçalho.
      if l.contains(&"[") && l.contains(&"]") 
         { cabecalho_detectado = true; } 
      // acessa atual chave.
      else { 
         /* reduzindo impressões desnecessárias
          * por não serem, obviamente, chaves. */
         if chave == ""
            { continue; }
         match mapa.get_mut(&chave) {
            Some(atual) => { 
               let linha = {
                  l.trim_matches('\r')
                  .trim_matches('\t')
                  .trim()
                  .to_string()
               };
               if linha != ""
                  { atual.push(linha); }
            } None => { 
               println!("chave '{}' não existe, provavelmente espaço vázio.", chave); }
         };
      }

      /* extrai conteúdo do cabeçalho, e cria
       * uma chave no dicionário com ela. Isso
       * aciona uma vez por cabeçalho. */
      if cabecalho_detectado {
         let i = l.find('[').unwrap() + 1;
         let f = l.find(']').unwrap();
         let cabecalho = l.get(i..f).unwrap().to_string();
         // mudando nova chave.
         chave = cabecalho.clone();
         mapa.insert(cabecalho, vec![]);
         // desativa até um próximo cabeçalho.
         cabecalho_detectado = false;
      }
   }

   return mapa;
}

use std::collections::VecDeque;
type Fila = VecDeque<char>;
/* separa a parte numérica das partes 
 * textuais. Exige a total ausência de
 * espaços em brancos para funcionar 
 * adequadamente. */
fn decomposicao_em_tokens(s: String) -> Vec<String> {
   let mut tokens = Vec::<String>::new();
   let numerico = |ch: char| ch.is_ascii_digit() || ch == '.';
   let alfabetico = |ch: char| ch.is_alphabetic();
   let (mut f_a, mut f_n) = (
      Fila::with_capacity(20),
      Fila::with_capacity(20)
   );
   for char in s.chars() {
      // cuida da sequência seguida de números.
      if numerico(char) {
         f_n.push_back(char);
      } else {
         /* injeta a sequência alfabetica. */
         let mut tk = String::new();
         while let Some(e) = f_n.pop_front() 
            { tk.push(e); }
         if tk != ""
            { tokens.push(tk); }
      }
      // se não disparar acima, cuida das letras.
      if alfabetico(char) {
         f_a.push_back(char);
      } else {
         // o oposto, injeta a sequência numérica.
         let mut tk = String::new();
         while let Some(e) = f_a.pop_front() 
            { tk.push(e); }
         if tk != ""
            { tokens.push(tk); }
      }
   }
   /* despejando o resto, já que ele não é 
    * colocado, pois acaba antes da troca de
    * símbolos se disparada, aliás, não haverá 
    * disparo pois acabou. */
   if !f_n.is_empty()
      { tokens.push(f_n.drain(..).collect::<String>()); }
   if !f_a.is_empty()
      { tokens.push(f_a.drain(..).collect::<String>()); }
   /* não aceita nada extraído, dado que a 
    * string é válida. */
   assert!(!tokens.is_empty());
   return tokens;
}

use std::collections::HashSet;
/* validade da decomposição, têm que haver
 * um númerio, então um textual, então novamente
 * numerico, e depois textual, os pares são
 * importantes, a função verifica isso. Por último,
 * verifica se os pesos são válidos, por enquanto
 * só são aceitos até 'meses'. */
fn validade(decomposicao: &Vec<String>) -> bool {
   // a quantia têm que sempre ser par.
   if decomposicao.len() % 2 != 0
      { return false; }

   /* verificam se uma string é "inteiramente"
    * numérica, ou texto(seria o peso). */
   let str_e_numero: fn(&str) -> bool = {
      |string: &str| 
         string.chars()
         .all(|c| c.is_ascii_digit() || c == '.')
   };
   let str_e_peso: fn(&str) -> bool = {
      |string: &str| 
         string.chars()
         .all(|c| c.is_ascii_alphabetic())
   };

   let (mut i, t) = (0, decomposicao.len());
   // pesos têm que ser válidos.
   let pesos: HashSet<&str> = HashSet::from_iter(vec![
      "min", "minuto", "minutos", "h", "horas",
      "dia", "dias", "d", "seg", "segundo",
      "segundos", "hora", "mês", "meses"
      ].drain(..)
   );

   /* testando se a ordem dos pares batem,
    * primeiro a parte numérica, depois o 
    * seu peso(seg, min, dias e etc.). */
   while i < t-1 {
      let (str1, str2) = (
         decomposicao[i].as_str(), 
         decomposicao[i + 1].as_str()
      );
      if !str_e_numero(str1) || !str_e_peso(str2)
         { return false; }
      i += 2;
   }

   /* filtra os pesos válidos, então verifica se
    * tal string está dentro dos 'pesos'
    * dados como válidos na codificação. */
   if !decomposicao.iter()
   .filter(|s| str_e_peso(s.as_str()))
   .all(|s| pesos.contains(s.as_str()))
      { return false; }

   // se chegar até aqui, está tudo ok!
   return true;
}

use std::time::Duration;
/* converte finalmente a array do 'output' numa
 * expiração, que será um 'Duration'. 
 * É exigido que há array tenha valores numéricos
 * e pesos válidos, ou o velho caso: GIGO(garbage in,
 * then garbage out). */
fn transforma_em_validade(array: Vec<String>) -> Duration {
   /* mapeando o equivalente de cada 'peso' 
    * em segundos. */
   let pesos: HashMap<&str, f32> = {
      HashMap::from([
         ("mês", 30f32 * 24f32 * 3600.0),
         ("meses", 30.0 * 24.0 * 3600f32),
         ("dias", 24.0 * 3_600.0),
         ("dia", 24.0 * 3_600.0),
         ("minuto", 60.0), ("min", 60.0),
         ("minutos", 60.0),
         ("seg", 1.0), ("segundos", 1.0),
         ("segundo", 1.0),
         ("h", 3600.0), ("hora", 3600.0),
         ("horas", 3_600.0),
      ])
   };
   // converte os pesos nos respectivos valores.
   let pesos_iterador = {
      array.iter().map(|chave| {
         let key = chave.as_str();
         match pesos.get(key) {
            Some(v) => *v,
            None => 0.0
         }
      }).filter(|v| *v >= 1.0)
   };
   // filtra apenas os valores numéricos.
   let numericos_iterador = {
      array.iter().map(|numero_str| {
         let n = numero_str.as_str();
         match f32::from_str(n) {
            Ok(v) => v,
            Err(_) => 0.0
         }
      }).filter(|v| *v >= 1.0)
   };
   let segundos: f32 = {
      numericos_iterador.zip(pesos_iterador)
      .map(|(n, p)| n * p).sum()
   };
   // enfim, criando-o.
   Duration::from_secs_f32(segundos)
}

/* O primeiro campo é o nome da extensão, já 
 * o segundo, a quantia total de tempo que 
 * é para conometrar até sua exclusão. */
type Validades = Vec<(String, Duration)>;
const TIPO_DE_ARQUIVOS: &str = "Tipos De Arquivos";
/* filtra um específico cabeçalho, que é 
 * o 'tipo de arquivos', para que dê o tempo
 * de validade para exclusão deste tipo de 
 * arquivo no arquivo `RAIZ`. Ele já pega
 * o tempo que tal 'tipo' específica e já 
 * transforma numa 'Validade'.  */
fn tipos_de_arquivos() -> Validades {
   // filtrando todo conteúdo do arquivo com elas.
   let caminho = Path::new("configuração.conf");
   let conteudo = read_to_string(caminho).unwrap();
   // lista de válidades.
   let mut validades = Validades::with_capacity(10);
   let mut mapa = todas_configuracoes(conteudo);
   let chave = TIPO_DE_ARQUIVOS.to_string();
   /* tem que ter a chave, assim com ela não pode
    * está sem nada, caso contrário o programa
    * irá parar. */
   if !mapa.contains_key(&chave) 
      { panic!("cabeçalho '{}' não existe no arquivo", chave); }
   if mapa.get(&chave).unwrap().is_empty()
      { panic!("cabeçalho sem conteúdo"); }
   /* percorre linhas, reparti elas baseado na
    * "igualdade", então pega o devido nome, e 
    * o tempo de expiração deste "tipo". */
   for linha in mapa.get_mut(&chave).unwrap().drain(..) {
      let (nome, resto) = linha.split_once(':').unwrap();
      let resto = resto.trim_matches('\"').to_string();
      let t = decomposicao_em_tokens(resto);
      /* a parte representando o tempo, têm que ser
       * válida para continuar. */
      if validade(&t) {
         let v = transforma_em_validade(t);
         let dado = (nome.to_string(), v);
         validades.push(dado);
      }
   }
   return validades;
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests { 
   use super::*;

   // transforma string para algo aceitáel para função.
   fn ajusta_string(s: &str) -> String {
      s.chars().filter(|c| !c.is_ascii_whitespace())
      .filter(|c| !(c.is_ascii_punctuation() && *c != '.'))
      .collect::<String>()
   }
   #[test]
   fn decomposicaoTesteInicial() {
      // retirando qualquer espaço em branco.
      let amostra = ajusta_string("10hora 52min 18segundos");
      assert_eq!( 
         vec!["10", "hora", "52", "min", 
         "18", "segundos"],
         decomposicao_em_tokens(dbg!(amostra))
      );
   }

   /* converte para argumento válido. */
   fn transforma_vetor(vetor: &Vec<&str>) -> Vec<String> {
      vetor.iter()
      .map(|ss| ss.to_string())
      .collect::<Vec<String>>()
   }
   #[test]
   fn validaSaidaDaDecomposicao() {
      let mut saidas = vec![
         vec!["10", "hora", "52", "min", "18", "segundos"],
         vec!["23", "minutos", "14", "seg"],
         vec!["59", "segundos"],
         vec!["04", "dias", "8", "h"]
      ];
      for vetor in saidas {
         assert!(validade(&transforma_vetor(&vetor)));
      }
   }

   #[test]
   fn decomposicoesCasosGerais() {
      let mut entradas = vec![
         // caso com dois dígitos de números.
         "35minutos 13segundos", "13dias13horas",
         "14dias 10horas 28minutos 56seg",
         "56minutos",
         /* ainda com dois dígitos, porém separados
          * o valor do peso. */
         "21 horas 38min", "18min 33 segundos",
         "12 horas 27 minutos 48 segundos",
         /* casos iguais os acima, porém com 
          * algo a mais, valores numéricos com
          * apenas um dígito. */
         "14h8seg", "1hora 2 minutos", "56 min 2 seg",
      ];
      // saídas seguindo as ordens das entradas acima.
      let mut saidas = vec![
         vec!["35", "minutos", "13", "segundos"],
         vec!["13", "dias", "13", "horas"],
         vec!["14", "dias", "10", "horas", "28", 
         "minutos", "56", "seg"],
         vec!["56", "minutos"],
         vec!["21", "horas", "38", "min"],
         vec!["18", "min", "33", "segundos"],
         vec!["12", "horas", "27", "minutos", 
         "48", "segundos"],
         vec!["14", "h", "8", "seg"],
         vec!["1", "hora", "2", "minutos"],
         vec!["56", "min", "2", "seg"]
      ];
      let decompoem: fn(String) -> Vec<String>;
      decompoem = decomposicao_em_tokens;
      for (e, s) in entradas.drain(..).zip(saidas.drain(..)) { 
         let e = ajusta_string(e);
         println!("'{}' == {:?}", e, s);
         assert_eq!(decompoem(e), s); 
      }
   }

   #[test]
   fn decomposicaoValorDecimal() {
      let ajusta: fn(&str) -> String;
      ajusta = ajusta_string;
      let entrada = ajusta("10.38hora 52.7min 18segundos");
      let saida = decomposicao_em_tokens(entrada.clone());
      println!("'{}' #===> {:?}", entrada, saida);
      assert_eq!( 
         vec!["10.38", "hora", "52.7", "min", 
         "18", "segundos"], saida
      );
      let entrada = ajusta("12.3min 53.1seg");
      let saida = decomposicao_em_tokens(entrada.clone());
      println!("'{}' #===> {:?}", entrada, saida);
      assert_eq!(vec!["12.3", "min", "53.1", "seg"], saida);
      let entrada = ajusta("1.791mês");
      let saida = decomposicao_em_tokens(entrada.clone());
      println!("'{}' #===> {:?}", entrada, saida);
      assert_eq!(vec!["1.791", "mês"], saida);
   }

   #[test]
   fn transformacaoEmValidades() {
      let mut entradas = vec![
         "35minutos 13segundos", "13dias13horas",
         "14dias 10horas 28minutos 56seg",
         "56minutos",
         "21 horas 38min", "18min 33 segundos",
         "12 horas 27 minutos 48 segundos",
         "14h8seg", "1hora 2 minutos", "56 min 2 seg",
      ];
      let mut saidas = vec![
         //vec!["35", "minutos", "13", "segundos"],
         Duration::from_secs_f32(35.0 * 60.0 + 13.0),
         //vec!["13", "dias", "13", "horas"],
         Duration::from_secs_f32(
            13.0 * 24.0 * 3600.0 
            + 13.0*3600.0
         ),
         /*vec!["14", "dias", "10", "horas", "28", 
         "minutos", "56", "seg"], */
         Duration::from_secs_f32(
            14.0 * 24.0 * 3600.0 + 10.0 * 3600.0
            + 28.0 * 60.0 + 56.0
         ),
         //vec!["56", "minutos"],
         Duration::from_secs_f32(56.0 * 60.0),
         //vec!["21", "horas", "38", "min"],
         Duration::from_secs_f32(21.0 * 3600.0 + 38.0 * 60.0),
         //vec!["18", "min", "33", "segundos"],
         Duration::from_secs_f32(18.0 * 60.0 + 33.0),
         /*vec!["12", "horas", "27", "minutos", 
         "48", "segundos"], */
         Duration::from_secs_f32(12.0 * 3600.0 + 27.0 * 60.0 + 48.0),
         // vec!["14", "h", "8", "seg"],
         Duration::from_secs_f32(14.0 * 3600.0 + 08.0),
         // vec!["1", "hora", "2", "minutos"],
         Duration::from_secs_f32(1.0 * 3600.0 + 120.0),
         //vec!["56", "min", "2", "seg"]
         Duration::from_secs_f32(56.0 * 60.0 + 2.0)
      ];
      let decompoem: fn(String) -> Vec<String>;
      let transforma: fn(Vec<String>) -> Duration;
      transforma = transforma_em_validade;
      decompoem = decomposicao_em_tokens;
      for (e, s) in entradas.drain(..).zip(saidas.drain(..)) { 
         let A = decompoem(ajusta_string(e));
         let entrada = transforma(A.clone());
         println!(
            "{:?}'{:#?}' #===> {:#?}", 
            A, entrada.clone(), s.clone()
         );
         assert_eq!(entrada, s);
      }
   }

   use utilitarios::legivel::tempo as Tempo;
   #[test]
   fn tiposDeArquivosFiltroVisual() {
      let saida = tipos_de_arquivos();
      for (nome, expiracao) in saida { 
         println!(
            "{} ==> {:#?}({})", 
            nome, expiracao.clone(), 
            Tempo(expiracao.as_secs(), true)
         ); 
      }
   }
   /* cópia da função principal, espefíca para 
    * testes. Recebe o arquivo principal, que não
    * está inserida na função para fazer uma 
    * variedade de testes. */
   fn tipos_de_arquivos_teste<P>(caminho: &P) -> Validades 
     where P: AsRef<Path> + ?Sized
   {
      // filtrando todo conteúdo do arquivo com elas.
      let conteudo = read_to_string(caminho.as_ref()).unwrap();
      // lista de válidades.
      let mut validades = Validades::with_capacity(10);
      let mut mapa = todas_configuracoes(conteudo);
      let chave = TIPO_DE_ARQUIVOS.to_string();
      /* tem que ter a chave, assim com ela não pode
       * está sem nada, caso contrário o programa
       * irá parar. */
      if !mapa.contains_key(&chave) 
         { panic!("cabeçalho '{}' não existe no arquivo", chave); }
      if mapa.get(&chave).unwrap().is_empty()
         { panic!("cabeçalho sem conteúdo"); }
      /* percorre linhas, reparti elas baseado na
       * "igualdade", então pega o devido nome, e 
       * o tempo de expiração deste "tipo". */
      for linha in mapa.get_mut(&chave).unwrap().drain(..) {
         let (nome, resto) = linha.split_once(':').unwrap();
         let resto = resto.trim_matches('\"').to_string();
         let t = decomposicao_em_tokens(resto);
         /* a parte representando o tempo, têm que ser
          * válida para continuar. */
         if validade(&t) {
            let v = transforma_em_validade(t);
            let dado = (nome.to_string(), v);
            validades.push(dado);
         }
      }
      return validades;
   }
   #[test]
   #[should_panic="cabeçalho sem conteúdo"]
   fn tiposDeArquivosSemConteudo() {
      let caminho_str = "tests/configuração_sem_conteúdo.conf";
      tipos_de_arquivos_teste(caminho_str); 
   }
   #[test]
   #[should_panic="cabeçalho 'Tipos De Arquivos' não existe no arquivo"]
   fn tiposDeArquivosSemCabecalho() {
      let caminho_str = "tests/configuração_sem_cabeçalho.conf";
      tipos_de_arquivos_teste(caminho_str); 
   }
}
