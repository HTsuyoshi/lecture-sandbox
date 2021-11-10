# Tutorialzão python

## O que é o python

Estamos usando a versão do python3.

Python é um `interpretador`, ou seja ele vai executar os seguintes passos:

1. Ler seu código
2. Transformar em linguagem de máquina
3. Executar

O computador consegue entender apenas linguagem de máquina, mas não é prático para nós humanos escrever por exemplo: 10011010 (eu n faço ideia doq isso faz). Para facilitar usamos uma linguagem de programação `compilada` ou `interpretada` (Python é uma linguagem interpretada).

## O que é uma variável

Uma `variável` é uma parte que foi alocada (separada) da memória pra você poder guardar algo lá. Por exemplo: o nome do usuário, data de nascimento e coisa do tipo.

Uma `variável` pode ter o nome que você quiser desde que não seja uma `palavra reservada`.

## Tipos de variáveis

Os tipos de `variáveis` existentes no python são:

* `Inteiro (int)`: Um tipo de variável que guarda números sem vírgula.
    * Exemplo: 1, 10, -12, 22

* `Float (float)`: Um  tipo de variável que guarda números com vírgula.
    * Exemplo: 1.2, 12323212.11231, -2.4, 0.23423442341

* `Booleana (bool)`: Uma tipo de variável que guarda apenas verdadeiro ou falso
    * Exemplo: True, False

* `Caractere (char)`: Uma tipo de variável que guarda apenas uma letra.
    * No python não tem char, tudo é considerado uma string

* `String (str)`: Uma tipo de variável que guarda apenas uma letra.
    * Exemplo: "ola", "mundo", "    ", "$%#@ e"

## O que são palavras reservadas

`Palavras reservadas` são palavras que tem um efeito especial quando forem ser lidas pelo interpretador do python

#### Por que variáveis não podem ter o mesmo nome de `Palavras reservadas`? 

Imagine que você quer usar a `função` print() mas voce também tem uma `variável` print, como o `interpretador` vai saber se você quer usar uma `função` ou ver uma `variável`?

Isso mesmo ele não vai saber, por exemplo:

```python
print = 1
print('ola')
```
Vai dar erro, pois agora print não é uma função agora ele é uma variável

## Funções

O python tem algumas funções prontas, chamadas funções `built-in`, ou seja essas funções vão ter em qualquer python da mesma versão.

Essas funções podem ser vistas como funções matemáticas, elas recebem um `argumento`.

Um jeito fácil de reconhecer uma função é ver que ela tem parênteses no final

Exemplo:

* `print('ola')`: Neste caso a função é o `print` e o argumento é o `'ola'`

* `fazer_cafe('2', 'Litros')`: Neste caso a função é o `fazer_cafe` e os argumentos são as strings `'2'` e `'Litros'`

#### Algumas funções importantes

* `print()`: Essa função imprime algo na tela
    * Exemplo: print('olaaa'), print(23), print(0.2)

* `type()`: Essa função mostra o tipo de uma variável
    * Exemplo: type('string'), type(23), type(0.2)

* `input()`: Essa função recebe uma entrada do usuário
    * Exemplo: input('coloque um número aqui:'), input('coloque sua data de nascimento'), type(0.2)

## Controle de fluxo

Agora chegou a hora de vermos os famosos `if` e `else` statements.

Exemplo:
