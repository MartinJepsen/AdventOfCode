import itertools
from pathlib import Path

INPUT_PATH = Path(__file__).parent.parent / "input.txt"




MAX_GRADIENT = 3
MIN_GRADIENT = 1


def main():
    n_safe_reports = 0
    with open(INPUT_PATH, "r") as file:
        for line in file:
            numbers = [int(num) for num in line.strip().split(" ")]

            # Check the gradient of the first two numbers
            # We don't have to check remaining gradients if the first two are outside the safe range
            first_gradient = numbers[1] - numbers[0]
            if not (MIN_GRADIENT <= abs(first_gradient) <= MAX_GRADIENT):
                continue
            # Get the direction of the gradient of the first two numbers
            # All subsequent gradients should have the same direction, otherwise, the report is unsafe
            is_ascending = first_gradient > 0
            
            # We can skip the first pair of numbers, because we already checked them
            for (y_current, y_next) in itertools.pairwise(numbers[1:]):
                current_gradient = y_next - y_current
                # Report is unsafe if gradient changed direction or is outside the safe range
                if (
                    (current_gradient > 0) is not is_ascending
                    or not (MIN_GRADIENT <= abs(current_gradient) <= MAX_GRADIENT)
                ):
                    break
            # If we traversed all numbers in the report, it is safe
            else:
                n_safe_reports += 1

    print(f"The number of safe reports is: {n_safe_reports}")


if __name__ == "__main__":
    main()

