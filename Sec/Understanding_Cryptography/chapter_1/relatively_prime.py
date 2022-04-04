def is_rel_prime(n: int):
    primes = []
    for k in range(1, n):
        if n % k != 0:
            primes.append(k)
    return primes

if __name__ == '__main__':
    print(is_rel_prime(4))
    print(is_rel_prime(5))
    print(is_rel_prime(9))
    print(is_rel_prime(26))

