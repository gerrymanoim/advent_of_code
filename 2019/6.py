"""
To verify maps, the Universal Orbit Map facility uses orbit count checksums -
the total number of direct orbits (like the one shown above) and indirect
orbits.

Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain can
be any number of objects long: if A orbits B, B orbits C, and C orbits D, then
A indirectly orbits D.
"""
from pathlib import Path
from typing import Dict, List
from itertools import groupby
from collections import defaultdict

GroupTree = Dict[str, List[str]]
DepthTree = Dict[int, List[str]]


def inserter(key: str, depth: int, in_dict: GroupTree, tree: DepthTree):
    if key in in_dict:
        children = in_dict[key]

        tree[depth] += children
        for child in children:
            inserter(child, depth + 1, in_dict, tree)


def build_depth_dict(start_key: str, parsed_input: GroupTree) -> DepthTree:
    depth_dict: GroupTree = defaultdict(list)
    inserter(start_key, 0, parsed_input, depth_dict)
    return depth_dict


def calculate_inderect(depth_dict: DepthTree) -> int:
    return sum(depth * len(items) for depth, items in depth_dict.items())


def parse_input(lines: List[str]) -> GroupTree:
    pairs = [pair.split(")") for pair in lines]
    s_func = lambda x: x[0]
    pairs.sort(key=s_func)
    group_dict = {}
    for k, g in groupby(pairs, s_func):
        group_dict[k] = [i[1] for i in list(g)]
    return group_dict


def calculate_checksum(text: str) -> int:
    lines = text.splitlines()
    direct_links = len(lines)
    groups = parse_input(lines)
    depth_dict = build_depth_dict("COM", groups)
    indirect_links = calculate_inderect(depth_dict)
    return direct_links + indirect_links


test_input = """COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"""
assert calculate_checksum(test_input) == 42

print(calculate_checksum(Path("input6.txt").read_text()))

test_input_2 = """COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"""


def find_ancestors(key: str, groups: GroupTree):
    path = set()
    while key != "COM":
        key = [k for k in groups.keys() if key in groups[k]][0]
        path.add(key)
    return path


def calculate_transfers(text: str) -> int:
    lines = text.splitlines()
    groups = parse_input(lines)
    for k, g in groups.items():
        if "SAN" in g:
            santa_planet = k
        if "YOU" in g:
            your_planet = k
    you_path = find_ancestors(your_planet, groups)
    santa_path = find_ancestors(santa_planet, groups)
    return len(you_path ^ santa_path) + 2


assert calculate_transfers(test_input_2) == 4

print(calculate_transfers(Path("input6.txt").read_text()))
