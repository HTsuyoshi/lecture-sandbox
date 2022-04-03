OPERATIONS = {'sum':'+','mul':'*'}
SUM = 'sum'
MUL = 'mul'

def nothing(w: int, h: int, N: int):
    return w

def sum(w: int, h: int, N: int):
    return (w + h) % N

def mul(w: int, h: int, N: int):
    return (w * h) % N

def print_row(op, h: int, N: int):
    for w in range(0, N):
        print(f'{op(w, h, N)}', end=' ')
    print()

def table(op: str, n: int):
    print(f'{OPERATIONS[op]} | ', end="")
    operation = nothing
    print_row(operation, 0, n)
    print('-'*(n*2 + 4))
    for column in range(0, n):
        print(f'{column} | ', end="")

        if op == 'sum':
            operation = sum
        elif op == 'mul':
            operation = mul

        print_row(operation, column, n)
    print()

if __name__ == '__main__':
    table(SUM, 4)
    table(SUM, 5)
    table(MUL, 5)
    table(SUM, 6)
    table(MUL, 6)
