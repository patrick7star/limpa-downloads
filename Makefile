NOME 		= limpa-downloads
VERSAO 	= 0.5.0
BACKUP	= ../versões/$(NOME).v$(VERSAO).tar

salvar:
	tar --wildcards --exclude target -cvf $(BACKUP) \
		lib/ src/ tests/ Cargo.toml definicoes.json Makefile

backups:
	@echo "\n\nListagem dos backups de $(NOME) ...\n"
	@ls --sort time -h -s ../versões/$(NOME)*
	@echo ""

compila-release:
	@cargo rustc  --release --offline -- -Llib/ --extern utilitarios 
