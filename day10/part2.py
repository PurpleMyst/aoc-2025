# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "z3-solver",
# ]
# ///
from z3 import *


def main() -> None:
    result = 0

    for line in open("src/input.txt"):
        s = Optimize()
        vars = []
        presses = 0

        _, *buttons, expected_counters = line.split()
        expected_counters = list(map(int, expected_counters[1:-1].split(",")))

        real_counters = [0] * len(expected_counters)
        for i, button in enumerate(buttons):
            var = Int(f"btn{i}")
            vars.append(var)
            s.add(var >= 0)
            presses += var
            for j in map(int, button[1:-1].split(",")):
                real_counters[j] += var

        for real, expected in zip(real_counters, expected_counters):
            s.add(real == expected)
        s.minimize(presses)
        assert s.check() == sat
        model = s.model()
        for var in vars:
            result += model[var].as_long()

    print(result)



if __name__ == "__main__":
    main()
