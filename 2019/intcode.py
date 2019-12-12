from collections import namedtuple

from typing import List, Callable, Any


class DefaultList(list):
    """
    Like `defaultdict` but for lists. Takes a factory for new values past the
    current length.
    """

    def __init__(self, default_factory: Callable):
        self._default_factory = default_factory

    def __getitem__(self, key: int):
        if len(self) <= key:
            super().extend(
                [self._default_factory() for _ in range(key - len(self) + 1)]
            )

        return super().__getitem__(key)

    def __setitem__(self, key: int, value: Any):
        if len(self) <= key:
            super().extend(
                [self._default_factory() for _ in range(key - len(self) + 1)]
            )
        super().__setitem__(key, value)


class Computer:
    Opt = namedtuple("Optcode", ["n_args", "func", "output"])

    code_to_opt = {
        "99": "return",
        "01": Opt(2, "f01", True),
        "02": Opt(2, "f02", True),
        "03": Opt(0, "f03", True),
        "04": Opt(1, "f04", False),
        "05": Opt(2, "f05", False),
        "06": Opt(2, "f06", False),
        "07": Opt(2, "f07", True),
        "08": Opt(2, "f08", True),
        "09": Opt(1, "f09", False),
    }

    def __init__(
        self,
        input_ixs: List[str],
        inputs: List[int] = None,
        debug_mode: bool = False,
    ):
        self.ixs = DefaultList(lambda: "0")
        self.ixs.extend(input_ixs)
        self.pntr = 0
        self.relative_base = 0
        self.output = []
        if inputs:
            self.inputs = inputs.copy()
        else:
            self.inputs = []
        self.is_halted = False
        self.debug_mode = debug_mode

    def __call__(self):
        while not self.is_halted:
            out = self.run_instruction()
            if out is not None:
                break
        return self.output[-1]

    def __str__(self):
        diagnostics = f"""
            inputs: {self.inputs}
            instructions: {self.ixs}
            pointer: {self.pntr}
            relative_base: {self.relative_base}
            ix: {self.ixs[self.pntr]}
            output: {self.output}
            is_halted: {self.is_halted}
            debug_mode: {self.debug_mode}
        """
        return diagnostics

    def __lshift__(self, input_to_add):
        self.inputs.append(input_to_add)

    def run_until_halt(self):
        while not self.is_halted:
            self.run_instruction()
        return self.output

    def run_instruction(self) -> int:
        if self.debug_mode:
            print(self)
        ix = self.ixs[self.pntr]
        if ix == "99":
            self.pntr = 0
            self.is_halted = True
            return
        opt_str, modes = ix[-2:].zfill(2), ix[:-2].zfill(3)[::-1]
        # print(f"opt_str {opt_str}, modes {modes}")
        opt = self.code_to_opt[opt_str]

        buffer = []
        for i in range(opt.n_args):
            mode = modes[i]
            pos = int(self.ixs[self.pntr + i + 1])
            # print(f"{i}, {pos}")
            if mode == "1":
                buffer.append(pos)
            elif mode == "2":
                buffer.append(int(self.ixs[self.relative_base + pos]))
            else:
                buffer.append(int(self.ixs[pos]))
        if opt.output:
            mode = modes[opt.n_args]
            pos = int(self.ixs[self.pntr + opt.n_args + 1])
            if mode == "2":
                buffer.append(self.relative_base + pos)
            else:
                buffer.append(pos)

        # print(f"buffer {buffer}")
        # lookup callable in class
        return getattr(self, opt.func)(*buffer)

    def f01(self, in_1, in_2, out_loc):
        self.ixs[out_loc] = str(in_1 + in_2)
        self.pntr += 4

    def f02(self, in_1, in_2, out_loc):
        self.ixs[out_loc] = str(in_1 * in_2)
        self.pntr += 4

    def f03(self, out_loc):
        self.ixs[out_loc] = str(self.inputs.pop(0))
        self.pntr += 2

    def f04(self, out):
        self.output.append(out)
        self.pntr += 2
        return out

    def f05(self, in_1, in_2):
        if in_1 != 0:
            self.pntr = in_2
        else:
            self.pntr += 3

    def f06(self, in_1, in_2):
        if in_1 == 0:
            self.pntr = in_2
        else:
            self.pntr += 3

    def f07(self, in_1, in_2, out_loc):
        self.ixs[out_loc] = "1" if in_1 < in_2 else "0"
        self.pntr += 4

    def f08(self, in_1, in_2, out_loc):
        self.ixs[out_loc] = "1" if in_1 == in_2 else "0"
        self.pntr += 4

    def f09(self, in_1):
        self.relative_base += int(in_1)
        self.pntr += 2
