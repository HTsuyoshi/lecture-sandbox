

# This file was *autogenerated* from the file a.sage
from sage.all_cmdline import *   # import sage library

_sage_const_2 = Integer(2); _sage_const_3 = Integer(3); _sage_const_9 = Integer(9); _sage_const_12 = Integer(12); _sage_const_6 = Integer(6); _sage_const_1 = Integer(1); _sage_const_4 = Integer(4)
x = var('x')
eq = _sage_const_2 *x**_sage_const_3  - _sage_const_9 *x**_sage_const_2  + _sage_const_12 *x
solutions = solve(diff(eq), x)
print(solutions)

x = var('x')
eq = e**(_sage_const_2 *x - _sage_const_6 ) - e
solutions = solve(diff(eq), x)
print(solutions)

x = var('x')
eq = e**x/(x-_sage_const_1 )
solutions = solve(diff(eq), x)
print(solutions)

x = var('x')
eq = sqrt(x**_sage_const_2  + _sage_const_4 )
solutions = solve(diff(eq), x)
print(solutions)

