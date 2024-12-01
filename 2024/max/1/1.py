FILE_NAME = "input"


def first_puzzle() -> int:
    with open(FILE_NAME) as file:
        file_content = file.read()

    left_list, right_list = [], []

    for row in file_content.splitlines():
        left_value, right_value = row.split()

        left_list.append(int(left_value))
        right_list.append(int(right_value))

    left_list.sort()
    right_list.sort()

    output_value = 0

    for i in range(len(left_list)):
        output_value += abs(right_list[i] - left_list[i])

    return output_value


def second_puzzle() -> int:
    with open(FILE_NAME) as file:
        file_content = file.read()

    left_list, right_list = [], []

    for row in file_content.splitlines():
        left_value, right_value = row.split()

        left_list.append(int(left_value))
        right_list.append(int(right_value))

    output_value = 0

    # Changes from first solution starting here.
    # Create a dict so we can keep track of every value.
    left_dict = {key: 0 for key in left_list}

    # If value from `right_list` is a key of `left_dict` - increment it's value.
    for i in right_list:
        if i in left_dict:
            left_dict[i] += 1

    # Count the dict values.
    for key, value in left_dict.items():
        output_value += key * value

    return output_value


def main():
    output_1 = first_puzzle()
    output_2 = second_puzzle()
    print(output_1, output_2)


if __name__ == "__main__":
    main()
