""" Compilação da biblioteca estática, e testes também, deste projeto. """

from os import (mkdir as make_directory, system as System, wait)

FONTE  = "main.c"
DEBUG  = "bin/debug"
OBJETO = "bin/instancia.obj"
LIB    = "bin/libinstancia.a"


try:
   make_directory("bin")
except FileExistsError:
   print("O diretório 'bin' já existe.")
finally:
   pass

print("\nCompilando seus testes unitários ...", end=" ")
System(
   "clang -O0 -Wall -g3 -std=gnu18 -pedantic -D__unit_tests__ -o %s %s" 
   % (DEBUG, FONTE)
)
print("feito")
print("Compilando a biblioteca estática ...", end=" ")
System("clang -Wall -std=gnu18 -pedantic -c -o %s %s" % (OBJETO, FONTE))
print("feito")
print("Arquivando objeto estático ...", end=" ")
System("ar rcs %s %s" % (LIB, OBJETO))
print("feito")


