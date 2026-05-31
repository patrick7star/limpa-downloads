NOME 		= limpa-downloads
VERSAO 	= 0.5.1
BACKUP	= ../versões/$(NOME).v$(VERSAO).tar
 
salvar:
	tar --wildcards --exclude target -cvf $(BACKUP) \
		lib/ src/ tests/ Cargo.toml definicoes.json Makefile

backups:
	@echo "\n\nListagem dos backups de $(NOME) ...\n"
	@ls --sort time -h -s ../versões/$(NOME)*
	@echo "feito."

import-lib:
	@ln -s $(UNIX_CODES)/rust-utilitarios ./lib
	@echo "Cria um linque no diretório de bibliotecas externas."

compila-release:
	@cargo rustc --verbose --release --offline -- --extern utilitarios -Llib/

