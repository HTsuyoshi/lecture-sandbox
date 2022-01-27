# respostas
## grep 

1. Liste os estados ou capitais que começam por vogal
```bash
grep '[aeiou][aeiou]' capitais.txt
```

2. Liste os estados ou capitais quec começam por vogal
```bash
grep '=[aeiou]' capitais.txt
```

3. Liste apenas as linhas onde os estados começam por vogal
```bash
grep '^[AEIOU]' capitais.txt
```

4. Liste as capitais que terminam em consoante
```bash
grep '[^aeiou]$' capitais.txt
```
5. No diretório /usr/share/linux.see existe o arquivo vocabulario.txt com uma relaçãode palavras em português. Como você poderia utiliza-lo para “trapacear” num jogo deforca?

Usando o grep como filtro e uma máscara com um '.' no lugar das letras que ainda não foram reveladas.

6. Escreva uma ou mais expressões regulares para encontrar alguns erros em textos em portugues. Por exemplo depois de q sempre vem um u, usa-se m antes de b e p, alguns plurais terminam em "ns" (e não "ms").
```bash
grep 'n[bp]' texto.txt
grep 'q[^u]' texto.txt
grep 'ms' texto.txt
```

7. Em um caso mais complicado, verifique a existencia de letras duplicadas. Pares de r, ssão válidos. Outros válidos como ee, oo são pouco frequentes e devem ser assinaladoscomo erros.
```bash
grep '\([a-qt-z]\)\1' texto.txt
```

=== sed ===

1. Verifique o que acontece no exemplo acima se você colocar a substituição da assinatura (XXX) antes das outras.
```bash
sed -e 's/XXX/Maria Madalena/' -e 's/X/do funil/' -e 's@D@7/9@' -e 's#Q#uma garrafa (cheia) de refrigerante#' modelo.txt
```

2. Liste os estados ou capitais quec começam por vogal
Olá!
Nós da turma do funil, vamos nos reunir dia 7/9, para comemorarmos o início das férias.
Cada participante deve levar um item. Você deve levar uma garrafa (cheia) de refrigerante.
Nos veremos lá.
Maria Madalena

2. Mude o texto Maria Madalena para Charles Xavier e veja o que acontece.
```bash
sed -e 's/XXX/Charles Xavier/' -e 's/X/do funil/' -e 's@D@7/9@' -e 's#Q#uma garrafa (cheia) de refrigerante#' modelo.txt
```

Olá!
Nós da turma do funil, vamos nos reunir dia 7/9, para comemorarmos o início das férias.
Cada participante deve levar um item. Você deve levar uma garrafa (cheia) de refrigerante.
Nos veremos lá.
Charles do funilavier

3. Como você poderia garantir que a substituição sempre funcionasse?
```bash
sed -e 's/XXX/Charles Xavier/' -e 's/\sX\s/do funil/' -e 's@D@7/9@' -e 's#Q#uma garrafa (cheia) de refrigerante#' modelo.txt
```

=== awk ===
1. Imprima cada linha do arquivo (apenas os 9 primeiros campos) com os dois primeiros campos trocados. Exemplo um grande pais seria impresso como grande pum pais.
```bash
awk '{print $1; print $2; for (i=3; i<=9; i++) print $i}' citacao.txt
```

2. Modifique o script acima para imprimir também o número da linha mais longa.
```bash
awk 'BEGIN {maior=0; numeroLinha=0; linha=0} {numeroLinha++; if (length($0) > maior) { maior=length($0); linha=numeroLinha}} END { print "A maior linha tem " maior " caracteres na linha " linha}' texto.txt
```

3. Imprima apenas a linha mais longa
```bash
awk 'BEGIN {maior=""} {if (length($0) > maior) { maior=$0 }} END { print maior}' texto.txt
```

4. Imprima a liha atual se ela for mais longa que todas as linhas anteriores. Ou seja, cada vez que você encontra uma linha mais longa que todas as que você viu antes voĉe a imprime. Se a primeira linha for a mais longa de todas, você só imprime a primeira linha.
```bash
awk 'BEGIN {maior=0} {if (length($0) > maior) { maior=length($0); print $0 }}' texto.txt
```

5. Como você imprimiria todas as linhas do arquivo em ordem crescente de comprimento? Sugestão: utilize os recursos de composição do shell e os comandos sort e cut.

fazer
awk 'BEGIN {maior=0} {numeroLinha++; if (length($0) > maior) { maior=length($0); print $0 }}' texto.txt



