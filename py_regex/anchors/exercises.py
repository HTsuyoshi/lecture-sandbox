import re

def exercise_1():
    line1 = 'be nice'
    line2 = '"best!"'
    line3 = 'better?'
    line4 = 'oh no\nbear spotted'

    pat = re.compile(r'\Abe')

    match: bool

    match = bool(pat.search(line1))
    assert (match == True)

    match = bool(pat.search(line2))
    assert (match == False)

    match = bool(pat.search(line3))
    assert (match == True)

    match = bool(pat.search(line4))
    assert (match == False)

def exercise_2():
    words = 'bred red spread credible'
    assert (re.sub(r'\bred\b', 'brown', words) == 'bred brown spread credible')

def exercise_3():
    words = ['hi42bye', 'nice1423', 'bad42', 'cool_42a', 'fake4b']
    assert ([w for w in words if re.search(r'\B42\B', w)] == ['hi42bye', 'nice1423', 'cool_42a'])

def exercise_4():
    items = ['lovely', '1\ndentist', '2 lonely', 'eden', 'fly\n', 'dent']
    assert ([e for e in items if re.search(r'\Aden', e) or re.search(r'ly\Z', e)] == ['lovely', '2 lonely', 'dent'])

def exercise_5():
    para = '''ball fall wall tall
mall call ball pall
wall mall ball fall
mallet wallet malls'''
    print(re.sub(r'^mall', '1234', para, flags=re.M))

def exercise_6():
    # WTH I need to do here
    #items = ['12\nthree\n', '12\nThree', '12\nthree\n4', '12\nthree']
    #assert ([w for w in items if re.search(r'12\nthree', w, flags=re.I)] == ['12\nThree', '12\nthree'])
    pass

def exercise_7():
    items = ['handed', 'hand', 'handy', 'unhanded', 'handle', 'hand-2']
    assert ([re.sub(r'\bhand\B', 'X', e) for e in items] == ['Xed', 'hand', 'Xy', 'unhanded', 'Xle', 'hand-2'])

def exercise_8():
    # TODO

if __name__ == '__main__':
    exercise_1()
    exercise_2()
    exercise_3()
    exercise_4()
    exercise_5()
    exercise_6()
    exercise_7()
