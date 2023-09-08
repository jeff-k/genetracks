"""
This is a standalone version of genetracks for use with ChatGPT-4's
Advanced Data Analysis feature.

The entire package is contained in this file so that it can be uploaded
into a chat.

Type annotations are compatible with Python 3.8
"""

from typing import Union, Tuple, List
from enum import Enum
from abc import ABC, abstractmethod
from collections import namedtuple
import io
import unittest

from PIL import Image
import cairosvg  # type: ignore


class Direction(Enum):
    F = ("f",)
    R = ("r",)
    FR = ("fr",)


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
    WHITE = "white"

    @staticmethod
    def new(color: str) -> "SvgColor":  # pylint: disable=undefined-variable
        """Instantiate a Color from either enum value or string"""
        if color.upper() in SvgColor.__members__:
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
    def _parse_hex(value: str) -> Tuple[int, int, int, int]:
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
        return f"#{self._r:02X}{self._g:02X}{self._b:02X}{self._a:02X}"

    @property
    def r(self) -> int:
        """Red channel"""
        return self._r

    @r.setter
    def r(self, value: int) -> None:
        self._r = max(0, min(255, value))

    @property
    def g(self) -> int:
        """Green channel"""
        return self._g

    @g.setter
    def g(self, value: int) -> None:
        self._g = max(0, min(255, value))

    @property
    def b(self) -> int:
        """Blue channel"""
        return self._b

    @b.setter
    def b(self, value: int) -> None:
        self._b = max(0, min(255, value))

    @property
    def a(self) -> int:
        """Alpha channel"""
        return self._a

    @a.setter
    def a(self, value: int) -> None:
        self._a = max(0, min(255, value))


Color = Union[SvgColor, HexColor]

# Drawing primitives:

Coord = namedtuple("Coord", ["x", "y"])


class Primitive(ABC):
    """Baseclass for drawable image primitive"""

    def __init__(self, coords: Union[Coord, List[Coord]], color: Color):
        self.coords = coords
        self.color = color

    @abstractmethod
    def _generate_svg(self) -> str:
        pass

    def to_svg(self) -> str:
        return self._generate_svg()


class Text(Primitive):
    def __init__(
        self,
        coords: Coord,
        text: str,
        color: Color = SvgColor.BLACK,
        font_size: int = 10,
    ):
        if not isinstance(coords, Coord):
            raise TypeError

        super().__init__(coords, color)
        self.text = text
        self.font_size = font_size

    def _generate_svg(self) -> str:
        assert isinstance(self.coords, Coord)
        x: float = self.coords.x
        y: float = self.coords.y
        return f'<text x="{x}" y="{y}" dy="0.5em" text-anchor="middle" font-size="{self.font_size}" fill="{self.color}">{self.text}</text>'


class Lines(Primitive):
    def __init__(self, coords: List[Coord], color: Color = SvgColor.BLACK):
        super().__init__(coords, color)

    def _generate_svg(self):
        points_str = " ".join([f"{x},{y}" for (x, y) in self.coords])
        return f'<polyline points="{points_str}" stroke="{self.color}" stroke-width="1.0" fill="none" />'


class Rectangle(Primitive):
    def __init__(self, coords: List[Coord], color: Color = SvgColor.LIGHTGREY):
        if len(coords) != 2:
            raise ValueError

        super().__init__(coords, color)

    def _generate_svg(self) -> str:
        return f'<rect x="{self.coords[0].x}" y="{self.coords[0].y}" width="{self.coords[1].x}" height="{self.coords[1].y}" fill="{self.color}" />'


class Group(Primitive):
    """A collection of Primitive elements that can be rendered into SVG"""

    def __init__(
        self,
        coords: Coord,
        color: SvgColor = SvgColor.WHITE,
        primitives: Union[None, List[Primitive]] = None,
    ):
        super().__init__(coords, color)
        if primitives is None:
            primitives = []
        self.primitives = primitives

    def append(self, primitive: Primitive):
        if not isinstance(primitive, Primitive):
            raise TypeError
        self.primitives.append(primitive)

    def _generate_svg(self) -> str:
        if self.primitives is None:
            raise ValueError  # empty group!

        grouped_elements = "\n".join(
            [primitive.to_svg() for primitive in self.primitives]
        )
        return f"<g>\n{grouped_elements}\n</g>"


def rasterise(svg_document):
    # Rasterize SVG to PNG using cairosvg
    png_bytes = cairosvg.svg2png(bytestring=svg_document.encode("utf-8"))

    # Create a PIL Image object from the PNG bytes
    png_image = Image.open(io.BytesIO(png_bytes))

    # Display the rasterized image using PIL's show method (this opens the image using the default image viewer)
    png_image.show()


class Drawing:
    def __init__(self, width: int, height: int):
        self.width: int = width
        self.height: int = height
        self.groups: List[Group] = []  # This will hold the grouped primitives

    def append(self, group: Group):
        """Appends a Group of Primitives to the SVG document."""
        self.groups.append(group)

    def to_svg(self):
        """Converts the entire Drawing object into a string of SVG format."""
        # Starting the SVG document
        svg_header = f'<svg width="{self.width}" height="{self.height}" xmlns="http://www.w3.org/2000/svg">\n'

        # Adding SVG elements
        svg_elements = "".join([group.to_svg() for group in self.groups])

        # Closing the SVG document
        svg_footer = "</svg>"

        # Combining everything
        full_svg = svg_header + svg_elements + "\n" + svg_footer
        return full_svg


class TrackElement(ABC):
    """An element that represents a 1-dimensional figure element"""

    def __init__(self, start: int, end: int, color: Color = SvgColor.WHITE):
        self.elements: List[TrackElement] = []
        self.start: int = start
        self.end: int = end
        self.color: Color = color

    def add(self, element: "TrackElement"):
        """Linear track elements can have child elements inside them"""
        self.elements.append(element)

    @abstractmethod
    def _draw_elements(self, group: Group) -> Group:
        pass

    def draw(self) -> Group:  # TODO what arguments does this need?
        # recursively draw children ontop of self
        coords = Coord(x=0, y=0)  # TODO figure out
        group = Group(coords)  # TODO figure out arguments
        self._draw_elements(group)
        for child in self.elements:
            group.append(child.draw())
        return group


class Tick(TrackElement):
    """A single tick mark for a linear track"""

    def __init__(self, start: int, color: Color = SvgColor.RED):
        super().__init__(start, 0, color)

    def _draw_elements(self, group: Group) -> Group:
        x = self.start
        y = 0  # assuming zero as base y-coordinate
        line = Lines(
            [Coord(x, y), Coord(x, y + 10)], color=self.color
        )  # 10 is the tick height
        group.append(line)
        return group


class Label(TrackElement):
    """A text label drawn onto a track element"""

    def __init__(
        self,
        start: int,
        text: str,
        color: Color = SvgColor.BLACK,
        font_size: int = 10,
    ):
        super().__init__(start, 0, color)
        self.font_size = font_size
        self.text: str = str(text)  # TODO: sanitise input?

    def _draw_elements(self, group: Group) -> Group:
        x = self.start
        y = 0
        group.append(
            Text(
                Coord(x=x, y=y),
                self.text,
                font_size=self.font_size,
                # font_family="monospace",
                # text_anchor="middle",
            )
        )
        return group


class Segment(TrackElement):
    """Track representing an interval of a genomic sequence"""

    def __init__(
        self,
        start: int,
        end: int,
        color: Color = SvgColor.LIGHTGREY,
        direction: Union[None, Direction] = None,
    ):
        super().__init__(start, end, color)
        self.direction = direction

    def _draw_elements(self, group: Group) -> Group:
        x1 = self.start
        x2 = self.end
        y = 0  # assuming zero as base y-coordinate
        rect = Rectangle(
            [Coord(x1, y), Coord(x2 - x1, 10)], color=self.color
        )  # 10 is the segment height
        group.append(rect)
        return group


class Coverage(TrackElement):
    """Coverage graph"""

    def __init__(
        self,
        start: int,
        end: int,
        ys: List[float],
        color: Color = SvgColor.BLUE,
        opacity: float = 1.0,
    ):
        super().__init__(start, end, color)
        self.opacity: float = opacity
        self.ys: List[float] = ys

    def _draw_elements(self, group: Group) -> Group:
        for i, y in enumerate(self.ys):
            x = self.start + i
            rect = Rectangle([Coord(x, 0), Coord(1, y)], color=self.color)
            group.append(rect)
        return group


class Track:
    def __init__(self) -> None:
        self.elements: List[TrackElement] = []

    def add(self, element: TrackElement):
        self.elements.append(element)

    def draw(self) -> Group:
        group = Group(Coord(0, 0))
        for element in self.elements:
            group.append(element.draw())
        return group


class Figure:
    """A Figure has an ordered list of Tracks"""

    def __init__(self, track_height: int = 10):
        self.tracks: List[Track] = []
        self.track_height = track_height

    def add(self, element: Union[None, TrackElement]) -> None:
        """Initialise a new track and add the optional elements to it"""
        track = Track()
        if element is not None:
            track.add(element)
        self.tracks.append(track)

    def __get__(self, index: int) -> Track:  # TODO: check correctness
        return self.tracks[index]

    def draw(
        self, width: Union[None, int] = None, height: Union[None, int] = None
    ) -> Drawing:
        if width is None:
            width = 500  # TODO if the user doesn't specify this we could intead use the maximum coords in the figure's logical values
        if height is None:
            height = self.track_height * len(
                self.tracks
            )  # default height based on the number of tracks

        drawing = Drawing(width, height)

        for track in self.tracks:
            drawing.append(track.draw())

        return drawing

    def show(self, width: Union[None, int] = None, height: Union[None, int] = None):
        rasterise(self.draw(width=width, height=height).to_svg())


class TestGeneTracks(unittest.TestCase):
    def test_tick(self):
        tick = Tick(10)
        track = Track()
        track.add(tick)
        group = track.draw()
        self.assertEqual(
            group.to_svg(),
            '<g>\n<g>\n<polyline points="10,0 10,10" stroke="red" stroke-width="1.0" fill="none" />\n</g>\n</g>',
        )

    def test_segment(self):
        segment = Segment(10, 60)
        track = Track()
        track.add(segment)
        group = track.draw()
        self.assertEqual(
            group.to_svg(),
            '<g>\n<g>\n<rect x="10" y="0" width="50" height="10" fill="lightgrey" />\n</g>\n</g>',
        )

    def test_label(self):
        label = Label(10, "TestLabel")
        track = Track()
        track.add(label)
        group = track.draw()
        self.assertEqual(
            group.to_svg(),
            '<g>\n<g>\n<text x="10" y="0" font-size="10" fill="black">TestLabel</text>\n</g>\n</g>',
        )

    def test_figure(self):
        fig = Figure()
        segment = Segment(10, 60)
        tick = Tick(30)
        fig.add(segment)
        fig.add(tick)
        drawing = fig.draw()
        self.assertIn(
            '<rect x="10" y="0" width="50" height="10" fill="lightgrey" />',
            drawing.to_svg(),
        )
        self.assertIn(
            '<polyline points="30,0 30,10" stroke="red" stroke-width="1.0" fill="none" />',
            drawing.to_svg(),
        )


# if __name__ == "__main__":
#    unittest.main()
