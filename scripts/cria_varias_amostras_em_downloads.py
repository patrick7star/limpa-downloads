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
from multiprocessing import (Process)


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
   """
   Cria um arquivo, dado o caminho, dada também a extensão. No fim, preenche
   ele com uma quantia, arbitrariamente escolhida, com bytes zerados.
   """
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
         arquivo.write(b'\0')

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

def gerador_aleatorio_de_varias_extensoes_paralelo() -> None:
   destino = Path(getenv("HOME"), "Downloads") 
   pool = []

   with open("definicoes.json", "rt") as stream:
      definicoes = json.load(stream) 

      print("\nCriação dos arquivos com as devidas extensões ...\n")

      for extensao in definicoes.keys():
         funcao = cria_arquivo_branco
         ID = Process(target=funcao, args=(destino, extensao))

         ID.start()
         pool.append(ID)

      print("Todas criações lançadas em paralelo.")

      for fio in pool:
         fio.join(None)
         print("Tarefa [%d] terminada com sucesso." % fio.pid)
      print("Tudo foi executado conforme.")

# === === ===  === === === === === === === === === === === === === === === ==
#	                  Testes Unitários 
# === === ===  === === === === === === === === === === === === === === === ==
class AmostrasDeStringsGeradasRandomicamente(unittest.TestCase):
   def runTest(self):
      for _ in range(10):
         t = randint(15, 30)
         print(gera_uma_string_aleatoria(t))

class CriacaoDeArquivoPraDeterminadaExtensao(unittest.TestCase):
   def runTest(self):
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

class GeracaoAleatoriaDeVariasExtensoesEmDefinicoes(unittest.TestCase):
   def setUp(self):
      self.outputs = []

   def tearDown(self):
      print("\nRemoção do que foi criado:")

      for pathname in self.outputs:
         pathname.unlink()
         self.assertFalse(pathname.exists())
         print('\t', pathname,"...", "removido.")

   def runTest(self):
      destino = Path(getenv("HOME"), "Downloads") 

      with open("definicoes.json", "rt") as stream:
         definicoes = json.load(stream) 

         print("Criação por iteração das extensões e suas validades:\n")
         for extensao in definicoes.keys():
            result = cria_arquivo_branco(destino, extensao)
            self.outputs.append(result)

@unittest.skip("Tarefa consome o máximo de CPU e energia")
class GeradorAleatorioDeVariasExtensoesParalelo (unittest.TestCase):
   def runTest(self):
      gerador_aleatorio_de_varias_extensoes_paralelo()

# === === ===  === === === === === === === === === === === === === === === ==
#	                   Execução do Programa
# === === ===  === === === === === === === === === === === === === === === ==
import getopt
from sys import (argv as Argumentos)

def manual_de_uso() -> None:
   print("""
   \rTodas opções que podem ser usadas no script.

   --help\tEste manual de ajuda aqui mostrado.

   --max\tPotência máxima ao gerar tais arquivos. Cuidado, esta
        \topção pode realmente "congestionar" o computador, pois ele usa
        \to máximo do CPU(todos cores).
   """)

if __name__ == "__main__":
   filtrado = Argumentos[1:]
   (menu, _) = getopt.getopt(filtrado, "mh:", ["max", "help"])

   #geracao_aleatoria_de_varias_extensoes_em_definicoes()
   if menu == []:
      geracao_aleatoria_de_varias_extensoes_em_definicoes()

   for (opcao, valor) in menu:
      if opcao == "--help":
         manual_de_uso()

      elif opcao == "--max":
         print(
            "\nAtenção! Quase todo poder computacional da sua máquina "
            + "cuidará desta tarefa. Algumas máquinas podem travar durante "
            + "tal realização\n"
         )
         gerador_aleatorio_de_varias_extensoes_paralelo()


