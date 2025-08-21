"""
   Para monitoramento do programa, será preciso de algumas amostras. E é 
 exatamente isso que este script faz, ele cria simples amostras no diretório
 'Downloads', assim o programa pode excluí-los.

   Pra isso, ele le o JSON com todas as extensões, e suas devidas validades,
 principalmente a parte de 'debug', então cria randomicamente arquivos com
 tais extensões. Veja que, os arquivos não são realmente reais, apenas as
 suas extensões -- e futuramente o magic number também, assim você não conta
 apenas com a extensão de tal arquivo pra identficação -- tais arquivos 
 preenchido com uma quantia randomica de bytes(zeros).
"""

import json, string, unittest
from random import (randint)
from pathlib import (Path)
from os import (getenv)


def gera_uma_string_aleatoria(n: int) -> str:
   assert (n >= 10)

   grande_string_seletora = (
      string.ascii_letters + string.ascii_lowercase + 
      string.ascii_uppercase + string.digits + string.hexdigits +
      string.octdigits
   )
   selecoes = []
   total = len(grande_string_seletora)

   for _ in range(n):
      P = randint(0, total - 1)
      selecoes.append(grande_string_seletora[P])
   return "".join(selecoes)

def cria_arquivo_branco(caminho: Path, extensao: str) -> Path:
   assert (caminho.exists())   
   assert (extensao != "")

   nome_aleatorio = gera_uma_string_aleatoria(randint(15, 20))
   nome = nome_aleatorio + "." + extensao
   pathname = caminho.joinpath(nome)
   # De kB até mB.
   BARREIRAS = (1_200, 2_593_811)
   quantia_aleatoria = lambda: randint(*BARREIRAS)

   with open(pathname, "wb") as arquivo:
      for _ in range(quantia_aleatoria()):
         arquivo.write(b'0')

   print("'{}' gerado com sucesso em '{}'.".format(nome, caminho))
   return pathname

def geracao_aleatoria_de_varias_extensoes_em_definicoes() -> None:
      destino = Path(getenv("HOME"), "Downloads") 

      with open("definicoes.json", "rt") as stream:
         definicoes = json.load(stream) 

         print("\nCriação dos arquivos com as devidas extensões ...\n")
         for extensao in definicoes.keys():
            result = cria_arquivo_branco(destino, extensao)
         print("Finalizado.")

class Unitarios(unittest.TestCase):
   def amostras_de_strings_geradas_randomicamente(self):
      for _ in range(10):
         t = randint(15, 30)
         print(gera_uma_string_aleatoria(t))

   def criacao_de_arquivo_pra_determinada_extensao(self):
      destino = Path(getenv("HOME"), "Downloads") 
      extensoes = ["mp4", "pdf", "dat"]
      outputs = []

      for ext in extensoes:
         result = cria_arquivo_branco(destino, ext)
         self.assertTrue(result.exists())
         outputs.append(result)

      print("Excluindo arquivos gerados:")
      for pathname in outputs:
         print('\t', pathname,"...", "removido.")
         pathname.unlink()
         self.assertFalse(pathname.exists())

   def geracaoAleatoriaDeVariasExtensoesEmDefinicoes(self):
      destino = Path(getenv("HOME"), "Downloads") 
      outputs = []

      with open("definicoes.json", "rt") as stream:
         definicoes = json.load(stream) 

         print("Criação por iteração das extensões e suas validades:\n")
         for extensao in definicoes.keys():
            result = cria_arquivo_branco(destino, extensao)
            outputs.append(result)

         print("\nRemoção do que foi criado:")
         for pathname in outputs:
            pathname.unlink()
            self.assertFalse(pathname.exists())
            print('\t', pathname,"...", "removido.")


if __name__ == "__main__":
   geracao_aleatoria_de_varias_extensoes_em_definicoes()
