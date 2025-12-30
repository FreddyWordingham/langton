from .operation import Operation

from typeguard import typechecked


@typechecked
class Bytecode:
    def __init__(self) -> None:
        self._operations = []

    def append(self, operation: Operation) -> None:
        """
        Append an operation to the bytecode.
        """

        self._operations.append(operation)

    def encode(self) -> str:
        """
        Encode the bytecode into a simple text format with a single instruction per line.
        """

        out = []
        for operation in self._operations:
            out.append(str(operation))
        return "\n".join(out) + "\n"
