# Cheat sheet               

## Módulo 1

#### Comandos de teclado 

* Ctrl-c: cancelar um comando
* Ctrl-d: indicar fim de dados
* Ctrl-u: apagar uma linha corrente
* Ctrl-s: suspender a impressão
* Ctrl-q: continuar a impressão

#### Manipular diretórios

* pwd: informa o diretório corrente          
* find: procura um arquivo ou diretório no sistema de arquivos
* cd: muda de diretório
* mkdir: cria um diretório 
* rmdir: remove um diretório vazio

#### Manipular arquivos         

* cp: copia um (ou mais) arquivos
* mv: move (renomeia) um (ou mais) arquivos 
* rm: remove um (ou mais) arquivos

#### Manipular conteúdo de arquivos         

* cat: lista um (ou mais) arquivos
* cmp: compara dois arquivos 
* sort: ordena as linhas de um (ou mais) arquivos 
* wc: conta o número de linhas, caracteres e palavras de um arquivo 

#### Caracteres especiais

* $: (dollar ou cifrão) o identificador a seguir é o nome de uma variável
* *: (asterisco) expande para uma lista de arquivos
* ~: (til) substituído pela variável HOME (o shell original /bin/sh não suporta o ~)
* ?: (interrogação) no nome dos arquivos, vale por qualquer caractere ́ ́ ́ ́ ́(acento grave) execução de comando
* <: (menor) redirecionamento da entrada padrão
* >: (maior) redirecionamento da saída padrão
* |: (barra vertical) composição de programas
* \: (barra invertida) protege o próximo caractere
* “: (aspas) texto com substituição de variáveis (exemplo: “texto entre aspas”)
* ‘: (apóstrofe) texto sem substituição de variáveis (exemplo: ‘texto entre apóstrofes’)
* (: (abre-parentesis) sub-shell — termina com “)” (fecha-parentesis)
* !: (exclamação) acesso a história de comandos (apenas em modo interativo)

#### Estrutura de diretórios

* /: (raiz) início do sistema de arquivos
* /bin: utilitários mais comuns (essenciais)
* /dev: dispositivos do sistema
* /etc: informações e definições do sistema
* /home: diretório dos usuários
* /lib: bibliotecas mais comuns
* /man: manuais do sistema
* /mnt: ponto de montagem de sistemas de arquivos
* /opt: pacotes opcionais
* /proc: situação do sistema
* /sbin: utilitários para o administrador
* /tmp: diretório temporário
* /usr: aplicações/programas/utilitários de usuários
* /var: arquivos de trabalho

#### Diretório corrente

* .: diretório corrente
* ..: diretório pai ou diretório acima
* /: diretório raiz (origem)
* ~: diretório $HOME (nem todo shell ou programa aceita ~)

#### Horários dos arquivos e diretórios

* atime: horário  do  último  acesso  ao  arquivo  (alterado  quando  se  lê)
* ctime: horário da última alteração de status do arquivo (alterado quando o arquivo écriado  ou  quando  o  dono  ou  as  permissões  são  alteradas)
* mtime:  horário  da  última  alteração  do  arquivo  (escrita)

## Módulo 2

#### Arquivos padrão

* stdin:  entrada  padrão  (descritor  0)
* stdout:  saída  padrão  (descritor  1)
* stderr:  saída de erros (descritor 2)

#### Histórico de comandos

* history  5: lista  os  ultimos  5  comandos
* !!: reexecuta o último comando
* !date: reexecuta  o  ultimo  comando  date
* !?see: reexecuta a ultima linha que contem o texto see

#### Variáveis do shell

* PS1: indicador que o shell esta pronto para receber um comando (prompt) — veja abaixo
* HOME: diretório  principal  do  usuário
* PATH: relação  de  diretórios  onde  estão  os  programas
* TERM: definição  do  terminal

Além  destas,  existem  outras  variáveis  definidas  no  ambiente  que  são  utilizados  poroutros  programas.  Por  exemplo,

* EDITOR: editor  preferido  pelo  usuário  (ver  emacs  e  vi)
* PAGER: programa  de  paginação  (ver  more  &  less)

#### O Prompt

* PS1: Este é o prompt primário. Existem algumas seqüencias de escape que são expan-didas antes da sua exibição.
	1. \h: significa o nome do computador
	2. \w: significa o nome do diretório corrente
	3. \#: número  do  comando  na  história  de  comandos
* PS2: Utilizado para indicar que o comando está incompleto. O shell aguarda continuação.
* PS3: Exibido  durante  o  comando  [Select].
* PS4: Usado  quando  o  shell  está  executando  em  modo  de  depuração

#### Entrada e Saída

* <: da  entrada  padrão
* \>: da  saída  padrão,  sobrescreve
* \>>: da  saída  padrão,  adiciona
* 2>: da  saída  de  erros
* |: da  saída  padrão  para  a  entrada  padrão  (pipe)

## Módulo 3 e 4

#### Utilitários de manipulação de arquivos e diretórios

* ls: listar  conteúdo  de  diretório
* find: procurar  arquivos
* ln: criar  um  “sinônimo”  para  um  arquivo
* cp & mv & rm: copiar,  renomear  e  remover
* chown & chmod: alterar  dono  e  permissões  dos  arquivos
* file: informar  o  tipo  de  um  arquivo  examinando  seu  conteúdo
* du & df: utilização de espaço em disco
* mkdir & rmdir: criar  e  remover  diretórios
* cmp: comparar  dois  arquivos

#### Processamento de Textos

* echo: imprime  os  argumentos
* cat: imprime  o  conteúdo  de  um  arquivo
* head & tail: imprime  as  primeiras  ou  últimas  linhas  de  um  arquivo
* sort: ordena  um  arquivo
* cut: seleciona  campos  de  um  arquivo
* uniq: elimina  linhas  duplicadas  em  um  arquivo
* tr: substitui  caracteres
* wc: conta linhas, palavras e caracteres de um arquivo
* grep: procura  por  um  padrão  em  um  arquivo
* sed: edição  programada  de  um  arquivo
* awk: processamento  de  testos  e  padrões
* more & less: paginação  de  arquivo
* join: união  de  arquivos  por  um  campo  comum
* diff: comparação  de  dois  arquivos

#### Expressões regulares

* x: onde x é um caractere sem significado especial, “casa” com o caractere. Este é o quechamamos de caractere literal
* .: (ponto) "casa" com qualquer caractere
* [aeiou]: define que estamos procurando por qualquer um dos caracteres entre "[" e "]".
* [\^aeiou]: define que estamos procurando por um caractere diferente dos que estão entre "[" e "]". No caso, não apenas as consoantes mas também os caracteres de pontuação,espaço em branco, etc

#### Ancoras

* ^: início do string (ex: ^A, uma palavra que começa com A)
* $: fim do string (ex: $B, uma palavra que termina com B)

#### Repetições e Opcionais

* \*: zero ou mais vezes
* +: uma ou mais vezes (apenas no egrep)
* ?: opcional (apenas no egrep)

* Exemplos:
	1. * a: deve conter uma letra a
	2. * .\*: qualquer caractere repetido zero ou mais vezes
	3. * e: seguido de uma letra e

## Módulo 7

#### Controle de fluxo

* if: condicional um  comando  é  executado  se  uma  determinada  condição  é  satisfeita
	1. if test “nao” = “$resposta” then
		echo «Que pena. Eu estava contanto com este dinheiro» 
	else    
	echo “Obrigado pela sua generosa doacao :-)” 
	fi
* for: iteração um comando é executado para cada elemento de uma lista
	1. for sobrinho in Huginho Zezinho Luizinho
	do
		echo “$sobrinho diz: Boa noite, Tio Donald!”
	done
* while: iteração um  comando  é  executado  enquanto  uma  determinada  condição  for  válida
	1. while test $contador -lt 5;
	do 
		echo “contador=$contador”;
		contador=`expr $contador + 1`;
	done

#### A função test

* -f  ARQUIVO: verifica se um arquivo existe e não está vazio
* -N  ARQUIVO: verifica se o arquivo foi modificado desde a última vez que foi lido
* ARQ1  -nt  ARQ2: se ARQ1 foi modificado mais recentemente que ARQ2
* -z  VARIAVEL: testa se a variável está vazia
* -n  VARIAVEL: teste se a variável não está vazia
* STRING1 = STRING2: verifica se os dois strings (sequencias de caracteres) são iguais
* STRING1 != STRING2: verifica se STRING1  é  diferente  de  STRING2
* EXPR1  OP  EXPR2: verifica se a expressão 1 (EXPR1) é OP a expressão 2 (EXPR2). OP  é  um  dos  seguintes  operadores:
	1. -lt: menor
	2. -le: menor  ou  igual
	3. -eq: igual
	4. -ne: diferente
	5. -ge: maior  ou  igual
	6. -gt: maior

#### Acesso a Rede

* UDP: User Datagram Protocol
* TCP: Transmission Control Protocol
* ICMP: Internet Control Message Protocol

#### Processos

* daytime: (porta 13) retorna data e horas correntes
* http: (porta 80) HyperText Protocol, utilizado para acessar páginas web (WWW - World Wide Web). Utiliza TCP
* smtp: (porta 25) Simple Mail Transfer Protocol, utilizado para transferência de e-mails através de TCP/IP

## Módulo 9

#### Estendendo o Ambiente

* C: linguagem compilada de alta performance
* shell: e por que nao?
* perl: linguagem interpretada voltada a composição de ferramentas
* tcl: linguagem interpretada voltada a composição de ferramentas
* [ruby]: linguagem interpretada orienteada a objetos
* [python]: linguagem interpretada de alto nível

## Módulo 10

#### O Sistema de Arquivos como Base de Dados

* ACID: Atomicidade Consistência Isolamento Durabilidade
* Atomicidade: ou a operação acontece ou não. Resultados intermediário não são perceptiveis.
* Consistência: não é possível executar uma operação que gere dados inválidos.
* Isolamento: se duas ou mais operações acontecem ao mesmo tempo, o resultado é o mesmo se elas tivessem sido executadas uma após a outra. Em particular, uma operação não vê os resultados intermediários da outra.
* Durabilidade: uma vez terminada uma operação os seus resultados são permanentementes no sentido que persistem mesmo que o computador falhe.
