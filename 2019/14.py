"""
You ask the nanofactory to produce a list of the reactions it can perform that
are relevant to this process (your puzzle input). Every reaction turns some
quantities of specific input chemicals into some quantity of an output
chemical. Almost every chemical is produced by exactly one reaction; the only
exception, ORE, is the raw material input to the entire process and is not
produced by a reaction.

You just need to know how much ORE you'll need to collect before you can
produce one unit of FUEL.

Each reaction gives specific quantities for its inputs and output; reactions
cannot be partially run, so only whole integer multiples of these quantities
can be used. (It's okay to have leftover chemicals when you're done, though.)
For example, the reaction 1 A, 2 B, 3 C => 2 D means that exactly 2 units of
chemical D can be produced by consuming exactly 1 A, 2 B and 3 C. You can run
the full reaction as many times as necessary; for example, you could produce 10
D by consuming 5 A, 10 B, and 15 C.
"""
from pathlib import Path
from collections import namedtuple, Counter
from math import ceil
from typing import List, Dict, Tuple


Reactant = namedtuple("Reactant", ["amount", "chemical"])


def parse_line(line) -> Tuple[str, int, List[Reactant]]:
    inputs, output = line.split(" => ")
    output_amount, output_chemical = output.split()
    reactants = []
    for pair in inputs.split(", "):
        in_amount, in_chemical = pair.split()
        reactants.append(Reactant(int(in_amount), in_chemical))
    return (output_chemical, int(output_amount), reactants)


def parse_input(in_txt: str) -> Dict[str, Tuple[int, List[Reactant]]]:
    reactions = {}
    for line in in_txt.splitlines():
        output_chemical, output_amount, reactants = parse_line(line)
        reactions[output_chemical] = (output_amount, reactants)
    return reactions


in_txt = Path("input14.txt").read_text().rstrip()

recipies = parse_input(in_txt)
chemical_bag = Counter()


def ore_needed(needed_reactant: Reactant, lookup_dict: dict, bag: Counter):
    needed_amount = needed_reactant.amount
    output_amount, inputs = lookup_dict[needed_reactant.chemical]
    bagged_amount = bag[needed_reactant.chemical]
    if bagged_amount > 0:
        if bagged_amount >= needed_amount:
            bag[needed_reactant.chemical] -= needed_amount
            return 0
        else:
            needed_amount -= bagged_amount
            bag[needed_reactant.chemical] = 0
    n_times = ceil(needed_amount / output_amount)

    ore_counter = 0
    for reactant in inputs:
        if reactant.chemical == "ORE":
            n_ore = reactant.amount*n_times
        else:
            n_ore = ore_needed(Reactant(reactant.amount*n_times, reactant.chemical), lookup_dict, bag)
        ore_counter += n_ore
    excess_amount = n_times*output_amount - needed_amount
    bag[needed_reactant.chemical] += excess_amount
    return ore_counter



ore_per_fuel = ore_needed(Reactant(1, "FUEL"), recipies, chemical_bag)

print(f"Part 1: {ore_per_fuel}")

def fuel_given_ore(lookup_dict: dict, ore_per_fuel: int, n_ore: int):
    for n in range(4199999, n_ore, 1):
        ore_used = ore_needed(Reactant(n, "FUEL"), lookup_dict, Counter())
        if ore_used > n_ore:
            return n-1


print(f"Part 2: {fuel_given_ore(recipies, ore_per_fuel, 1000000000000)}")