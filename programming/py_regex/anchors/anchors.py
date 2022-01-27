import re

# start - \A, end - \Z
def match_start_end():
    match: bool

    # Start
    match = bool(re.search(r'\Acat', 'cater'))
    assert (match == True)

    match = bool(re.search(r'\Acat', 'concatenation'))
    assert (match == False)

    # End
    match = bool(re.search(r'are\Z', 'spare'))
    assert (match == True)

    match = bool(re.search(r'are\Z', 'nearest'))
    assert (match == False)

    # Appending text
    assert (re.sub(r'\Z', 'er', 'cat') == 'cater')

# re.fullmatch(pattern, string, flags=0)
def re_fullmatch():
    word_pat = re.compile(r'\Acat\Z')

    assert (bool(word_pat.search('cat')) == True)
    assert (bool(word_pat.search('concatenation')) == False)

def line_anchors():
    # ^ - Start of line, $ - End of line
    # If no \n they will behave the same as \A and \Z
    match: bool

    greeting = 'hi there\nhave a nice day\n'

    match = bool(re.search(r'day$', greeting))
    assert (match == True)

    match = bool(re.search(r'day\n$', greeting))
    assert (match == True)

    match = bool(re.search(r'day\Z', greeting))
    assert (match == False)

    match = bool(re.search(r'day\n\Z', greeting))
    assert (match == True)

    # re.M or re.MULTILINE

    # check if any line in the string starts with 'top'
    match = bool(re.search(r'ˆtop', 'hi hello\ntop spot', flags=re.M))
    assert (match == True)

    # check if any line in the string ends with 'ar'
    match = bool(re.search(r'ar$', 'spare\npar\ndare', flags=re.M))
    assert (match == True)

def word_anchors():
    # \b - Start/End of word
    # \B is the \b complement

    words = 'par spar apparent spare part'

    # replace 'par' only at start of word
    assert (re.sub(r'\bpar', 'X', words) == 'X spar apparent spare Xt')

    # replace 'par' only at end of word
    assert (re.sub(r'par\b', 'X', words) == 'X spar apparent spare Xt')

    # replace word 'par'
    assert (re.sub(r'\bpar\b', 'X', words) == 'X spar apparent spare Xt')

    # put space in every \b
    assert (re.sub(r'\b', ' ', 'foo_baz=num1+35*42/num2').strip() == 'foo_baz = num1 + 35 * 42 / num2')

    # put X in every par inside a word
    assert (re.sub(r'\Bpar', 'X', words) == 'par sX apXent sXe part')

if __name__ == '__main__':
    # Metacharacters = character with special meaning
    match_start_end()
    re_fullmatch()
    line_anchors()
    word_anchors()
