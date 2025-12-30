from .bytecode import Bytecode
from .transpiler import Transpiler


def transpile(source: str) -> Bytecode:
    """
    Transpile a Python expression into Langton bytecode.
    """

    return Transpiler().compile_expr(source)
