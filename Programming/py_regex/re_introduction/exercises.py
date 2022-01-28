import re

def exercise_1():
    # Check whether the given strings contain 0xB0
    line1 = 'start address: 0xA0, func1 address: 0xC0'
    line2 = 'end address: 0xFF, func2 address: 0xB0'
    assert (bool(re.search(r'0xB0', line1)) == False)
    assert (bool(re.search(r'0xB0', line2)) == True)

def exercise_2_3():
    # Replace all occurrences of 5 with five for the given string.
    ip = 'They ate 5 apples and 5 oranges'
    assert (re.sub(r'5', 'five', ip) == 'They ate five apples and five oranges')

    # Replace first occurrence of 5 with five for the given string.
    assert (re.sub(r'5', 'five', ip, count=1) == 'They ate five apples and 5 oranges')

def exercise_4():
    # For the given list, filter all elements that do not contain e 
    items = ['goal', 'new', 'user', 'sit', 'eat', 'dinner']
    assert ([w for w in items if not re.search(r'e', w)] == ['goal', 'sit'])

def exercise_5():
    ip = 'This note should not be NoTeD'
    assert (re.sub(r'note', 'X', ip, flags=re.I) == 'This X should not be XD')

def exercise_6():
    ip = b'tiger imp goat'
    assert (bool(re.search(rb'at', ip)) == True)

def exercise_7():
    para = '''good start
    Start working on that
    project you always wanted
    stars are shining brightly
    hi there
    start and try to
    finish the book
    bye'''
    pat = re.compile(r'start', flags=re.I)
    for line in para.split('\n'):
        if not pat.search(line):
            print(line)

def exercise_7_8():
    items = ['goal', 'new', 'user', 'sit', 'eat', 'dinner']
    assert ([w for w in items if re.search(r'w', w) or re.search(r'a', w)] == ['goal', 'new', 'eat'])

    items = ['goal', 'new', 'user', 'sit', 'eat', 'dinner']
    assert ([w for w in items if re.search(r'e', w) and re.search(r'n', w)] == ['new', 'dinner'])

def exercise_9():
    ip = 'start address: 0xA0, func1 address: 0xC0'
    assert (re.sub(r'0xC0', r'0x1F', re.sub(r'0xA0', r'0x7F', ip)) == 'start address: 0x7F, func1 address: 0x1F')

if __name__ == '__main__':
    exercise_1()
    exercise_2_3()
    exercise_4()
    exercise_5()
    exercise_6()
    exercise_7()
    exercise_9()
