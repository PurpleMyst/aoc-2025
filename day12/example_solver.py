# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "z3-solver",
#     "tqdm",
# ]
# ///
"""
Left here for posterity; this was my initial Z3-based solver for the puzzle. Obviously, the actual
input is devious and does not require any of this :D
"""
import string
from concurrent.futures import ProcessPoolExecutor, as_completed
from functools import partial

import z3
from tqdm import tqdm


def parse_block(s: str):
    block = set()
    for y, row in enumerate(s.splitlines()[1:]):
        for x, cell in enumerate(row):
            if cell == "#":
                block.add((x, y))
    return frozenset(block)


def show(block):
    for y in range(3):
        for x in range(3):
            if (x, y) in block:
                tqdm.write("█", end="")
            else:
                tqdm.write("░", end="")
        tqdm.write("")


def flip(block):
    return frozenset({(2 - x, y) for (x, y) in block})


def rotate(block):
    return frozenset({(2 - y, x) for (x, y) in block})


def main() -> None:
    blocks = list(open("src/input.txt").read().split("\n\n"))
    cases = blocks.pop().splitlines()
    blocks = list(map(parse_block, blocks))
    blocksets = []
    for block in blocks:
        variations = [block]
        for _ in range(3):
            variations.append(rotate(variations[-1]))
        for var in variations.copy():
            variations.append(flip(var))
        variations = list(set(variations))
        blocksets.append(variations)

    flat_blocksets = []
    for blockset in blocksets:
        flat_blocksets.extend(blockset)

    do_solve = partial(solve, blocksets, flat_blocksets)

    with ProcessPoolExecutor(max_workers=4) as pool:
        futures = [
            pool.submit(do_solve, case) for case in cases
        ]

        part1 = 0
        for fut in tqdm(as_completed(futures), total=len(futures)):
            part1 += fut.result()

    tqdm.write(f"{part1}")


def solve(blocksets, flat_blocksets, case):
    solver = z3.Solver()

    size, nums = case.split(":")
    w, h = map(int, size.split("x"))
    nums = list(map(int, nums.split()))

    total_area = w * h
    expected_area = 0
    for blockset, area_contribution in zip(blocksets, nums):
        expected_area += area_contribution * len(blockset[0])
    tqdm.write(f"{expected_area=} {total_area=} ({expected_area/total_area:.2%})")
    if expected_area > total_area:
        return 0

    block_types: list[list[int | z3.ArithRef]] = [[-1 for _ in range(w)] for _ in range(h)]
    occupied = [[[] for _ in range(w)] for _ in range(h)]

    for y in range(h - 2):
        for x in range(w - 2):
            block_types[y][x] = (block_ty := z3.Int(f"b{x:03}{y:03}"))
            solver.add(z3.And(block_ty >= -1, block_ty < len(flat_blocksets)))

            for i, block in enumerate(flat_blocksets):
                for offset_x, offset_y in block:
                    occupied[y + offset_y][x + offset_x].append(block_ty == i)

    counter = 0
    for blockset, expected in zip(blocksets, nums):
        j = counter
        assert flat_blocksets[j : j + len(blockset)] == blockset

        solver.add(
            z3.PbEq(
                [
                    (z3.Or(*(cell == k for k in range(j, j + len(blockset)))), 1)
                    for row in block_types
                    for cell in row
                ],
                expected,
            )
        )

        counter += len(blockset)

    for row in occupied:
        for cell in row:
            solver.add(z3.AtMost(*cell, 1))

    c = solver.check()
    if c == z3.sat:
        m = solver.model()
        grid = {}
        it = (f"\x1b[{31 + i % 6}m{c}\x1b[0m" for i, c in enumerate(string.ascii_uppercase))
        for y, row in enumerate(block_types):
            for x, cell in enumerate(row):
                if isinstance(cell, int) and cell == -1:
                    continue
                ty = m.eval(cell).as_long()
                if ty == -1:
                    continue
                foo = next(it)
                for ox, oy in flat_blocksets[ty]:
                    grid[(x + ox, y + oy)] = foo

        # for y in range(h):
        #     for x in range(w):
        #         tqdm.write(grid.get((x, y), "."), end="")
        #     tqdm.write()

        return 1
    else:
        return 0


if __name__ == "__main__":
    main()
