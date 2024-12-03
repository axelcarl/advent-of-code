right = []
left = []
right_dict = {}

def Main():
    Part2()

def Part2():
    parse("Day1/input.txt")
    result = 0
    for num in left:
        result += (num*right_dict[num])
    print(result)

def Part1():
    parse("Day1/input.txt")
    total = 0
    right.sort()
    left.sort()
    for i in range(len(right)):
        if left[i] >= right[i]:
            total += (left[i] - right[i])
        else:
            total += (right[i] - left[i])
    print(total)

def parse(file_path):
    num_list = []
    try:
        with open(file_path, 'r') as file:
            for line in file:
                num_list = [int(x) for x in line.split()]
                left.append(num_list[0])
                if num_list[1] not in right_dict:
                    right_dict[num_list[1]] = 0

                if num_list[0] not in right_dict:
                    right_dict[num_list[0]] = 0
                    
                right_dict[num_list[1]] += 1
    except FileNotFoundError:
        print("Error reading file")

Main()