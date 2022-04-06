
/**!
 Escreve um número inteiro positivo dado
 por extenso. Exemplos:
    5852 ==> "cinco mil oitocentos e cinquenta
             e dois"
    94   ==> "noventa e quatro"
    86914 ==> "ointenta e seis mil novecentos 
              e quartoze"
    320 823 119 539 ==> "trezentos e vinte bilhões 
      oitocentos e vinte e três milhões cento e 
      dezenote mil quinhentos e trinta e nove"

 Alguns casos para ficar de olho:
   80000 ==> "oitenta mil"
   5003 ==> "cinco mil e três"
   304015044 ==> "trezentos e quatro milhões quinze mil
      e quarenta e quatro"
   100 ==> "cem"
   101 ==> "cento e um"
   138 ==> "centro e trinto e oito"
   132921 ==> "um milhão e trezentos e vinte e nove mil
      novecentos e vinte e um"
*/

use std::str::FromStr;


// cuida primeiramente dos primeiros 1000 casos.
fn zero_a_mil(algarismos:&[u8]) -> String {
   if algarismos.len() != 3
      { panic!("só é necessário três elementos!"); }
   // apelido para facilitar codificação:
   let algs = algarismos;
   // string concatenadora.
   let mut escrita = String::new();

   // casa das dezenas na classe.
   let dezenas = match algs[1] {
      0 => "",
      1 => "dez",
      2 => "vinte",
      3 => "trinta",
      4 => "quarenta",
      5 => "cinquenta",
      6 => "sessenta",
      7 => "setenta",
      8 => "oitenta",
      9 => "noventa",
      _ => { panic!("inválido como algarismos!"); }
   };
   // unidades da classe.
   let unidades = match algs[2] {
      0 => "",
      1 => "um",
      2 => "dois",
      3 => "três",
      4 => "quatro",
      5 => "cinco",
      6 => "seis",
      7 => "sete",
      8 => "oito",
      9 => "nove",
      _ => { panic!("inválido como algarismos!"); }
   };
   // casa das centenas na classe.
   let centenas = match algs[0] {
      0 => "",
      1 => "cento",
      2 => "duzentos",
      3 => "trezentos",
      4 => "quatrocentos",
      5 => "quinhentos",
      6 => "seiscentos",
      7 => "setecentos",
      8 => "oitocentos",
      9 => "novecentos",
      _ => { panic!("inválido como algarismos!"); }
   };

   // abrindo em casos ...
   if algs[0] == 0 && algs[1] == 0 && algs[2] == 0
      { escrita = String::from("zero"); }
   else if algs[0] != 0 && algs[1] == 0 && algs[2] == 0 { 
      /* tipo de casos trabalhados neste bloco:
       * 100, 200, 300, 400,... ,800, 900 */
      if algs[0] == 1
         { escrita = String::from("cem"); }
      else 
         { escrita = centenas.to_string(); }
   }
   else if algs[0] == 0 && algs[1] != 0 && algs[2] == 0 { 
      /* tipo de casos trabalhados neste bloco:
       * 10, 20, 30, 40,... ,80, 90 */
      escrita = dezenas.to_string(); 
   }
   else if algs[0] == 0 && algs[1] == 0 && algs[2] != 0 { 
      /* tipo de casos trabalhados neste bloco:
       * 1, 2, 3, 4,... 8, 9. */
      escrita.push_str(unidades);
   }
   else if algs[0] == 0 && algs[1] != 0 && algs[2] != 0 { 
      /* tipo de casos trabalhados neste bloco:
       * 85, 39, 24, 15 e etc. */
      escrita.push_str(dezenas);
      escrita.push_str(" e ");
      escrita.push_str(unidades);
   }
   else if algs[0] != 0 && algs[1] == 0 && algs[2] != 0 { 
      /* tipo de casos trabalhados neste bloco:
       * 805, 309, 204, 105 e etc. */
      escrita.push_str(centenas);
      escrita.push_str(" e ");
      escrita.push_str(unidades);
   }
   else if algs[0] != 0 && algs[1] != 0 && algs[2] == 0 { 
      /* tipo de casos trabalhados neste bloco:
       * 850, 390, 240, 150 e etc. */
      escrita.push_str(centenas);
      escrita.push_str(" e ");
      escrita.push_str(dezenas);
   }
   else {
      /* todos os demais, onde não há algarismos nulos
       * serão tratados aqui. Por exemplo: 312, 582,
       * 958, 642, 231, 253 e etc...*/
      escrita.push_str(centenas);
      escrita.push_str(" e ");
      escrita.push_str(dezenas);
      escrita.push_str(" e ");
      escrita.push_str(unidades);
   }

   // retorna número por extenso.
   return escrita;
}

// peguando casos especiais e reescrevendo string.
fn consertando_casa_dos_dez(numero_escrito:String) -> String {
   // apelindo para codificação.
   let mut ne = numero_escrito;

   // onze.
   if ne.contains("dez e um") 
      { ne = ne.replace("dez e um", "onze"); }
   // doze.
   if ne.contains("dez e dois") 
      { ne = ne.replace("dez e dois", "doze"); }
   // treze.
   if ne.contains("dez e três") 
      { ne = ne.replace("dez e três", "treze"); }
   // quartoze.
   if ne.contains("dez e quatro") 
      { ne = ne.replace("dez e quatro", "quartoze"); }
   // quinze.
   if ne.contains("dez e cinco") 
      { ne = ne.replace("dez e cinco", "quinze"); }
   // dezesseis.
   if ne.contains("dez e seis") 
      { ne = ne.replace("dez e seis", "dezesseis"); }
   // dezesete.
   if ne.contains("dez e sete")
      { ne = ne.replace("dez e sete", "dezesete"); }
   // dezoito.
   if ne.contains("dez e oito")
      { ne = ne.replace("dez e oito", "dezoito"); }
   // dezenove.
   if ne.contains("dez e nove")
      { ne = ne.replace("dez e nove", "dezenove"); }

   // re-retornando a string passada, talvez consertada.
   return ne;
}

/* decompõe um número em algarismos, onde 
 * a parte mais a esquerda têm uma potência
 * maior que o mais a esquerda, mesmo como
 * é escrito a mão. */
fn decompoe(numero:u64) -> Vec<u8> {
   // pilha contendo algarismos.
   let mut algarismos:Vec<u8> = Vec::new();

   // empilhando algarismos ...
   for alg in numero.to_string().chars() { 
      let s = alg.to_string();
      let inteiro:u8 = u8::from_str(s.as_str()).unwrap();
      algarismos.push(inteiro); 
   }

   // faz sempre o número um múltiplo de três.
   let qtd = algarismos.len();
   if qtd % 3 == 1
      { algarismos.insert(0,0); algarismos.insert(0,0); }
   else if qtd % 3 == 2
      { algarismos.insert(0,0); }

   return algarismos;
}

pub fn escreve_por_extenso(numero:u64) -> Result<String, &'static str> {
   /* no caso de um valor de 0 à 1000, um função cuida
    * perfeitamente disso, precisando apenas que aplique
    * uma correção, como os demais casos. */
   if numero < 1000 { 
      let numero_str = zero_a_mil(&decompoe(numero));
      let pos_conserto = consertando_casa_dos_dez(numero_str);
      return Ok(pos_conserto);
   } else { 
      let mut escrita:String = String::new();
      let algarismos = decompoe(numero);
      let qtd = algarismos.len();

      if qtd == 6 {
         let slice:&[u8] = &algarismos[0..3];
         let forma_numero = zero_a_mil(slice);
         if forma_numero != "zero" {
            escrita += forma_numero.as_str();
            escrita += " mil ";
         }

         let slice:&[u8] = &algarismos[3..6];
         let forma_numero = zero_a_mil(slice);
         if forma_numero != "zero" 
            { escrita += forma_numero.as_str(); }
      }
      else if qtd == 9 {
         let slice:&[u8] = &algarismos[0..3];
         let forma_numero = zero_a_mil(slice);
         let no_plural:bool = {
            forma_numero != "zero" && 
            forma_numero != "um"
         };
         let no_singular:bool = {
            forma_numero != "zero" && 
            forma_numero == "um"
         };
         if no_plural {
            escrita += forma_numero.as_str();
            escrita += " milhões ";
         } else if no_singular {
            escrita += forma_numero.as_str();
            escrita += " milhão ";
         }

         let slice:&[u8] = &algarismos[3..6];
         let forma_numero = zero_a_mil(slice);
         if forma_numero != "zero" {
            escrita += forma_numero.as_str();
            escrita += " mil ";
         }

         let slice:&[u8] = &algarismos[6..9];
         let forma_numero = zero_a_mil(slice);
         if forma_numero != "zero" 
            { escrita += forma_numero.as_str(); }
      }
      else if qtd == 12 {
         let slice:&[u8] = &algarismos[0..3];
         let forma_numero = zero_a_mil(slice);
         let no_plural:bool = {
            forma_numero != "zero" && 
            forma_numero != "um"
         };
         let no_singular:bool = {
            forma_numero != "zero" && 
            forma_numero == "um"
         };
         if no_plural {
            escrita += forma_numero.as_str();
            escrita += " bilhões ";
         }
         else if no_singular {
            escrita += forma_numero.as_str();
            escrita += " bilhão ";
         }

         let slice:&[u8] = &algarismos[3..6];
         let forma_numero = zero_a_mil(slice);
         let no_plural:bool = {
            forma_numero != "zero" && 
            forma_numero != "um"
         };
         let no_singular:bool = {
            forma_numero != "zero" && 
            forma_numero == "um"
         };
         if no_plural {
            escrita += forma_numero.as_str();
            escrita += " milhões ";
         }
         else if no_singular {
            escrita += forma_numero.as_str();
            escrita += " milhão ";
         }

         let slice:&[u8] = &algarismos[6..9];
         let forma_numero = zero_a_mil(slice);
         if forma_numero != "zero" {
            escrita += forma_numero.as_str();
            escrita += " mil ";
         }

         let slice:&[u8] = &algarismos[9..12];
         let forma_numero = zero_a_mil(slice);
         if forma_numero != "zero" 
            { escrita += forma_numero.as_str(); }
      }
      else if qtd == 15 {
         let slice:&[u8] = &algarismos[0..3];
         let forma_numero = zero_a_mil(slice);
         let no_plural:bool = {
            forma_numero != "zero" && 
            forma_numero != "um"
         };
         let no_singular:bool = {
            forma_numero != "zero" && 
            forma_numero == "um"
         };
         if no_plural {
            escrita += forma_numero.as_str();
            escrita += " tilhões ";
         } else if no_singular {
            escrita += forma_numero.as_str();
            escrita += " tilhão ";
         }

         let slice:&[u8] = &algarismos[3..6];
         let forma_numero = zero_a_mil(slice);
         let no_plural:bool = {
            forma_numero != "zero" && 
            forma_numero != "um"
         };
         let no_singular:bool = {
            forma_numero != "zero" && 
            forma_numero == "um"
         };
         if no_plural {
            escrita += forma_numero.as_str();
            escrita += " bilhões ";
         }
         else if no_singular {
            escrita += forma_numero.as_str();
            escrita += " bilhão ";
         }

         let slice:&[u8] = &algarismos[6..9];
         let forma_numero = zero_a_mil(slice);
         let no_plural:bool = {
            forma_numero != "zero" && 
            forma_numero != "um"
         };
         let no_singular:bool = {
            forma_numero != "zero" && 
            forma_numero == "um"
         };
         if no_plural {
            escrita += forma_numero.as_str();
            escrita += " milhões ";
         } else if no_singular {
            escrita += forma_numero.as_str();
            escrita += " milhão ";
         }

         let slice:&[u8] = &algarismos[9..12];
         let forma_numero = zero_a_mil(slice);
         if forma_numero != "zero" {
            escrita += forma_numero.as_str();
            escrita += " mil ";
         }

         let slice:&[u8] = &algarismos[12..15];
         let forma_numero = zero_a_mil(slice);
         if forma_numero != "zero" 
            { escrita += forma_numero.as_str(); }
      } else { 
         // se for uma ordem ainda não trabalhada ...
         let mensagem = "quatrilhão, quintilhão e etc; não implementada!";
         return Err(mensagem);
      }

      /* concertando dezenas que por meio automático foram
       * traduzidas como por exemplo:
       *    'dez e cinco' ao invés de 'quize'
       *    'trezentos e dez e um' ao invés de 'trezentos
       *    e onze' */
      Ok(consertando_casa_dos_dez(escrita))
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn testa_decompoe() {
      assert_eq!(vec![1, 2, 3], decompoe(123));
      assert_eq!(vec![9, 3, 1], decompoe(931));
      assert_ne!(vec![9, 3, 2], decompoe(931));
      assert_eq!(vec![2,3, 0, 9, 5, 2], decompoe(230_952));
      /* testando números com quantia de algarismos
       * não múltiplas de três. */
      assert_ne!(vec![1,2,3,4], decompoe(1234));
      assert_eq!(vec![0,0,1,2,3,4], decompoe(1234));
      assert_eq!(vec![0,0,2], decompoe(2));
      assert_ne!(vec![2], decompoe(2));
      assert_eq!(vec![0,0,5,2,3,1,1,1,9], decompoe(5_231_119));
      assert_ne!(vec![5,2,3,1,1,1,9], decompoe(5_231_119));
   }

   #[test]
   fn testa_zero_a_mil() {
      // testando com três algarismos:
      assert_eq!("oitocentos e cinquenta e dois", zero_a_mil(&[8,5,2]));
      assert_eq!(zero_a_mil(&[7,8,9]), "setecentos e oitenta e nove");
      assert_eq!(zero_a_mil(&[3;3]), "trezentos e trinta e três");
      assert_eq!(zero_a_mil(&[6,9,1]), "seiscentos e noventa e um");
      // com dezenas e unidades zeradas.
      assert_eq!(zero_a_mil(&[1,0,0]), "cem");
      assert_eq!(zero_a_mil(&[5,0,0]), "quinhentos");
      assert_eq!(zero_a_mil(&[9,0,0]), "novecentos");
      // com apenas dezenas zeradas.
      assert_eq!(zero_a_mil(&[1,0,9]), "cento e nove");
      assert_eq!(zero_a_mil(&[1,0,1]), "cento e um");

      // testando com apenas dois algarismos:
      assert_eq!(zero_a_mil(&[0,2,3]), "vinte e três");
      assert_eq!(zero_a_mil(&[0,7,2]), "setenta e dois");
      assert_eq!(zero_a_mil(&[0,4,6]), "quarenta e seis");
      assert_eq!(zero_a_mil(&[0,9,1]), "noventa e um");
      // testando com as unidades zeradas:
      assert_eq!(zero_a_mil(&[0,5,0]), "cinquenta");
      assert_eq!(zero_a_mil(&[0,1,0]), "dez");
      assert_eq!(zero_a_mil(&[0,3,0]), "trinta");
      assert_eq!(zero_a_mil(&[0,8,0]), "oitenta");

      // testando com apenas um algarismos:
      assert_eq!(zero_a_mil(&[0,0,8]), "oito");
      assert_eq!(zero_a_mil(&[0,0,7]), "sete");
      assert_eq!(zero_a_mil(&[0,0,3]), "três");
      assert_eq!(zero_a_mil(&[0,0,4]), "quatro");
      assert_eq!(zero_a_mil(&[0,0,9]), "nove");
      assert_eq!(zero_a_mil(&[0,0,6]), "seis");
      assert_eq!(zero_a_mil(&[0,0,0]), "zero");

      // possíveis erros:
      assert_ne!(zero_a_mil(&[3,1,9]), "trezentos e dezenove"); 
      assert_ne!(zero_a_mil(&[8,1,1]), "oitocentos e onze"); 
      assert_ne!(zero_a_mil(&[1,1,5]), "cento e quinze"); 
      // último certo para balançar a tedência.
      assert_eq!(zero_a_mil(&[6,1,2]), "seiscentos e dez e dois"); 
   }

   #[test]
   fn testa_consertando_casa_dos_dez() {
      let s = "oitocentos e dez e quatro";
      let r = "oitocentos e quartoze";
      let conserto = consertando_casa_dos_dez(s.to_string());
      assert_eq!(r, conserto.as_str());

      let s = "dez e oito";
      let certo = "dezoito";
      let conserto = consertando_casa_dos_dez(s.to_string());
      assert_eq!(certo, conserto.as_str());

      let s = "quinhentos e dez e dois";
      let certo = "quinhentos e doze";
      let conserto = consertando_casa_dos_dez(s.to_string());
      assert_eq!(certo, conserto.as_str());

      let s = "cem e dez e cinco";
      let certo = "cem e quinze";
      let conserto = consertando_casa_dos_dez(s.to_string());
      assert_eq!(certo, conserto.as_str());

      let s = "novecentos e dez e sete";
      let certo = "novecentos e dezesete";
      let conserto = consertando_casa_dos_dez(s.to_string());
      assert_eq!(certo, conserto.as_str());
   }

   #[test]
   fn testa_escreve_por_extenso() {
      assert_eq!("cinco", escreve_por_extenso(5).unwrap());
      assert_eq!("onze", escreve_por_extenso(11).unwrap());
      assert_eq!("cinquenta e dois", escreve_por_extenso(52).unwrap());
      assert_eq!("quatrocentos e vinte e oito", escreve_por_extenso(428).unwrap());
      assert_eq!(
         "nove mil seiscentos e dezenove", 
         escreve_por_extenso(9_619).unwrap()
      );
      assert_eq!(
         "noventa e um mil duzentos e quarenta e três", 
         escreve_por_extenso(91_243).unwrap()
      );
      assert_eq!(
         "quatrocentos e setenta e oito mil cento e onze",
         escreve_por_extenso(478_111).unwrap()
      );
      assert_eq!(
      "sete milhões quinhentos e vinte e sete mil setecentos e oitenta e quatro",
      escreve_por_extenso(7_527_784).unwrap()
      );
      assert_eq!(
      "trinta e sete milhões cento e cinco mil duzentos e quartoze",
      escreve_por_extenso(37_105_214).unwrap()
      );
      assert_eq!(
      "oitocentos e oitenta e um milhões novecentos e doze mil dezenove",
      escreve_por_extenso(881_912_019).unwrap()
      );
      assert_eq!(
      "um bilhão um milhão novecentos e noventa e seis mil setecentos e quarenta",
      escreve_por_extenso(1_001_996740).unwrap()
      );
      assert_eq!(
      "vinte e nove bilhões seiscentos milhões quinhentos mil duzentos",
      escreve_por_extenso(29_600_500_200).unwrap()
      );
      assert_eq!(
      "cento e vinte e nove bilhões seiscentos milhões cento e treze mil duzentos",
      escreve_por_extenso(129_600_113_200).unwrap()
      );
   }
}
