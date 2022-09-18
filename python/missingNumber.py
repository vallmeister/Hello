n = int(input())
numbers = []

line = input()
for i in line.split():
    numbers.append(int(i))

for i in range(1, n+1):
    if i not in numbers:
        print(i)
        break