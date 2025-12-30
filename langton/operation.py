from dataclasses import dataclass
from typing import Union


@dataclass(frozen=True)
class Operation:
    name: str
    argument: Union[int, None] = None

    def __str__(self) -> str:
        if self.argument is None:
            return self.name
        else:
            return f"{self.name} {self.argument}"
