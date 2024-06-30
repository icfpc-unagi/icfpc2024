# def main(v4):
#     if v4 > 2:
#         def f(v6, v7):
#             if v6 == v4:
#                 return v7
#             if main(v6 + 1) > v6 - 1:
#                 if v4 % v6 == 0:
#                     y = (v7 / main(v6)) * (main(v6) - 1)
#                 else:
#                     y = v7
#             else:
#                 y = v7
#             return f(v6 + 1, y)
#         x = f(2, v4)
#     else:
#         x = v4
#     return min(v4, 1 + x)


import functools

@functools.cache
def main(v4):
    if v4 > 2:
        def f(v6, v7):
            # if v6 == v4:
            #     return v7
            # if v4 % v6 == 0 and main(v6 + 1) > v6 - 1:
            #     tmp = main(v6)
            #     y = (v7 // tmp) * (tmp - 1)
            # else:
            #     y = v7
            # return f(v6 + 1, y)
            while True:
                if v6 == v4:
                    return v7
                if v4 % v6 == 0 and main(v6 + 1) > v6 - 1:
                    tmp = main(v6)
                    y = (v7 // tmp) * (tmp - 1)
                else:
                    y = v7
                v7 = y
                v6 += 1
                # return f(v6 + 1, y)
        x = f(2, v4)
    else:
        x = v4
    return min(v4, 1 + x)

for i in range(100):
    print(i, main(i))
print(main(1234567))

# for i in range(1234567+1):
#     print(i, main(i))

# print(main(0))
# print(main(1))
# print(main(2))
# print(main(3))