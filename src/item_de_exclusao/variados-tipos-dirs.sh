
# mudando para temporários, em caso
# de interrupção, serão excluída sempre,
# porém não imediatamente.
cd '/tmp'

# pasta própria para colocar tudo.
# também entra nele.
mkdir 'pasta-testes'
cd 'pasta-testes'

# diretórios com subdiretórios em todos os 
# mais profundos níveis, variando é claro.
mkdir --parents amostra_i/subdirs-{A..C}/
mkdir amostra_i/subdirs-A/subsubdirs{1..7}/
mkdir amostra_i/subdirs-B/subsubdirs{2..5}/
mkdir amostra_i/subdirs-C/subsubdirs{1..10}/

mkdir --parents amostra_ii/subdirs-{D..K}/
mkdir amostra_ii/subdirs-{D..K}/subsubdirs{1..3}/

mkdir --parents amostra_iii/subdirs-{L..P}/
mkdir amostra_iii/subdirs-{L..P}/subsubdirs{14..23}/
mkdir amostra_iii/subdirs-M/subsubdirs14/profundo{1..4}/
mkdir amostra_iii/subdirs-N/subsubdirs17/profundo{1..2}/
mkdir amostra_iii/subdirs-P/subsubdirs21/profundo{1..3}/

# amostras com alguns arquivos para diferenciar.
mkdir --parents amostra_iv/subdirs-{A..C}/
mkdir amostra_iv/subdirs-A/subsubdirs{1..7}/
mkdir amostra_iv/subdirs-B/subsubdirs{2..5}/
mkdir amostra_iv/subdirs-C/subsubdirs{1..10}/
touch amostra_iv/subdirs-B/subsubdirs3/dados.{1,2}.dat

# diretórios mais simples.
mkdir --parents amostra_v/unico/

mkdir --parents amostra_vi/unico/
touch amostra_vi/unico/teste.a.data
touch amostra_vi/teste.b.data

mkdir amostra_vii/

