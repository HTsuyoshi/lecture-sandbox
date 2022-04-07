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

[Vulnerabilidades baseadas em login](./login.md)
[Vulnerabilidades baseadas em multi-fator](./multi_factor.md)
[Vulnerabilidades baseadas em outros tipos de autenticação](./other.md)
