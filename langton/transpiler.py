from ast import Add, BinOp, Constant, NodeVisitor, parse, UnaryOp, USub
from typing import Literal

from typeguard import typechecked


from .bytecode import Bytecode
from .operation import Operation


Kind = Literal["int", "bool"]


@typechecked
class Transpiler(NodeVisitor):
    def __init__(self) -> None:
        self._code = Bytecode()

    def compile(self, src: str) -> Bytecode:
        self._code = Bytecode()  # Reset bytecode
        tree = parse(src, mode="eval")
        self.visit(tree.body)
        self._code.append(Operation("HALT"))
        return self._code

    # -- Nodes --

    def visit_Constant(self, node: Constant) -> Kind:
        if isinstance(node.value, bool):
            self._code.append(Operation("PUSH_BOOL", node.value))
            return "bool"

        if isinstance(node.value, int):
            self._code.append(Operation("PUSH", int(node.value)))
            return "int"

        raise SyntaxError(f"Unsupported constant type: {type(node.value).__name__}")

    def visit_BinOp(self, node: BinOp) -> Kind:
        if not isinstance(node.op, Add):
            raise SyntaxError("Only + supported")

        lhs_kind = self.visit(node.left)
        rhs_kind = self.visit(node.right)

        if lhs_kind != "int" or rhs_kind != "int":
            raise TypeError(f"ADD requires int + int, got {lhs_kind} and {rhs_kind}")

        self._code.append(Operation("ADD"))
        return "int"

    def visit_UnaryOp(self, node: UnaryOp) -> Kind:
        if not isinstance(node.op, USub):
            raise SyntaxError("Only unary - supported")

        kind = self.visit(node.operand)
        if kind != "int":
            raise TypeError(f"NEG requires int, got {kind}")

        self._code.append(Operation("NEG"))
        return "int"

    def generic_visit(self, node):
        """
        Unknown nodes fail loudly and early.
        """

        raise SyntaxError(f"Unsupported syntax: {type(node).__name__}")
