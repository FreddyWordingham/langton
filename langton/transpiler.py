from ast import Add, BinOp, Constant, NodeVisitor, parse, UnaryOp, USub

from typeguard import typechecked


from .bytecode import Bytecode
from .operation import Operation


@typechecked
class Transpiler(NodeVisitor):
    def __init__(self) -> None:
        self._code = Bytecode()

    def compile_expr(self, src: str) -> Bytecode:
        tree = parse(src, mode="eval")
        self.visit(tree.body)
        self._code.append(Operation("HALT"))
        return self._code

    # --- nodes ---
    def visit_Constant(self, node: Constant):
        if isinstance(node.value, bool):
            # If you ever allow bools, decide semantics explicitly.
            raise SyntaxError("bool literals not supported")
        if not isinstance(node.value, int):
            raise SyntaxError("only integer literals supported")
        self._code.append(Operation("PUSH", int(node.value)))

    def visit_BinOp(self, node: BinOp):
        if not isinstance(node.op, Add):
            raise SyntaxError("only + supported")
        self.visit(node.left)
        self.visit(node.right)
        self._code.append(Operation("ADD"))

    def visit_UnaryOp(self, node: UnaryOp):
        if not isinstance(node.op, USub):
            raise SyntaxError("only unary - supported")
        self.visit(node.operand)
        self._code.append(Operation("NEG"))
