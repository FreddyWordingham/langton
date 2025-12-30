from __future__ import annotations
import ast as pyast
from dataclasses import dataclass
from typing import List, Tuple, Union

import langton


if __name__ == "__main__":
    print("Generating bytecode...")

    # -------- Bytecode --------

    @dataclass(frozen=True)
    class Op:
        name: str
        arg: Union[int, None] = None

    Bytecode = List[Op]

    def encode_text(bc: Bytecode) -> str:
        """Simple text format: one instruction per line."""
        out = []
        for op in bc:
            if op.arg is None:
                out.append(op.name)
            else:
                out.append(f"{op.name} {op.arg}")
        return "\n".join(out) + "\n"

    # -------- Transpiler --------

    class Transpiler(pyast.NodeVisitor):
        def __init__(self) -> None:
            self.bc: Bytecode = []

        def compile_expr(self, src: str) -> Bytecode:
            tree = pyast.parse(src, mode="eval")
            self.visit(tree.body)
            self.bc.append(Op("HALT"))
            return self.bc

        # --- nodes ---
        def visit_Constant(self, node: pyast.Constant):
            if isinstance(node.value, bool):
                # If you ever allow bools, decide semantics explicitly.
                raise SyntaxError("bool literals not supported")
            if not isinstance(node.value, int):
                raise SyntaxError("only integer literals supported")
            self.bc.append(Op("PUSH", int(node.value)))

        def visit_BinOp(self, node: pyast.BinOp):
            if not isinstance(node.op, pyast.Add):
                raise SyntaxError("only + supported")
            self.visit(node.left)
            self.visit(node.right)
            self.bc.append(Op("ADD"))

        def visit_UnaryOp(self, node: pyast.UnaryOp):
            if not isinstance(node.op, pyast.USub):
                raise SyntaxError("only unary - supported")
            self.visit(node.operand)
            self.bc.append(Op("NEG"))

    def python_to_bytecode(src: str) -> Bytecode:
        return Transpiler().compile_expr(src)

    # -------- demo --------

    if __name__ == "__main__":
        expr = "1 + 2 + (40 + -3)"
        bc = python_to_bytecode(expr)
        print(";; expr:", expr)
        print(encode_text(bc))
