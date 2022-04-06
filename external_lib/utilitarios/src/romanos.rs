/*! converte um número decimal para romanos. */

use std::collections::HashMap;

// algarismos romanos equivalentes:
const UM:&str = "I";
const CINCO:&str = "V";
const QUATRO:&str = "IV";
const NOVE:&str = "IX";
const DEZ:&str = "X";
const QUARENTA:&str = "XL";
const CINQUENTA:&str = "L";
const NOVENTA:&str = "XC";
const CEM:&str = "C";
const QUATROCENTOS:&str = "CD";
const QUINHENTOS:&str = "D";
const NOVECENTOS:&str = "CM";
const MIL:&str = "M";


// inverte um array de quatro elementos:
fn inverte_array(array:&mut [u8; 4]) {
   for k in 0..array.len()/2{
      // posição do seu último elemento.
      let final_posicao = (array.len()-1) - k;
      // inversão de valores:
      let copia = array[k];
      array[k] = array[final_posicao];
      array[final_posicao] = copia;
   }
}

/* centenas, dezenas e unidades.
 * como só se pode representar(inicialmente)
 * valores romanos até 3999, então o máximo
 * de dígitos é quatro. 
 * Os números serão representados assim como
 * no mundo real, a maior casa(esquerda) para
 * menor casa(direita). */
fn decompoe_algarismos(numero:u16) -> [u8; 4] {
   // quatro algarismos, que é o máximo a representar.
   let mut algs:[u8; 4] = [0, 0, 0, 0];
   // cópia do valor.
   let mut n = numero;
   // posição na array.
   let mut p = 0;
   
   // enquanto o valor for maior que dez aplicar divisão...
   while n > 10 {
      // resto em divisão por dez
      let resto = (n % 10) as u8;
      algs[p] = resto;
      // divide por dez.
      n = n / 10;
      // próximo local na array.
      p += 1;
   }

   // colocando quociente que finaliza algs.
   algs[p] = n as u8;

   // inverte a array.
   inverte_array(&mut algs);
   
   // retorno de algarismos.
   return algs;
}

/* pega uma array representando os algarismos 
 * de determinado número na base dez, e, faz
 * e retorna tal expansão deste número. */
fn potencia_dez_adequada(algarismos:&[u8; 4]) -> [u16; 4] {
   // expansão de um número.
   let mut expansao:[u16; 4] = [0; 4];
   
   for (i, _) in algarismos.into_iter().enumerate() {
      let potencia:u32 = (4 - (1 + i)) as u32;
      let a:u16 = algarismos[i] as u16;
      expansao[i] = a * 10_u16.pow(potencia);
   }

   return expansao;
}

/** pega número e retorna uma string com 
  sua representação em números romanos. */
pub fn decimal_para_romano(numero:u16) -> String {
   // string para concatenação...
   let mut num_str:String = String::from("");
   // transforma os algarismos em múltiplos de potências de dez.
   let decomposicao = decompoe_algarismos(numero);
   let expansao = potencia_dez_adequada(&decomposicao);

   // percorrendo as casas na expansão...
   for a in expansao {
      // centenas.
      if a >= 100 && a <= 900 {
         if a == 100 {
            num_str += CEM;
         }
         else if a == 400 {
            num_str += QUATROCENTOS;
         }
         else if a == 500 {
            num_str += QUINHENTOS;
         }
         else if a == 900 {
            num_str += NOVECENTOS;
         }
         else {
            if a == 200 {
               num_str += CEM;
               num_str += CEM;
            }
            else if a == 300 {
               num_str += CEM;
               num_str += CEM;
               num_str += CEM;
            }
            else if a == 600 {
               num_str += QUINHENTOS;
               num_str += CEM;
            }
            else if a == 700 {
               num_str += QUINHENTOS;
               num_str += CEM;
               num_str += CEM;
            }
            else if a == 800 {
               num_str += QUINHENTOS;
               num_str += CEM;
               num_str += CEM;
               num_str += CEM;
            }
         }
      }

      // casa das dezenas.
      else if a >= 10 && a <= 90 {
         if a == 90 {
            num_str += NOVENTA;
         }
         else if a == 50 {
            num_str += CINQUENTA;
         }
         else if a == 40 {
            num_str += QUARENTA;
         }
         else if a == 10 {
            num_str += DEZ;
         }
         else {
            if a == 80 {
               num_str += CINQUENTA;
               num_str += DEZ;
               num_str += DEZ;
               num_str += DEZ;
            }
            else if a == 70 {
               num_str += CINQUENTA;
               num_str += DEZ;
               num_str += DEZ;
            }
            else if a == 60 {
               num_str += CINQUENTA;
               num_str += DEZ;
            }
            else if a == 30 {
               num_str += DEZ;
               num_str += DEZ;
               num_str += DEZ;
            }
            else if a == 20 {
               num_str += DEZ;
               num_str += DEZ;
            }
         }
      }
      
      // casa das unidades:
      else if a >= 1 && a <= 9 {
         if a == 9 {
            num_str += NOVE;
         }
         else if a == 4 {
            num_str += QUATRO;
         }
         else if a == 5 {
            num_str += CINCO; 
         }
         else if a == 1 {
            num_str += UM;
         }
         else {
            if a == 8 {
               num_str += CINCO;
               num_str += UM;
               num_str += UM;
               num_str += UM;
            }
            else if a == 7 {
               num_str += CINCO;
               num_str += UM;
               num_str += UM;
            }
            else if a == 6 {
               num_str += CINCO;
               num_str += UM;
            }
            else if a == 3 {
               num_str += UM; 
               num_str += UM; 
               num_str += UM; 
            }
            else if a == 2 {
               num_str += UM; 
               num_str += UM; 
            }
         }
      }
      
      // caso contrário, resta apenas as casas
      // das unidades de milhar.
      else {
         if a == 1000 {
            num_str += MIL;
         }
         else if a == 2000 {
            num_str += MIL;
            num_str += MIL;
         }
         else if a == 3000 {
            num_str += MIL;
            num_str += MIL;
            num_str += MIL;
         }
      }
   }
   
   // string representando romanos.
   return num_str;
}


fn conserta_ultimo_alg_fante(algs:&mut Vec<String>, string:&str) {
   // último posição da slice-string.
   let fim = string.len()-1;
   // pega os dois últimos algarismos...
   let ponta = string.get(fim-1..fim+1).unwrap();
   // verifica se os dois últimos algarismos são compostos?
   let e_composto = {
         ponta == "IV" || ponta == "IX" ||
         ponta == "XC" || ponta == "CD" ||
         ponta == "CM" || ponta == "XL"
   };
   // caso não seja, adiciona na array tal símbolo...
   // só neste caso, pois é onde falha a inserção
   // no código que usa tal função, os algs.-compostos
   // pegam bem.
   if !e_composto {
      // pega último alg. e transforma-o em string.
      algs.push(string.get(fim..fim+1)
                     .unwrap()
                     .to_string());
   }
}

fn decompoe_numero_romano(numero_romano:&str) -> Vec<String> {
   /* pega um número romano representando numa
    * string, e separa seus algarismos. O retorno
    * é uma array com todos eles.  */
   // array realocável de slice-strings.
   let mut algs:Vec<String> = Vec::new();
   let mut i = 1;
   let fim = numero_romano.len()-1;
   let mut parte_ac_posicao:&str = "";
   let mut e_composto:bool;

   while i <= fim {
      let a = numero_romano.get(i-1..i+1).unwrap();
      // verifica se os dois símbolos não formam um alg. composto.
      e_composto = {
         a == "IV" || a == "IX" ||
         a == "XC" || a == "CD" ||
         a == "CM" || a == "XL"
      };

      if e_composto {
         algs.push(a.to_string());
         /* grava o segundo símbolo do algarismo composto
          * assim, quando for adicionar o algarismo com
          * uma letra, não confude-se com ele. */
         parte_ac_posicao = numero_romano.get(i..i+1).unwrap();
      }
      else {
         // pega apenas um algarismo.
         let a = numero_romano.get(i-1..i).unwrap();
         // vê se não faz parte do algarismo anterior composto.
         if a != parte_ac_posicao {
            algs.push(a.to_string());
         }
      }
      // vai para o próximo alg.
      i += 1;
   }
   /* por falha no algoritmo de incluir o último
    * algarismo, terceiriza há uma função externa
    * tal trabalho, em vez de remexer todo algarismo
    * para ver se é viável tal concerto. */
   conserta_ultimo_alg_fante(&mut algs, numero_romano); 
   return algs;
}

/** pega uma string representando um número 
  romano e transforma-o num decimal. */
pub fn romano_para_decimal(numero:&str) -> u16 {
   // dicionário para conversão:
   let mut tolkens:HashMap<&str, u16> = HashMap::new();
   let array = [(UM, 1), (QUATRO,4), (CINCO,5), 
                (NOVE, 9),(DEZ, 10), (CINQUENTA,50), 
                (QUARENTA, 40), (NOVENTA, 90), (CEM, 100), 
                (QUATROCENTOS, 400), (QUINHENTOS,500),
                (NOVECENTOS, 900), (MIL, 1000)];
   for (chave, valor) in array {
      tolkens.insert(chave, valor);
   }
   
   let mut acumulador:u16 = 0;
   let algs_romanos = decompoe_numero_romano(numero);
   for alg in algs_romanos {
      let v = tolkens.get(alg.as_str()).unwrap();
      acumulador += *v;
   }
   
   // retorna transformação.
   return acumulador;
}


// ---- ---- --- série de testes ---- ---- ----
#[cfg(test)]
mod tests {
   #[test]
   fn inversao_array() {
      let mut array = [1,2,3,4];
      super::inverte_array(&mut array);
      println!("array agora = {:?}", array);
      assert_eq!([4,3,2,1], array);
   }

   #[test]
   fn decomposicao_algs_i() {
      let numero:u16 = 192;
      let algs = super::decompoe_algarismos(numero);
      assert_eq!([0,1,9,2], algs);
   }

   #[test]
   fn decompoe_algarismos_ii() {
      let algs = super::decompoe_algarismos(9253);
      assert_eq!([9,2,5,3], algs);
   }

   #[test]
   fn decompoe_algarismos_iii() {
      // algora o teste para dois e um algarismos.
      let algs_i = super::decompoe_algarismos(75);
      let algs_ii = super::decompoe_algarismos(8);
      
      let um_algarismo = [0,0,7,5] == algs_i;
      let dois_algarismos = [0,0,0,8] == algs_ii;
      assert!(um_algarismo && dois_algarismos);
   }

   #[test]
   fn expandir_numeros() {
      let valor = 1298;
      let valor_i = 99;
      let valor_ii = 366;
      
      let d = super::decompoe_algarismos(valor);
      let pda = super::potencia_dez_adequada(&d);
      assert_eq!(pda, [1000, 200, 90, 8]);
      let d_i = super::decompoe_algarismos(valor_i);
      let pda_i = super::potencia_dez_adequada(&d_i);
      assert_eq!(pda_i, [0, 0, 90, 9]);
      let d_ii = super::decompoe_algarismos(valor_ii);
      let pda_ii = super::potencia_dez_adequada(&d_ii);
      assert_eq!(pda_ii, [0, 300, 60, 6]);
      println!(
         "expansão({0}) = {3:?}
         \rexpansão({1}) = {4:?}
         \rexpansão({2}) = {5:?}\n", 
         valor, valor_i, valor_ii,
         d, d_i, d_ii
      );
   }

   #[test]
   fn conversao_decimal_para_romano() {
      let romano_teste_i = super::decimal_para_romano(62);
      let romano_teste_ii = super::decimal_para_romano(1005);
      let romano_teste_iii = super::decimal_para_romano(749);

      println!("{} ==> {}",62,romano_teste_i);
      println!("{} ==> {}",1005,romano_teste_ii);
      println!("{} ==> {}",749,romano_teste_iii);

      assert_eq!("LXII", romano_teste_i.as_str());
      assert_eq!("MV", romano_teste_ii.as_str());
      assert_eq!("DCCXLIX",romano_teste_iii.as_str());
   }

   #[test]
   fn romanos_limites() {
      // limites superior e inferior.
      let limite_inferior = super::decimal_para_romano(1);
      let limite_superior = super::decimal_para_romano(3999);
     
      // se mostrar os resultados:
      println!("1 ==> {}", limite_inferior);
      println!("3999 ==> {}", limite_superior);

      assert_eq!("I", limite_inferior.as_str());
      assert_eq!("MMMCMXCIX", limite_superior.as_str());
   }

   #[test]
   fn algarismos_de_romanos() {
      let numero_i = super::decompoe_numero_romano("XLIX");
      let numero_ii = super::decompoe_numero_romano("MMDCXXXIX");
      let numero_iii = super::decompoe_numero_romano("CMXCIV");
      let numero_iv = super::decompoe_numero_romano("MMMVI");
      let numero_v = super::decompoe_numero_romano("CCXCV");

      println!("algs de xlix(49) = {:?}", numero_i);
      println!("algs de mmdcxxxix(2639) = {:?}", numero_ii);
      println!("algs de cmxciv(994) = {:?}", numero_iii);
      println!("algs de mmmvi(3006) = {:?}", numero_iv);
      println!("algs de ccxcv(295) = {:?}", numero_v);

      assert_eq!(vec!["XL","IX"], numero_i);
      assert_eq!(vec!["M","M","D","C", "X","X","X","IX"], numero_ii);
      assert_eq!(vec!["CM","XC","IV"], numero_iii);
      assert_eq!(vec!["M","M","M","V","I"], numero_iv);
      assert_eq!(vec!["C","C","XC","V"], numero_v);
   }
   
   #[test]
   fn transforma_romano_em_decimal_de_volta() {
      let romano = "MMMVI";
      let romanoi = "CCXCV";
      let romanoii = "XXIX";

      // visualizando transformações:
      println!("MMMVI ==> {}", super::romano_para_decimal(romano));
      println!("CCXCV ==> {}", super::romano_para_decimal(romanoi));
      println!("XXIX ==> {}", super::romano_para_decimal(romanoii));

      // verificando compatibilidade.
      assert_eq!(super::romano_para_decimal(romano), 3006);
      assert_eq!(super::romano_para_decimal(romanoi), 295);
      assert_eq!(super::romano_para_decimal(romanoii), 29);
   }

   #[test]
   fn todos_romanos_possiveis() {
      for num in 1..4_000 {
         println!("{} ==> {}", num, super::decimal_para_romano(num as u16));
      }
      assert!(true);
   }
}
