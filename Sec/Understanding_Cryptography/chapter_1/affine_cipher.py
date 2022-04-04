TEXT = 'falszztysyjzyjkywjrztyjztyynaryjkyswarztyegyyj'.lower()

def affine(letter: str, a: int, b: int):
    return chr((((ord(letter) - ord('a')) * a + b) % 26) + ord('a'))

def decrypt_affine(letter: str, a: int, b: int):
    return chr((((ord(letter) - ord('a') - b) * pow(a, -1, 26)) % 26) + ord('a'))

from itertools import cycle

if __name__ == '__main__':
    plain_text = ''.join([*map(affine, TEXT, cycle([7, 7]), cycle([22, 22]))])
    plain_text = ''.join([*map(decrypt_affine, TEXT, cycle([7, 7]), cycle([22, 22]))])
    print(plain_text)
