/*   Verifica se o tipo de programa já está em execução. Aqui, irei 
 * apresentar diferentes modos. Entretanto, o principal será sobre 
 * vasculhar o caminho dos processos, e verificar se bate com este aqui em 
 * execução. Parece um principio lento, mas tentarei otimiza-lo ao máximo.
 */

#include "instancia.h"
// Biblioteca padrão do C:
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <ctype.h>
// Bibliotecas do Sistema:
#include <dirent.h>
#include <unistd.h>

struct Lista { char** caminhos; int total; };
const int T = 300;


static void free_lista(struct Lista obj) {
   for (int n = 1; n <= obj.total; n++) 
      free(obj.caminhos[n - 1]);
}

static bool totalmente_numerico(char* string) 
{
   for (int n = 0; n < strlen(string); n++)  {
      char simbolo = string[n];

      if (!isdigit(simbolo))
         return false; 
   }
   return true;
}

static char* forma_caminho(char* In_a, char* In_b) {
   int comprimento = strlen(In_a) + strlen(In_b) + 3;
   const int sz = sizeof(char);
   char* output = (char*)malloc(comprimento * sz);

   strcpy(output, In_a);
   strcat(output, "/");
   strcat(output, In_b);
   strcat(output, "/");
   strcat(output, "exe");
   return output;
}

static bool entrada_invalida(struct dirent* entrada) {
/* Algumas restrições que, já ignora de cara se deve processar tal diretório
 * ou não. Os 'atual' e 'anterior' totalmente. Se for um arquivo, obviamente
 * também. Valores de 'pid' menores que mil são reservados para o sistema,
 * então podem ser descartado também pra este tipo de aplicação, que roda
 * sempre com o privilégio de usuário. Outro seria o próprio 'pid' do 
 * processo que faz esta verificação. Ele também é descartável. */
   const char* nome = (*entrada).d_name;
   unsigned char tipo = (*entrada).d_type;
   int pid = atoi(nome);

   if (strcmp(nome, ".") == 0)
      return true;
   else if (strcmp(nome, "..") == 0)
      return true;
   else if (tipo != DT_DIR)
      return true;
   else if (pid < 1000)
      return true;
   else if (pid == getpid())
      return true;
   else
      return false;
}

static struct Lista filtra_processos(void) {
   const int M = 2000, size = sizeof(char*);
   char** lista = (char**)calloc(M, size);
   struct Lista output = { .caminhos=lista, .total=0 };
   char* const RAIZ = "/proc";
   DIR* iterador = opendir(RAIZ);
   struct dirent* atual;
   char* nome = NULL, *caminho = NULL;

   do {
      atual = readdir(iterador);
      if (atual == NULL)
         break;
      nome = (*atual).d_name;

      if (entrada_invalida(atual))
         continue;

      if (totalmente_numerico(nome)) {
         caminho = forma_caminho(RAIZ, nome);

         output.caminhos[output.total] = caminho;
         output.total += 1;
      }

   } while(true) ;

   return output;
}

static bool path_eq(char* path_a, char* path_b) 
   { return strcmp(path_a, path_b) == 0; }

static char* executavel_do_processo(void) {
   const int size = sizeof(char);
   char* output = calloc(T, size);
   char caminho[T];

   sprintf(caminho, "/proc/%d/exe", getpid());
   readlink(caminho, output, T);
   return output;
}

bool ha_outra_instancia_em_execucacao(void) {
/* Verifica se há outra instância deste processo lançado em execução. */
   struct Lista Out = filtra_processos();
   char programa[T];
   char* executavel = executavel_do_processo();

   memset(programa, 0x0, T);
   realpath(executavel, programa);

   for (int n = 1; n <= Out.total; n++) {
      memset(executavel, 0x0, T);
      readlink(Out.caminhos[n - 1], executavel, T);

      if (path_eq(programa, executavel))
         return true;
   }

   // Liberando alocações feitas. Então, responde a "pergunta" com negativo.
   free_lista(Out);
   free(executavel);
   return false;
}


#if defined(__unit_tests__) && defined(__linux__)
#define stringfy(S) (S? "true" : "false")

void estrutura_do_algoritmo(char* argumentos[]) {
   struct Lista Out = filtra_processos();
   const int T = 200;
   char executavel[T], programa[T];
   bool corresponde = false;

   memset(programa, 0x0, T);
   realpath(argumentos[0], programa);

   for (int n = 1; n <= Out.total; n++) {
      memset(executavel, 0x0, T);
      readlink(Out.caminhos[n - 1], executavel, T);
      corresponde = path_eq(programa, executavel);

      printf(
         "'%s' é o caminho deste programa? %s\n", 
         Out.caminhos[n - 1], stringfy(corresponde)
      );
   }
   free_lista(Out);
}

int main(int N, char* argv[]) 
{
   estrutura_do_algoritmo(argv);
   return EXIT_SUCCESS;
}
#endif
