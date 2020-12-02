from pathlib import Path
from typing import Union, List

INPUT_PATH = Path("../inputs/")

def load_input(day: int, splitlines = True) -> Union[str, List[str]]:
    data = (INPUT_PATH /f"input{day}.txt").read_text()
    
    return data.splitlines() if splitlines else data
