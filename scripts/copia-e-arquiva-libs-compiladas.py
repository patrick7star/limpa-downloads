#!/usr/bin/python3 -O
"""
   Ao invés de copiar e renomear toda vez as bibliotecas rust compiladas, de
 forma puramente manual, em uma dada arquitetura, para algum subdiretório da 
 pasta 'lib' do Rust referente a tal arquitetura é claro. Iremos automatizar
 tal processo com um script. 

 Nota: Este script foi copiado do projeto 'cobrinha clássica', e modificado
       para atender este projeto. Por isso, não estranhe se houver alguma
       coisa não conforme, já que ele não foi inicialmente desenhado para
       esta aplicação. Também pode ter sido ele mesmo ter sido copiad de
       algum projeto mais antigo. Está aqui o motivo para eu colocar esta
       nota, já que não lembro se este é o caso. Tais notas, ajudarão no 
       futuro saber quando é uma cópia ou não. Outro indicativo de ser a 
       pura cópia de outro projeto é que, geralmente eu deleto os trechos
       de testes unitários, apesar de, às vezes faços testes unitários prás
       leves modificações feitas, então pode haver testes unitários, porém não
       seriam os mesmos dos arquivo original. Você pode pensar que os nomes
       iguais ajudam, entretanto, eu geralmente ou modifico nomes -- pois achei
       um melhor do que naquele momento que criei pela primeira vez; ou também
       alterei pois, uso os "escopos" das pastas para ser o mais genérico 
       possível com nomes, então não havia qualquer alternativa.
"""

import platform, shutil, os, glob
from pathlib import (Path)
from tarfile import (TarFile)

sistema = platform.system().lower()
maquina = platform.machine()
nome_do_diretorio = "{}_{}".format(sistema, maquina)
DESTINO = Path("lib", nome_do_diretorio, "binaries")
if __debug__:
	DIRETORIO_FONTE = Path("target/debug/deps")
else:
	DIRETORIO_FONTE = Path("target/release/deps")
PADRAO = DIRETORIO_FONTE.joinpath("lib*.rlib")

# === === ===  === === === === === === === === === === === === === === ==== ==
#                          Funções Auxiliares 
# === === ===  === === === === === === === === === === === === === === ==== ==
def cria_diretorio_se_necessario() -> None:
   # Verifica se o diretório já é existente, se não, então o cria.
   try:
      os.makedirs(DESTINO)
   except FileExistsError:
      print("O diretório '{}' já existe.")
   finally:
      assert(DESTINO.exists())

def realiza_copia_de_todos_bibliotecas_disponiveis() -> None:
   contador = 0

   print("Realizando copia dos binários ...")

   for caminho in glob.glob(str(PADRAO)):
      caminho = Path(caminho)
      print('\t\b\b-', caminho.name) 
      shutil.copy2(caminho, DESTINO)
      contador += 1

   print("\n... para '%s'." % DESTINO)
   print("Todas %d libs foram copiadas com sucesso." % contador)

def apara_este_grande_id_do_nome(caminho: Path) -> Path:
   """
      Você pode observar, que há algo bem estranho concatenado ao seu nome. 
    Por isso, vamos arrancar isso, e deixar apenas o nome normal. Como está
    entre um ponto(.) e um traço(-), tal operação fica bastante razoável.
   """
   diretorio = caminho.parent
   atual_nome = str(caminho.name)
   # Vem pela esquerda para achar o traço, e pela direita pelo ponto.
   p = atual_nome.index('-')
   q = atual_nome.rindex('.')
   # Copia tudo, menos este trecho. Então, forma o novo caminho com tal.
   novo_nome = atual_nome[0:p] + atual_nome[q:]

   return diretorio.joinpath(novo_nome)

def renomea_todos_libs_copiadas() -> None:
   caminho = destino.joinpath("./*.rlib")
   lista_de_binarios = glob.glob(str(caminho))

   print("\nAplicando processo de renomeação ...")

   for entrada in lista_de_binarios:
      In = Path(entrada)
      Out = apara_este_grande_id_do_nome(In)

      # Pode pode também ser um caminho absoluto.
      In.rename(Out)
      print("\t\b\b{} ==> {}".format(In.name, Out.name))
      # Confirma renomeação.
      assert(not In.exists())
      assert(Out.exists())

   print("\n... processo terminado.\n")

def cria_hard_linques_sem_o_codigo() -> None:
   regex = DESTINO.joinpath("./*.rlib")
   lista_das_novas_fontes = glob.glob(str(regex))

   print(
      "\nLincado vários binários referente na base do diretório com um " + 
      "nome mais limpo."
   )

   for entrada in lista_das_novas_fontes:
      In = Path(entrada)
      Out = apara_este_grande_id_do_nome(In)
      # Reprocessa saída para ir um diretório acima. Faz um linque no
      # diretório 'pai' do subdiretório com binários.
      base = Out.parent.parent
      Out = base.joinpath(Out.name)

      # Pode pode também ser um caminho absoluto.
      Out.hardlink_to(In)
      print("\t\b\b'{}' \u27fe '{}'".format(In.name, Out.name))

      # Confirma renomeação.
      assert(Out.exists())

   print("\n... processo terminado.\n")

def salva_depedencias_ja_compiladas() -> None:
	"""
	Método bem mais direto. Não precisa das cópias, apenas vai no repositório
	com as lib já compilada, então linca(hardlink) ao diretório de 'lib' que 
	representa o sistema e arquitetura do processador.
	"""
	regex = str(PADRAO)
	lista_das_novas_fontes = glob.glob(regex)
	SETA_UNICODE = '\u27fe'

	print(
		"\nLincado vários binários referente na base do diretório com um " + 
		"nome mais limpo."
	)

	for entrada in lista_das_novas_fontes:
		In = Path(entrada)
		Out = apara_este_grande_id_do_nome(In)
		base = DESTINO.parent
		Out = base.joinpath(Out.name)

		# Pode pode também ser um caminho absoluto.
		try:
			Out.hardlink_to(In)
			print("\t\b\b'{}' {} '{}'".format(In.name, SETA_UNICODE, Out.name))
		except FileExistsError:
			print("O linque '{}' já foi copiado, ou está lá.".format(Out.name))

		# Confirma renomeação.
		assert(Out.exists())

	print("\n... processo terminado.\n")

def arquivacao_dos_binarios_copiados() -> None:
   caminho = "lib/{}.tar".format(nome_do_diretorio)
   archive = TarFile.open(caminho, mode="x") 
   regex = "lib/{}/*.rlib".format(nome_do_diretorio)

   for item in glob.glob(regex):
      archive.add(item)
   archive.close()
# === === ===  === === === === === === === === === === === === === === ====
#                       Execução do Script
# === === ===  === === === === === === === === === === === === === === ====
if __name__ == "__main__":
   print(
      """
      \rFaz uma cópia das bibliotecas que foram compiladas nesta máquina.
      \rEles estarão localizadas em 'lib', no diretório da devida arquitetura
      \re OS. O processo é mais direto, pois corta a antiga cópia, que parece
      \rdesnecessária, pois a lincagem já é realizada com 'hardlink'. O outro
      \rera com simbólicos, depois foi transportado para este, por a cópia
      \rde bibliotecas inicialmente. O código foi revisto, e aperfeiçoado.
      """
   )
   print("Local:", DESTINO.parent)
   cria_diretorio_se_necessario()
   # Remove diretório desnecessário neste algoritmo.
   DESTINO.rmdir()
   salva_depedencias_ja_compiladas()
   print("Agora, fazendo arquivação deles ...", end=' ')
   arquivacao_dos_binarios_copiados()
   print("feito.")

from unittest import (TestCase)

class ArquivacaoDosBinariosCopiados(TestCase):
   def setUp(self):
      caminho = "lib/{}.tar".format(nome_do_diretorio)
      self.file = Path(caminho)
      print("File: '%s'" % str(self.file))
   
   def tearDown(self):
      print("Excluindo '%s' ..." % str(self.file), end=' ')
      self.file.unlink()
      print("feito")

   def runTest(self):
      try:
         arquivacao_dos_binarios_copiados()
      except FileExistsError:
         print("Arquivo já existe! Deletando, e tentando novamente.")
         self.file.unlink()
         arquivacao_dos_binarios_copiados()
      else: pass
      finally: pass

