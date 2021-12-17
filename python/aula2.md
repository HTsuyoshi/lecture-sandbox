# Tutorialzão python

## Operadores

Acho que você já teve contato com esses operadores:

```
+, -, *, %, //, !
```

* +: `soma`
* -: `subtração`
* !: `negação` No python acho que não tem, eles usam o `not` ao invés do `!`
* \*: `multiplicação`
* \*\*: `exṕonenciação`
* //: `divisão inteira`
* /: `divisão`
* %: `resto`

Esses operadores fazem uma coisa para cada tipo de variável

#### SOMA

Se eu tentar somar dois `Inteiros` vamos ter o seguinte resultado:

Soma: `1 + 2`

Resultado: `3`

Mas a soma pode fazer coisas diferentes dependendo do tipo de variável que você está usando:

Se eu tentar somar duas `Strings` vamos ter o seguinte resultado:

Soma: `'oi' + 'como voce ta'`

Resultado: `'oicomo voce ta`

Como descobrimos hoje `strings` e `inteiros` não podem ser somados diretamente.

Mas `Float` e `Inteiros` podem ser somados "diretamente"

```
Na verdade na maior parte das linguagens a variável do tipo `Float` é maior que a variável do tipo `Inteiro` então se transformarmos um `Int` em um `Float` não vamos perder informação (Pois o `Inteiro` é menor que o `Float`), mas se colocarmos um `Float` dentro de um `Int` vamos perder informação

Se transformar 1,5231 em 1, vamos perder toda informação depois da vírgula
```

Para o python somar um `Float` com um `Int` ele resulta em um `Float`, pois se ele devolvesse um `Int` acabaria perdendo a informação depois da vírgula:

Se eu tentar somar um `Float` com um `Int` vamos ter o seguinte resultado:

Soma: `1.234 + 1`

Resultado: `2.234`

Existe uma coisa legal que você pode tentar também:

#### MULTIPLICAÇÃO

Se tentar multiplicar dois `Inteiros` vamos

Soma: `1 * 2`

Resultado: `2`

Se tentar multiplicar um `Inteiro` com uma `String`

```python
'oi'*10
```

Vai resultar em:

```python
'oioioioioioioioioioi'
```

É bom testar os outros operadores também!

```
Diferença entre / e //.

Fazer 2/3 vai resultar em 0.6666666666666666 (um float)

Fazer 2//3 vai resultar em 0 (um inteiro)

(Podemos perceber que ele fez a divisão e transformou em Inteiro (Retirou a informação depois da vírgula))
```

#### Exemplo

Por fim vou mostrar alguns exemplos de código que você poderá executar:

```python
print('-'*20)
print('Meu super título')
print('-'*20)
```

No exemplo abaixo podemos ver que conseguimos imprimir um `Float` e uma `String` usando uma vírgula ao invés de um +

```python
divisao = 2/3
print('2/3 =', divisao)
```

## Controle de fluxo

Agora chegou a hora de vermos os famosos `if`, `else` e `elif` statements.

Exemplo:

Recomendação de Exercícios:

* [1003](https://www.beecrowd.com.br/judge/pt/problems/view/1003)
* [1004](https://www.beecrowd.com.br/judge/pt/problems/view/1004)
* [1007](https://www.beecrowd.com.br/judge/pt/problems/view/1007)

Meus exercícios:

* Crie um programa que recebe um número e ele mostra se o número é PAR ou ÍMPAR e POSTIVO, NEGATIVO ou NULO
Por exemplo, se eu enviar 20 ele tenque imprimir POSITIVO PAR
Se eu enviar 0 ele tenque imprimir NULO PAR
Se eu enviar -1 ele tenque imprimir NEGATIVO ÍMPAR

Solução:

1003:
```python
A = int(input())
B = int(input())
S = A + B
print('SOMA =', S)
```
1004:
```python
A = int(input())
B = int(input())
P = A * B
print('PROD =', P)
```
1007:
```python
A = int(input())
B = int(input())
C = int(input())
D = int(input())
P = (A * B - C * D)
print('DIFERENCA =', P)
```

Meu ex:

```python
numero = int(input())
imprimir = ""

if numero == 0:
	imprimir = imprimir + "NULO"
elif numero > 0:
	imprimir = imprimir + "POSITIVO"
elif numero < 0:
	imprimir = imprimir + "NEGATIVO"

if numero % 2 != 0:
	imprimir = imprimir + " ÍMPAR"
else:
	imprimir = imprimir + " PAR"

print(imprimir)
```