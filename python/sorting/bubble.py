def sort(list):
    swapped = True
    while swapped:
        swapped = False
        for i in range(1,len(list)):
            if list[i] < list[i-1]:
                list[i], list[i-1] = list[i-1], list[i]
                swapped = True
    return list

