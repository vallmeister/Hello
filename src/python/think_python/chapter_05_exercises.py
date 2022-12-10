def check_fermat(a, b, c, n):
    if n > 2 and a**n + b**n == c**n:
        print("Holy smokes, Fermat was wrong!")
    else:
        print("No, that doesn’t work.")

def prompt_fermat():
    a = int(input("Give a\n"))
    b = int(input("Give b\n"))
    c = int(input("Give c\n"))
    n = int(input("Give n\n"))
    check_fermat(a, b, c, n)
prompt_fermat()

def is_triangle(a, b, c):
    if a > b + c or b > a + c or c > a + b:
        print("No")
    else:
        print("Yes")

def prompt_triangle():
    a = int(input("Give a\n"))
    b = int(input("Give b\n"))
    c = int(input("Give c\n"))
    is_triangle(a, b, c)
prompt_triangle()