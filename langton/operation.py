from __future__ import annotations
from dataclasses import dataclass
from typing import Union


@dataclass(frozen=True)
class Operation:
    name: str
    argument: Union[int, None] = None
