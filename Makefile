NOME 		= limpa-downloads
VERSAO 	= 0.5.1
BACKUP	= ../versões/$(NOME).v$(VERSAO).tar
 
salvar:
	tar --wildcards --exclude target -cvf $(BACKUP) \
		lib/ src/ tests/ Cargo.toml definicoes.json Makefile

backups:
	@echo "\n\nListagem dos backups de $(NOME) ...\n"
	@ls --sort time -h -s ../versões/$(NOME)*
	@echo ""

import-lib:
	@cp -uv $(RUST_CODES)/rust-utilitarios/lib/libutilitarios.rlib $(PWD)/lib/ 
	@echo "lib 'utilitários de Rust' copiado pro projeto." 

compila-release:
	@cargo rustc --verbose --release --offline -- --extern utilitarios -Llib/
