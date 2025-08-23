import os
from threading import (Thread, Lock)

CONTADOR = 0
RECURSOS = Lock()

def lanca_programa() -> None:
   global CONTADOR
   
   RECURSOS.acquire()
   CONTADOR += 1
   RECURSOS.release()

   RECURSOS.acquire()
   print("\n{}ª) Lançamento:".format(CONTADOR))
   RECURSOS.release()
   os.system("src/instancia/bin/debug")

fio_a = Thread(target=lanca_programa)
fio_b = Thread(target=lanca_programa)
fio_c = Thread(target=lanca_programa)

fio_a.start()
fio_b.start()
fio_c.start()

fio_a.join()
fio_b.join()
fio_c.join()
