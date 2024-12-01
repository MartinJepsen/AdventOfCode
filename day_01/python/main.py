
from pathlib import Path


INPUT_PATH = Path(__file__).parent.parent / "input.txt"


def get_lists() -> tuple[list[int], list[int]]:
    """Get the two lists of numbers from the input file, sorted in ascending order."""
    list_left: list[int] = []
    list_right: list[int] = []

    with open(INPUT_PATH, "r") as file:
        for line in file:
            left, right = line.strip().split("   ")
            list_left.append(int(left))
            list_right.append(int(right))

    return sorted(list_left), sorted(list_right)

def get_difference(list_left: list[int], list_right: list[int]) -> int:
    """Get the sum element-wise difference between the two lists."""
    return sum(abs(left - right) for left, right in zip(list_left, list_right))


def main():
    list_left, list_right = get_lists()
    difference = get_difference(list_left, list_right)

    print(f"The sum of the differences between the two lists is: {difference}")



if __name__ == "__main__":
    main()