from itertools import cycle

if __name__ == '__main__':
    # 2.1
    def encrypt(word: str, key: str):
        def one_letter(p: tuple[str, str]):
            return chr(abs(((ord(p[0]) - 2*ord('a')) + ord(p[1])) % 26) + ord('a'))
        return ''.join([*map(one_letter, [*zip(word, cycle(key))])])

    def decrypt(word: str, key: str):
        def one_letter(p: tuple[str, str]):
            return chr(abs(((ord(p[0]) - ord('a')) - (ord(p[1]) - ord('a'))) % 26) + ord('a'))
        return ''.join([*map(one_letter, [*zip(word, cycle(key))])])

    plain_text = 'bsaspp kkuosp'
    key = 'rsidpy dkawoa'

    word1 = 'bsaspp'
    word2 = 'kkuosp'

    key1 = 'rsidpy'
    key2 = 'dkawoa'

    #print(decrypt(plain_text, key))
    print(decrypt(word1, key1))
    print(decrypt(word2, key2))
    print('stabbing')
