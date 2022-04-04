# Im lazy

from string import ascii_letters

CIPHER = [26, 20, 29, 22, 29]
TEXT = []
A = 17
B = 1

if __name__ == '__main__':
    for letter in CIPHER:
        TEXT.append(ascii_letters[(pow(A, -1, 30) * (letter - B)) % 30])
    print(TEXT)
