# Vulnerabilidades logins baseados em senha

### Ataques de força bruta

Um ataque de força bruta acontece quando um atacante usa um sistema de tentativa e erro para acertar credenciais válidas de algum usuário. Esses ataques são automatizados com wordlists de usuários e de senhas.

Sites que não possuem implementado uma proteção de força bruta suficientemente forte, estão bem vulnráveis tendo em vista a eficiência desses ataques.

### Força bruta nos nomes de usuários

Nomes de usuário são fáceis de adivinhar se eles seguirem um padrão, como por exemplo um email. Mesmo que não exista um padrão, contas com privilégios altos também são criadas com nomes fáceis de adivinhar como `administrator` ou `admin`.

Durante um teste verifique se o site revela potenciais nomes de usuário. Por exemplo: é possível acessar os perfis sem estar autenticado? Mesmo que o conteúdo do perfil esteja escondido, ainda é possíel que o nome do perfil seja o mesmo nome para a autenticação.

### Força bruta nas senhas

Senhas podem ser alvos de brute force, dependendo da força da senha. Muitos sites adotam alguma política de senha, alguns exemplos são: entropia-alta (Que são, teoricamente, mais difíceis de quebrar), usar caracteres especiais

### Enumeração de usuários

A enumeraçãode usuários acontece quando um atacante consegue observar mudanças no comportamento do site, assim conseguindo identificar quais usuários são válidos.

Geralmente ocorre em uma tela de login, um exemplo é quando o usuário consegue acertar a o usuário mas erra a senha. Assim o atacante consegue criar uma rápida lista de possíveis usuários.

Quando tentar usar Força bruta em uma página, alguns tópicos devem ser levados em consideração:

- `Status codes`: É bem provável que o código do HTTP retornado seja bem parecido, mas quando uma mudança ocorrer é uma forte indicaçao de que o nome de usuário está correto. (É uma boa prática a aplicação sempre retornar o mesmo código de status)

- `Error messages`: Algumas vezes a mensagem de erro é diferente dependendo se o usuário ou a senha estão certos (Ex: Usuário correto e senha incorreta). É uma boa pratica a aplicação retornar o mesmo erro independente se o usuário conseguiu acertar um campo ou não.

- `Response times`: Se, por exemplo, um aplicação checar se a senha está correta apenas depois de checar se um usuário está correto esse pequeno passo pode aumentar o tempo de resposta. Um atacante pode por exemplo usar uma senha longa, assim a aplicação vai demorar mais para verificar a senha.

### Flawed brute-force protection

É muito provável que um ataque de força bruta envolva muitas tentativas falhas antes do atacante conseguir comprometer uma conta. Uma proteção contra força bruta tenta diminuir a quantidade de vezes que alguém pode tentar logar.

Duas formas comuns para isso são:

- Bloquear a conta que o usuário está tentanc conectar se muitas tentativas de login forem feitas

- Bloquear o IP do usuário se muitas tentativas de login forem feitas em pouco tempo

As duas soluções oferecem uma certa quantidade de proteção, mas nenhuma é invulnerável.

Um exemplo é o bloqueio do IP resetar quando um usuário conseguir logar, caso isso aconteça o atacante consegue logar em sua conta antesde tentar outra senha/usuário.

### Bloqueio de conta

Um método que o website tenta prevenir o ataque de força bruta é quando um certo critério acontece, geralmente a quantidade de erros em um login. Apenas com isso é possível ver que a conta fo bloqueada, assim o atacante pode enumerar os usuários.

Mas é um método falho quando o atacante quer ganhar acesso à uma conta aleatória.

Exemplo:

1. Estabeleça uma lista de usuários vélidos (Pode ser feito por enumeração ou se basear em alguma lista de usuários comuns)

2. Escolha uma lista de senhas de no máximo a quantidade de tentativas para bloquear a conta (Se uma conta for bloqueada em 4 tentativas, fazer uma lista de 3 senhas)

3. Usando uma ferramenta como Burp Intruder, tente todas as senhas para cada usuário. Dessa forma é possível tentar entrar em todas as contas usando as 3 senhas.

Também falha em proteger contra `Credential Stuffing` (Juntar dados de pessoas baseados em vazamentos de bancos de dados que aconteceram anteriormente), `Credential Stuffing` é perigoso contra esse método pois o atacante pode acabar conseguindo acesso à várias contas em apenas um ataque automatizado.

### User rate limiting

Outra forma para evitar ataques de força bruta é limitando o user, fazer muitos requests em um período muito curto de tempo faz com que seu IP seja bloqueado. Normalmente, o IP só pode ser desbloqueado das seguintes maneiras:

- Automaticamente depois que um certo período de tempo passou
- Manualmente pelo administrador
- Manualmente pelo usuário depois de complear um CAPTCHA

`User rate limiting` é as vezes referenciados como `account locking` sendo menos propício à enumeração de usuário e DoS. Mas não é completamente seguro, já que existem muitas formas de manipular o IP.

Como o limite é baseado na quantidade de requests HTTP feitas pelo usuário, é possível passar por essa defesa tentando muitas senhas ao mesmo tempo em um único request.

### HTTP basic authentication

Mesmo sendo antigo, por ser relativamente simples e fácil de implementação é provável que algumas vezes ele vai ser usado. O cliente recebe um token de autenticação do server, o token é construido concatenando o usuário e a senha e codificando em base64. Esse token é armazenado e gerenciado pelo navegador, que automaticamente adiciona o header `Authorization` para os próximos requests.

Não é considerado um método seguro de autenticação. Primeiro ele envolve o usuário enviar suas credenciais em TODAS as requests. A não ser que a aplicação implemente HSTS, as credenciais do usuário estão abertas para ser capturadas pelo ataque `man-in-the-middle`.

Geralmente ele também não implementa proteção de força bruta, e o token é feito de valores estáticos, isso pode levar à uma vulnerabilidade que pode ser atacada por ataques de força bruta.

Além disso ele é vulnerável particularmente por vulnerabilidades relacionados à sessão, um exemplo é o CSRF.

Em alguns casos o `HTTP basic authentication` não revela nada muito interessante, mas as senhas podem ter sido reutilizadas.
