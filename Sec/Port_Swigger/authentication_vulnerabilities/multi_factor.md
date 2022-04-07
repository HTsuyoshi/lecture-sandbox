# Vulnerabilidades logins baseados em autenticação multi-fator

2FA está se tornando cada vez mais utilizados nos websites baseado em alguma coisa que você conhece ou que você tem. Geralmente requer que o usuário entre a senha tradcional e um código de verificação que provém de um dispositivo físico.

Enquanto é possível conseguir uma senha em um `single knowledge-factor`, conseguir simultâneamente o código do outro fator é menos provável. Mas um `2FA` mal implementada não serve de ajuda nenhuma.

Também não faz sentido os benefícios da autenticação de multi-fator se eles não estão verificando fatores diferentes. Um `2FA` baseado em email é um desses exemplos, mesmo que o usuário envie a senha e o código multi-fator acessar o código multi-fator está baseada apenas em saber as credenciais da senha do email.

### Tokens de autenticação de 2 fatores

Os códigos de verificação são geralmente lidos pelo usuário por um dispositivo físico. Muitos sites com alta segurança, agora, provém dispositivos  para esse propósito, como um token RSA ou um dispositivo para acessar a conta no banco ou trabalhar no notebook. Para ter mais segurança esses dispositivos específicos tem a vangatem de gerar diretamente esses códigos.também é comum para sites usar um aplicativo dedicado.

Por outro lado, alguns sites enviam códigos para o celular do usuário como mensagem de texto. Isso ainda pode ser verificar algo que você tem, mas é aberto para ser explorado. Primeiramente, o código é transmitido por `SMS` ao invés de ser gerado pelo próprio dispositivo. Isso cria um potencial para o código ser interceptado. Também existe o risco de `SIM swapping`, onde o atacante, de forma fraudulenta, consegue um `SIM` card com o telefona da vítima. Então o atacante iria receber todos os `SMS` enviados para a vítima, incluindo o código de verificação.

### Bypassing two-factor authentication

Algumas vezes a implementação do `2FA` tem falhas que podem ser inutilizar totalmente o `2FA`.

Se um usuário coloca uma senha, e então entra um código de verificação em uma página separada, o usuário está efetivamente *logado* antes da verificação `2FA`. Nesse caso vale a pena testar se você consegue voce consegue apenas *logar* e passar pela autenticação. Ocasionalmente você vai descobrir que o site não checa se o usuário completou a segunda etapa da verificação antes de carregar a página.

### Flawed two-factor verification logic

Algumas vezes o `2FA` tem uma lógica falha, ou seja depois do site ter completado a etapa inicial de login ele não verifica que o mesmo usuário está completando a segunda etapa

Por exemplo o usuário pode enviar suas credenciais de login:

```
username=carlos&password=qwerty
```

Depois um cookie é enviado para o usuário:

```
Cookie: account=carlos
```

Então o site usa o cookie para determinar qual conta o usuário está tentando acessar:

```
Cookie: account=carlos
...
verification-code=123456
```

Nesse caso o atacante conseguiria logar na sua própria conta e trocar o cookie para o cookie de algum outro usuário.


```
Cookie: account=victim-user
...
verification-code=123456
```

Isso é extremamente perigoso se o atacante conseguir usar força bruta para acertar o código de `2FA`. Então não precisariam de uma senha, apenas do usuário.

### Ataque de força bruta em códigos de verificação de `2FA`

Com senhas, sites precisam de formas para prevenir ataques de força bruta em códigos de verificação `2FA`. Isso é essencialmente importante pois geralmente o código é simples de 4 ou 5 digitos. Sem uma proteção adequada de brute-force, quebrar esse código é trivial.

Alguns sites, para previnir isso, automaticamente desconectam o usuário se um certo número de erros for atingido. Isso não é efetivo na prática pois um atacante avançado pode até automatizar essas etapas usando macros no Burp Intruder. O Turbo Intruder também pode ser usado para esse propósito
