import re
import sys
from typing import overload


class Inst:
    pass


class Mul(Inst):
    def __init__(self, a: int, b: int) -> None:
        self.a: int = a
        self.b: int = b

    def eval(self) -> int:
        return self.a * self.b

    def __repr__(self) -> str:
        return f"Mul {{ a: Lit {{ value: {self.a} }}, b: Lit {{ value: {self.b} }} }}"


class Do(Inst):
    def __init__(self, do: bool) -> None:
        self.do: bool = do

    def eval(self):
        return self.do

    def __repr__(self) -> str:
        return f"Do {{ value: {1 if self.do else 0} }}"


def lexer(matche: re.Match[str]) -> Inst:
    if "mul" in matche.groups()[0]:
        if len(matche.groups()) < 3:
            raise ValueError("Malformed mul token")
        else:
            a = int(matche.groups()[1])
            b = int(matche.groups()[2])
            if a > 999 or b > 999:
                raise ValueError(f"a or b to high {a}, {b}")
            return Mul(a, b)
    elif "don't" in matche.groups()[0]:
        return Do(False)
    elif "do" in matche.groups()[0]:
        return Do(True)
    else:
        raise ValueError("Unknown token")


def main():
    lex = re.compile(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))")
    supersomme = 0
    instructions: list[Inst] = []
    for line in sys.stdin:
        instructions += [lexer(t) for t in lex.finditer(line)]

    enable = True
    somme = 0

    for i in instructions:
        print(i)
        if isinstance(i, Do):
            enable = i.eval()
        elif isinstance(i, Mul) and enable:
            somme += i.eval()
    print(somme)
    supersomme += somme
    print(supersomme)


if __name__ == "__main__":
    main()
