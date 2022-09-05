# Limpa Downloads
O gerenciador do diretório downloads em sistemas *Linux*. Ele verifica o tempo que arquivos e pastas ali dentro estão sem qualquer movimento, assim com o tipo do arquivo e tal. Depois de tal análise faz a exclusão de tais arquivos para economizar disco e deixar tal diretório limpo para novos downloads. A análise é muito organizada, não é qualquer `Item` recém movido para lá que é automaticamente deletado, trabalha com muitos "metadados" do arquivo para, aí sim, declará a exclusão em algum período posterior.

O programa tem duas opções a serem executadas. A primeira diz a respeito da saída padrão no terminal, à cada 30seg ele retorna um 'output' dizendo o estado dos `Item` a serem excluídos, contendo barra de progresso, tempo de espera para tal; a fila é separada de forma bem organizada. O temporizador mostra até quando será mostrada tal, que é muito, dependendo se há `Items` que serão excluídos em alguns minutos ou não. A outra forma é se o mesmo acima, porém executada usando o **ncurses**; tal têm uma dinâmica melhor, com o progresso e nome rolando em tempo real, sem falar de coloração que depende do estado do `Item`.
Para ativar este segundo modo, já que o primeiro é apenas executar o binário... é preciso passar o argumento **ncurses** quando executar o binário

#### *Nota*:
Se você não têm uma pasta "Downloads" onde ficam todos seus downloads feitos via browser ou qualquer outro programa que trabalha com internet,... tal programa não funcionará bem, é preciso fazer criar tal, e depositar todo despejo nele. É possível que o programa nem chegue a rodar se tal requisito não for atendido.

### Requisitos: 
Tal programa é testado principalmente no *Ubuntu-mate 20.04.4 LTS*, más não há nenhum motivo de que não vá funcionar em qualquer distribuição Linux. A versão padrão funciona até no *Windows 10*.

# Detalhes
No lado esquerdo ficam os *nomes dos arquivos* -- alguns se forem grandes demais para caber no devido espaço, ficam como numa placa de LED se movendo da direita à esquerda para que o nome completo seja mostrado -- que estão nas listas de exclusões, já no lado direito ficam a barra mostrando: mais à esquerda o *tempo restantes*(de forma legível), no centro *a barra de progresso* visual, e  o *percentual númerico* do restante.
![Exemplo do texto acima](https://drive.google.com/drive/folders/1lajq62hNKcPnrKLFC_tVkTQrR4Ewu02a)
