import random
import bubble
import insertion

for x in [bubble, insertion]:
    numbers = [random.randint(0,1000) for x in range(1000)]
    copy = numbers.copy()
    x.sort(numbers)
    copy.sort()
    assert numbers == copy


