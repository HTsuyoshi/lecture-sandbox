TEXT = 'xultpaajcxitltlxaarpjhtiwtgxktghidhipxciwtvgtpilpitghlxiwiwtxgqadds.'.lower()

from string import ascii_lowercase
from itertools import cycle

def shift(letter: str, n: int):
    if letter in ascii_lowercase:
        return chr(((ord(letter) - ord('a') + n) % 26) + ord('a'))
    return letter

if __name__ == '__main__':
    for n in range(26):
        print(f'Shift {n}: ', end='')
        print(''.join([*map(shift, TEXT, cycle([n,n]))]))

    print()
    N = 11
    print(''.join([*map(shift, TEXT, cycle([N]))]))
