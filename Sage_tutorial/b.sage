x=var('x')
eq=x^4 * (x-2)*(x+3)
solutions=find_local_maximum(eq)
print(solutions)
