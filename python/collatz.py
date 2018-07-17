def collatz(number):
    if number == 1:
        return 1
    if number % 2 == 0:
        return collatz(int(number/2))
    return collatz(number*3 + 1)

assert collatz(100) == 1
assert collatz(10000) == 1
assert collatz(124234) == 1
