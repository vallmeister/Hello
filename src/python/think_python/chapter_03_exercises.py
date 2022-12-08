# exercise 3.3
def right_justify(string):
    return (70 - len(string)) * ' ' + string
print(right_justify('allen'))

# exersice 3.4
def do_twice(f, n):
    f(n)
    f(n)

def print_spam():
    print('spam')

def print_twice(string):
    print(string)
    print(string)

def do_four(f, string):
    do_twice(print_twice, string)

do_twice(print_twice, 'spam')
do_four(print_twice, 'test')

# exercise 3.5
def grid():
    outer = 2 * ('+ ' + 4 * '- ')
    print(outer + '+')
    inner = 2 * ('| ' + 4 * '  ') + '|'
    do_four(print, inner)
    print(outer + '+')
    do_four(print, inner)
    print(outer + '+')
grid()

def grid_4x4():
    outer = 4 * ('+ ' + 4 * '- ')
    print(outer + '+')
    inner = 4 * ('| ' + 4 * '  ') + '|'
    do_four(print, inner)
    print(outer + '+')
    do_four(print, inner)
    print(outer + '+')
    do_four(print, inner)
    print(outer + '+')
    do_four(print, inner)
    print(outer + '+')
grid_4x4()