"""
   Pega os arquivos de startups já feitos, então faz deste programa algo
 que dispara para limpeza periódica de modo local.
"""

from pathlib import (Path)
from os import (getenv, execl)
from shutil import (copy2)



def copia_arquivos_de_configuracao_para_repositorio_local() -> None:
   BASE = getenv("HOME")
   COMPLEMENTO = ".config/systemd/user"
   DESTINO = Path(BASE).joinpath(COMPLEMENTO)
   FONTES = [
      Path("data/limpa-downloads.service"), 
      Path("data/limpa-downloads.timer"), 
   ]
         

   if __debug__:
      print("O caminho '{}' existe? {}".format(
         DESTINO, DESTINO.exists()
            ))

   DESTINO.mkdir(parents=True, exist_ok=True)

   for fonte in FONTES:
      copy2(fonte, DESTINO)

def inicializacao_dos_servico_e_temporizador() -> None:
   PROGRAMA = "/usr/bin/systemctl"

   execl(PROGRAMA, "--user", "daemon-reload")
   execl(PROGRAMA, "--user", "enable", "--now",
      "limpa-downloads.service",
      "limpa-downloads.timer"
      )

def altera_arquivos_para_atual_usuario() -> None:
   COMPLEMENTO = "data/limpa-downloads.service"
   CHAVE = 'ExecStart'
   caminho = Path().joinpath(COMPLEMENTO)
   caminho = caminho.absolute()
   arquivo = open(caminho, "rt")
   conteudo = arquivo.read()

   if __debug__:
      executavel = Path().joinpath("target/debug/limpa_downloads")
   else:
      executavel = Path().joinpath("target/release/limpa_downloads")

   executavel = executavel.absolute()
   arquivo.close()

   # Verifica se já tem tal caminho, então abando o resto do processo.
   if str(executavel) in conteudo:
      print("Abandonado pois já foi modificado!")
      return None

   with open(caminho, "wt") as stream:
      for linha in conteudo.split('\n'):
         if CHAVE in linha:
            print("%s=%s" % (CHAVE, executavel),file=stream)
         else:
            print(linha, file=stream)

altera_arquivos_para_atual_usuario()
copia_arquivos_de_configuracao_para_repositorio_local()
inicializacao_dos_servico_e_temporizador()
