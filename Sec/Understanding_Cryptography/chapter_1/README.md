# Exercises

## 1

#### 1.1

[Python Script](./letter_frequency.py)

#### 1.2

The text found is:

```
because the practice of the basic movements of kata is
the focus and mastery of self is the essence of
matsubayashi ryu karate do i shall try to elucidate the
movements of the kata according to my interpretation
based on forty years of study

it is not an easy task to explain each movement and its
significance and some must remain unexplained to give a
complete explanation one would have to be qualified and
inspired to such an extent that he could reach the state
of enlightened mind capable of recognizing soundless
sound and shapeless shape i do not deem myself the final
authority but my experience with kata has left no doubt
that the following is the proper application and
interpretation i offer my theories in the hope that the
essence of okinawan karate will remain intact
```

Using this [website](https://crypto.interactive-maths.com/frequency-analysis-breaking-the-code.html)

![website](./text_1.png)

#### 1.3

Shoshin Nagamine wrote it.

## 2

#### 2.1

[Brute Force](./shift_text.py)

#### 2.2

Tecumseh wrote it.

## 3

#### 3.1

Information:

```
Budget: 1 million
ASIC: 5.10⁸ keys/sec
ASIC cost: 50

Age of Universe: 10¹⁰
```

Answer:

```
2¹²⁸ = TimetoBreak*5*10⁸*(1,000,000/50)
2¹²⁸ = TimetoBreak*5*10⁸*(20,000)
2¹²⁸ = TimetoBreak*5*10⁸*(20,000)
TimetoBreak = 2¹²⁸/(5*10⁸*(20,000))
TimetoBreak = 2¹²⁸/(2*5*10¹²)
TimetoBreak = 2¹²⁸/(2*5*(5*2)¹²)
TimetoBreak = 2¹²⁸/(2¹³*5¹³)
TimetoBreak = 2¹¹⁵/5¹³
TimetoBreak = 2¹¹⁵/4¹³
TimetoBreak = 2¹¹⁵/(2²)¹³
TimetoBreak = 2¹¹⁵/2²⁶
TimetoBreak = 2⁸⁹ seconds
TimetoBreak = 2⁸⁹/(60*60*24*365) years
TimetoBreak = 2⁸⁹/(2⁷*3³*5³*73) years
TimetoBreak = 2⁸²/(3³*5³*73) years
TimetoBreak = 2⁸²/(2³*4³*64) years
TimetoBreak = 2⁶⁷ years
```

About six times the age of the universe

#### 3.2

Starting from this point of the `3.2` question

```
TimetoBreak = 2⁸⁹/(60*60*24) days
TimetoBreak = 2⁸⁹/(2⁷*3³*5²) days
TimetoBreak = 2⁸⁹/(2⁷*2³*4²) days
TimetoBreak = 2⁸⁹/(2⁷*2³*2⁴) days
TimetoBreak = 2⁸⁹/2¹⁴ days
TimetoBreak = 2⁷⁵ days
```

To break in 24 hours we would need about `2⁷⁵*18` months to get the computer power to break

## 4

#### 4.1

Given a password of 8 letters and each character can be represented  by 7 bits (128 possibilities), the keyspace which can be constructed by such passwords is `#k = 128⁸`

#### 4.2

In bits the corresponding keylength is `#k = (2⁷)⁸ = 2⁵⁶`

#### 4.3

In this case the key length is  about `#k = 26⁸ = (2⁵)⁸`

#### 4.4

#### 4.4.a

```
#k = 2¹²⁸
#k = (2⁷)^x

2¹²⁸ = (2⁷)^x
x = 19
```

To get 128 bits using characters of 7 bits its needed a password of about 19 characters

#### 4.4.b

```
#k = 2¹²⁸
#k = (2⁵)^x

2¹²⁸ = (2⁵)^x
x = 26
```

To get 128 bits using lowercase characters needed a password of about 26 characters

#### 5

#### 5.1

```
15*29 mod 13
15*3 mod 13
2*3 mod 13
6 mod 13
```

#### 5.2

```
2*29 mod 13
2*3 mod 13
6 mod 13
```

#### 5.3

```
2*3 mod 13
6 mod 13
```

#### 5.4

```
-11*3 mod 13
2*3 mod 13
6 mod 13
```

#### 5.5

Taking the example `5.2` the number `2*29` can we represented like this:

```
29 + 29
(13*2) + 3 + (13*2) + 3
(13*4) + 6
```

As we can see its very similar to the division formula `d = d*q + r` and `6` is the remainder

## 6

#### 6.1

```
1/5 mod 13
1*5⁻¹ mod 13
1*8 mod 13
8 mod 13
```

#### 6.2

```
1*5⁻¹ mod 7
1*3 mod 7
3 mod 7
```

#### 6.3

```
3*2/5 mod 7
3*2*3 mod 7
4 mod 7
```

## 7

#### 7.1, 7.2, 7.3

[Python Script](./construct_table.py)

#### 7.4

## 8

```
Z11 = 5⁻¹ = 9
Z12 = 5⁻¹ = 5
Z13 = 5⁻¹ = 8
```

## 9

## 9.1

```
3² mod 13
9 mod 13
```

## 9.2

```
7² mod 13
10 mod 13
```

## 9.3

```
3¹⁰ mod 13
3 mod 13
```

## 9.4

```
7¹⁰⁰ mod 13
10⁵⁰ mod 13
9²⁵ mod 13
9²⁴ * 9 mod 13
1 * 9 mod 13
9 mod 13
```

## 9.5

```
7^x = 11 mod 13
7^2 = 10 mod 13
7^5 = 11 mod 13
```

## 10

[Python Script](./relatively_prime.py)

## 11

## 11.1

[Python Script](./affine_cipher.py)

## 11.2

Lewis Carroll wrote it

## 12

## 12.1

Encryption:

```
E = a * x + b mod 30
```

Decryption:

```
E = (x - b) * a⁻¹ mod 30
```

## 12.2

The keyspace of this alphabet is `30^c`, and c in this equation is the quantity of characters

## 12.3

[Python Script](./german_affine_cipher.py)

## 12.4

FRODO?

## 13

???

```
c1 = x1 * p + y1 mod 26
c2 = x2 * p + y2 mod 26
```

## 14

## 14.1

```
e1 = a1*x + b1 mod 26
e2 = a2*x + b2 mod 26
e3 = e2(e1(x))
e3 = e2(a1*x + b1 mod 26))
e3 = a2*(a1*x + b1) + b2 mod 26
e3 = a2*a1*x + a2*b1 + b2 mod 26

a3 = a2*a1 mod 26
b3 = a2*b1 + b2 mod 26
```

## 14.2

Given that:

```
a1 = 3
b1 = 5
a2 = 11
b2 = 7
```
The values of `a3` and `b3` are:

```
a3 = 11*3 mod 26
a3 = 33 mod 26
a3 = 7 mod 26

b3 = 11*5 + 7 mod 26
b3 = 55 + 7 mod 26
b3 = 62 mod 26
b3 = 10 mod 26
```

## 14.3

Encrypting `K = 10` with `e1`:

```
C1 = 3 * 10 + 5 mod 26
C1 = 35 mod 26
C1 = 9 mod 26
```

Encrypting `e1(K)` with `e2`:

```
C2 = 11 * 9 + 7 mod 26
C2 = 106 mod 26
C2 = 4 mod 26
```

Encrypting `K` with `e3`:

```
C3 = 7 * 10 + 10 mod 26
C3 = 80 mod 26
C3 = 4 mod 26
```

## 14.4

If an exaustie key-search is applied to a double-encrypted affine ciphertext the key space interval remains the same, so encrypting two times doesn't make any difference to the attacker
