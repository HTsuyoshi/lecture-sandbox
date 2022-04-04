# Authentication Vulnerabilities

## O que é autenticação?

Autenticação é o processo de verificar a identidade de um usuário ou cliente

Três tipos de autenticação:

- `Knowledge factors`: Alguma informação que o usuário tem acesso (Senha ou questão de segurança)
- `Possession factors`: Alguma coisa que o usuário têm (Objeto físico, celular ou token de segurança)
- `Inherence factors`: Alguma coisa que o usuário faz ou é (Digital ou comportamento)

## Qual a diferença entre autenticação e autorização?

Autenticação é o processo de verificar quem o usuário realmente é, a autorização envolve verificar se um usuário é permitido fazer determinada ação

## Como vulnerabilidades de autenticação ocorrem?

A maioria das vulnerabilidades de autenticação correm de duas formas:

- Eles são vulneráveis à ataques de força-bruta
- Erros de lógica ou implementação ruim (Também pode ser referir como `broken authentication`)

## Qual é o impacto da vulnerabilidade de autenticação?

O impacto pode ser severo. Uma vez que o *hacker* conseguiu acessar a conta de algum usuário, todos os dados e a funcionalidade da conta podem ser usados (Caso uma conta de alto privilégio seja acessada é possível controlar todo o aplicativo e até ganhar acesso na infra estrutura).

Uma conta de baixo-privilégio ainda pode dar problemas como: acesso à dados sensíveis, acessar páginas que antes não era autorizado (Essas páginas podem dar poder para o *hacker* que ele não tinha antes), etc...

## Vulnerabilidades em mecanismos de autenticação

### Vulnerabilidades logins baseados em senha

#### Ataques de força bruta

Um ataque de força bruta acontece quando um atacante usa um sistema de tentativa e erro para acertar credenciais válidas de algum usuário. Esses ataques são automatizados com wordlists de usuários e de senhas.

Sites que não possuem implementado uma proteção de força bruta suficientemente forte, estão bem vulnráveis tendo em vista a eficiência desses ataques.

#### Força bruta nos nomes de usuários

Nomes de usuário são fáceis de adivinhar se eles seguirem um padrão, como por exemplo um email. Mesmo que não exista um padrão, contas com privilégios altos também são criadas com nomes fáceis de adivinhar como `administrator` ou `admin`.

Durante um teste verifique se o site revela potenciais nomes de usuário. Por exemplo: é possível acessar os perfis sem estar autenticado? Mesmo que o conteúdo do perfil esteja escondido, ainda é possíel que o nome do perfil seja o mesmo nome para a autenticação.

#### Força bruta nas senhas

Senhas podem ser alvos de brute force, dependendo da força da senha. Muitos sites adotam alguma política de senha, alguns exemplos são: entropia-alta (Que são, teoricamente, mais difíceis de quebrar), usar caracteres especiais

#### Enumeração de usuários

A enumeraçãode usuários acontece quando um atacante consegue observar mudanças no comportamento do site, assim conseguindo identificar quais usuários são válidos.

Geralmente ocorre em uma tela de login, um exemplo é quando o usuário consegue acertar a o usuário mas erra a senha. Assim o atacante consegue criar uma rápida lista de possíveis usuários.

Quando tentar usar Força bruta em uma página, alguns tópicos devem ser levados em consideração:

- `Status codes`: É bem provável que o código do HTTP retornado seja bem parecido, mas quando uma mudança ocorrer é uma forte indicaçao de que o nome de usuário está correto. (É uma boa prática a aplicação sempre retornar o mesmo código de status)

- `Error messages`: Algumas vezes a mensagem de erro é diferente dependendo se o usuário ou a senha estão certos (Ex: Usuário correto e senha incorreta). É uma boa pratica a aplicação retornar o mesmo erro independente se o usuário conseguiu acertar um campo ou não.

- `Response times`: Se, por exemplo, um aplicação checar se a senha está correta apenas depois de checar se um usuário está correto esse pequeno passo pode aumentar o tempo de resposta. Um atacante pode por exemplo usar uma senha longa, assim a aplicação vai demorar mais para verificar a senha.

#### Flawed brute-force protection

É muito provável que um ataque de força bruta envolva muitas tentativas falhas antes do atacante conseguir comprometer uma conta. Uma proteção contra força bruta tenta diminuir a quantidade de vezes que alguém pode tentar logar.

Duas formas comuns para isso são:

- Bloquear a conta que o usuário está tentanc conectar se muitas tentativas de login forem feitas

- Bloquear o IP do usuário se muitas tentativas de login forem feitas em pouco tempo

As duas soluções oferecem uma certa quantidade de proteção, mas nenhuma é invulnerável.

Um exemplo é o bloqueio do IP resetar quando um usuário conseguir logar, caso isso aconteça o atacante consegue logar em sua conta antesde tentar outra senha/usuário.






