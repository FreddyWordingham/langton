from dataclasses import dataclass
from typing import Union


@dataclass(frozen=True)
class Operation:
    name: str
    argument: Union[int, bool, None] = None

    def __str__(self) -> str:
        if self.argument is None:
            return self.name

        if isinstance(self.argument, bool):
            return f"{self.name} {'true' if self.argument else 'false'}"

        return f"{self.name} {self.argument}"
