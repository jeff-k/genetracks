"""Colour handling datastructures
"""
from typing import Union
from enum import Enum

# pylint: disable=invalid-name


class SvgColor(Enum):
    """Enum of valid SVG colours"""

    LIGHTBLUE = "lightblue"
    SALMON = "salmon"
    LIGHTGREY = "lightgrey"
    ORANGE = "orange"
    TURQUOISE = "turquoise"
    YELLOWGREEN = "yellowgreen"
    PLUM = "plum"
    RED = "red"
    DARKGREY = "darkgrey"
    STEELBLUE = "steelblue"
    MEDIUMAQUAMARINE = "mediumaquamarine"
    BLACK = "black"
    BLUE = "blue"
    FIREBRICK = "firebrick"
    SLATEBLUE = "slateblue"

    @staticmethod
    def new(color: str) -> SvgColor:  # pylint: disable=undefined-variable
        """Instantiate a Color from either enum value or string"""
        if color.upper() in SvgColor:
            return SvgColor[color.upper()]

        raise ValueError(f"Invalid color: {color}")

    def __str__(self) -> str:
        return self.value


class HexColor:
    """Represents colours using hex values"""

    def __init__(self, value: str):
        if not value.startswith("#"):
            value = "#" + value

        self._r, self._g, self._b, self._a = self._parse_hex(value)

    @staticmethod
    def _parse_hex(value: str) -> tuple[int, int, int, int]:
        value = value.lstrip("#")

        r: int = int(value[0:2], 16)
        g: int = int(value[2:4], 16)
        b: int = int(value[4:6], 16)
        a: int

        if len(value) == 6:
            a = 255
        elif len(value) == 8:
            a = int(value[6:8], 16)
        else:
            raise ValueError("Invalid hex colour")

        return r, g, b, a

    def __str__(self) -> str:
        return f"#{self._r:02X}#{self._g:02X}#{self._b:02X}#{self._a:02X}"

    @property
    def r(self) -> int:
        """Red channel"""
        return self._r

    @r.setter
    def r(self, value: int):
        self._r = max(0, min(255, value))

    @property
    def g(self) -> int:
        """Green channel"""
        return self._g

    @g.setter
    def g(self, value: int):
        self._g = max(0, min(255, value))

    @property
    def b(self) -> int:
        """Blue channel"""
        return self._b

    @b.setter
    def b(self, value: int):
        self._b = max(0, min(255, value))

    @property
    def a(self) -> int:
        """Alpha channel"""
        return self._a

    @a.setter
    def a(self, value: int):
        self._a = max(0, min(255, value))


Color = Union[SvgColor, HexColor]
