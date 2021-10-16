
x = var('x')
eq = 2*x^3 - 9*x^2 + 12*x
solutions = solve(diff(eq), x)
print(solutions)

x = var('x')
eq = e^(2*x - 6) - e
solutions = solve(diff(eq), x)
print(solutions)

x = var('x')
eq = e^x/(x-1)
solutions = solve(diff(eq), x)
print(solutions)

x = var('x')
eq = sqrt(x^2 + 4)
solutions = solve(diff(eq), x)
print(solutions)
