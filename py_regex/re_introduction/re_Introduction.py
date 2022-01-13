import re

def logging(tag: str):
    print(f'\n----- {tag} -----\n')

# re.search(pattern, string, flags=0)
def re_search():
    logging('re.search')
    sentence = 'world for An importAnt the random apple'

    if bool(re.search(r'world', sentence)):
        print(f'world is in {sentence}')

    # flag insensitive re.I or re.IGNORECASE
    words = [words for words in sentence.split(' ') if re.search(r'a', words, re.IGNORECASE)]
    print(words)

# re.sub(pattern, repl, string, count=0, flags=0)
def re_sub():
    logging('re.sub')

    word = 'This is my i3'
    print(word)

    word = re.sub(r'i3', 'Dynamic Window Manager', word)
    print(word)

# re.compile(pattern, flags=0)
def re_compile():
    logging('re.compile')

    find_dwm: re.Pattern = re.compile(r'dwm')

    sentence = 'this is my Dynamic Window Manager'
    print(bool(find_dwm.search(sentence)))

    sentence = 'this is my dwm'
    print(bool(find_dwm.search(sentence)))

    print(find_dwm.sub('DYNAMIC WINDOW MANAGER', sentence))

# Need raw bytes to search
def re_bytes():
    logging('bytes')

    sentence = b'this is my Dynamic Window Manager'
    print(bool(re.search(rb'Dynamic Window Manager', sentence)))

if __name__ == '__main__':
    re_search()
    re_sub()
    re_compile()
    re_bytes()


