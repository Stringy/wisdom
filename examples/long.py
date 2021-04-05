
def primes_slow(limit):
    count = 2
    result = 0
    while count < limit:
        top = count / 2
        n = 2
        while n < top:
            if count % n == 0:
                break
            n += 1
        if n > top:
            result += 1
        count += 1
    return result

print(f"found: {primes_slow(10000)} primes")