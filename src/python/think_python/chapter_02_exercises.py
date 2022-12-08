# exercise 2.2
import math

width = 17
height = 12.0
delimiter = '.'

print(type(width / 2)) # int
print(type(width / 2.0)) # float
print(type(height / 3)) # float
print(type(1 + 2 * 5)) # int
print(type(delimiter * 5)) # str

# exercise 2.3
print('Volume of sphere with r=5', 4 / 3 * math.pi * 5**3)
print('Total cost for 60 copies: $', 24.95 * 60 * 0.6 + 3 + 0.75 * 59)
start = 6 * 60 + 52
end = start + 1 * 8.25 + 3 * 7.2 + 1 * 8.25
print((str(int(end // 60)) + ":" + str(int(end % 60))))