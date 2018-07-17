def sort(list):
    for i in range(1,len(list)):
        while i != 0 and list[i] < list[i - 1]:
            list[i], list[i-1] = list[i-1], list[i]
            i -= 1
    return list
