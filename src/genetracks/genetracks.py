"""
    This is a standalone version of genetracks for use with ChatGPT-4's
    Advanced Data Analysis feature.
    
    The entire package is contained in this file so that it can be uploaded
    into a chat.
    
    Type annotations are compatible with Python 3.8
"""

from typing import Union, Tuple, List, NamedTuple
from enum import Enum
from abc import ABC, abstractmethod
import io
import unittest

import cairosvg  # type: ignore
from IPython.core.display import display_png


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
    GREEN = "green"
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


class Coord(NamedTuple):
    x: float
    y: float

    def rescale_x(self, xscale: float) -> "Coord":
        return Coord(self.x * xscale, self.y)


class ViewBox(NamedTuple):
    x: int
    y: int
    width: int
    height: int

    def __str__(self) -> str:
        return f"{self.x} {self.y} {self.width} {self.height}"


class Primitive(ABC):
    """Baseclass for drawable image primitive"""

    def __init__(self, coords: Union[Coord, List[Coord]], color: Color):
        self.coords = coords
        self.color = color

    @abstractmethod
    def _generate_svg(self) -> str:
        pass

    def to_svg(self, xscale: float = 1.0, yscale: float = 1.0) -> str:
        if isinstance(self.coords, Coord):
            self.coords = self.coords.rescale_x(xscale)
        else:
            self.coords = [coord.rescale_x(xscale) for coord in self.coords]

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
        attrs = {
            "x": x,
            "y": y,
            # "dy": "0.5em",
            "dominant-baseline": "central",
            "text-anchor": "middle",
            "font-family": "monospace",
            "font-size": self.font_size,
            "fill": self.color,
        }

        attr_str: str = " ".join(
            [f'{attr} = "{str(value)}"' for attr, value in attrs.items()]
        )
        return f"<text {attr_str}>{str(self.text)}</text>\n"


class Lines(Primitive):
    def __init__(self, coords: List[Coord], color: Color = SvgColor.BLACK):
        super().__init__(coords, color)

    def _generate_svg(self):
        points_str = " ".join([f"{x},{y}" for (x, y) in self.coords])
        return f'<polyline points="{points_str}" stroke="{self.color}" stroke-width="1.0" fill="none" />'


class Rectangle(Primitive):
    def __init__(self, coords: List[Coord], color: Color = SvgColor.LIGHTGREY):
        super().__init__(coords, color)

        if len(coords) != 2:
            raise ValueError
        if not isinstance(self.coords[0], Coord) or not isinstance(
            self.coords[1], Coord
        ):
            raise TypeError

    def _generate_svg(self) -> str:
        assert isinstance(self.coords[0], Coord)
        assert isinstance(self.coords[1], Coord)
        x: float = float(self.coords[0].x)
        y: float = float(self.coords[0].y)
        w: float = float(self.coords[1].x)
        h: float = float(self.coords[1].y)

        return f'<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{self.color}" />'


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
        self.translation: Union[None, Coord] = None
        self.xscale: float = 1.0

    def append(self, primitive: Primitive) -> "Group":
        if not isinstance(primitive, Primitive):
            raise TypeError
        self.primitives.append(primitive)
        return self

    def translate(self, coord: Coord) -> "Group":
        self.translation = coord
        return self

    def rescale_x(self, xscale: float) -> "Group":
        self.xscale = xscale
        assert isinstance(self.coords, Coord)
        self.coords = self.coords.rescale_x(xscale)
        return self

    def _generate_svg(self) -> str:
        if self.primitives is None:
            raise ValueError  # empty group!

        grouped_elements = "\n".join(
            [primitive.to_svg(xscale=self.xscale) for primitive in self.primitives]
        )

        open_tag = "<g>"
        if self.translation is not None:
            open_tag = (
                f'<g transform="translate({self.translation.x} {self.translation.y})">'
            )
        return f"{open_tag}\n{grouped_elements}\n</g>"


class Drawing:
    def __init__(self, width: int, height: int, xscale: float = 1.0):
        self.width: int = width
        self.height: int = height
        self.groups: List[Group] = []  # This will hold the grouped primitives
        self.viewbox: Union[None, ViewBox] = None
        self.xscale: float = xscale

    def append(self, group: Group):
        """Appends a Group of Primitives to the SVG document."""
        self.groups.append(group)

    def set_viewbox(self, x: int, y: int, width: int, height: int) -> "Drawing":
        self.viewbox = ViewBox(x, y, width, height)
        return self

    def to_svg(self) -> str:
        """Converts the entire Drawing object into a string of SVG format."""
        # Starting the SVG document

        header = {
            "width": self.width,
            "height": self.height,
            "preserveAspectRatio": "none",
            "xmlns": "http://www.w3.org/2000/svg",
        }

        if self.viewbox is not None:
            header["viewBox"] = self.viewbox

        header_attrs = " ".join(
            [f'{attr} = "{str(value)}"' for attr, value in header.items()]
        )
        svg_header = f"<svg {header_attrs} >\n"

        # Adding SVG elements
        svg_elements = "".join(
            [group.rescale_x(self.xscale).to_svg() for group in self.groups]
        )

        # Closing the SVG document
        svg_footer = "</svg>"

        # Combining everything
        full_svg = svg_header + svg_elements + "\n" + svg_footer
        return full_svg


class TrackElement(ABC):
    """An element that represents a 1-dimensional figure element"""

    def __init__(
        self,
        start: int,
        end: int,
        color: Color = SvgColor.WHITE,
        parent: Union[None, "TrackElement"] = None,
    ):
        self.elements: List[TrackElement] = []
        self.start: int = start
        self.end: int = end
        self.color: Color = color
        self.parent: Union[None, TrackElement] = None

        if parent is not None:
            self.parent = parent

        if self.start > self.end:
            raise ValueError

    def set_parent(self, parent: "TrackElement") -> "TrackElement":
        self.parent = parent
        return self

    def height(self) -> int:
        if self.parent is None:
            raise ValueError("Height attribute is not set")
        return self.parent.height()

    def add(self, element: "TrackElement") -> "TrackElement":
        """Linear track elements can have child elements inside them"""
        self.elements.append(element.set_parent(self))
        return self

    def _max_width(self) -> int:
        return max(
            [
                self.end,
            ]
            + list(map(lambda e: e._max_width(), self.elements))
        )

    @abstractmethod
    def _draw_elements(self, group: Group) -> Group:
        pass

    def draw(
        self, translation: Union[None, Coord] = None, xscale: float = 1.0
    ) -> Group:
        # recursively draw children ontop of self
        coords = Coord(x=0, y=0)  # TODO figure out
        group = Group(coords)  # TODO figure out arguments
        self._draw_elements(group)
        for child in self.elements:
            group.append(child.draw(xscale=xscale))
        return group.rescale_x(xscale)


class Tick(TrackElement):
    """A single tick mark for a linear track"""

    def __init__(self, start: int, color: Color = SvgColor.RED):
        super().__init__(start, start, color)

    def _draw_elements(self, group: Group) -> Group:
        x = self.start
        height = self.height()
        line = Lines([Coord(x, 0), Coord(x, height)], color=self.color)
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
        super().__init__(start, start, color)
        self.font_size = font_size
        self.text: str = str(text)  # TODO: sanitise input?

    def _draw_elements(self, group: Group) -> Group:
        assert self.parent is not None
        x_midpoint = self.parent.start + ((self.parent.end - self.parent.start) / 2)
        y_midpoint: float = self.height() / 2
        group.append(
            Text(
                Coord(x=x_midpoint, y=y_midpoint),
                self.text,
                font_size=self.font_size,
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
            [Coord(x1, y), Coord(x2 - x1, y + self.height())], color=self.color
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


class Track(TrackElement):
    def __init__(
        self,
        elements: Union[None, TrackElement, List[TrackElement]] = None,
        height: int = 12,
    ):
        self.track_height: int = height
        super().__init__(0, 0, SvgColor.WHITE)

        if elements is None:
            self.elements = []
        elif isinstance(elements, TrackElement):
            self.elements = [
                elements.set_parent(self),
            ]
        elif isinstance(elements, List):
            self.elements = [element.set_parent(self) for element in elements]
        else:
            raise TypeError

    def height(self) -> int:
        return self.track_height

    def _draw_elements(self, group: Group) -> Group:
        # TODO: reorganise
        return group

    def draw(
        self, translation: Union[None, Coord] = None, xscale: float = 1.0
    ) -> Group:
        group = Group(Coord(0, 0))
        if translation is not None:
            group.translate(translation)

        for element in self.elements:
            group.append(element.draw(xscale=xscale))
        return group


class Figure:
    """A Figure has an ordered list of Tracks"""

    def __init__(self, track_height: int = 10):
        self.tracks: List[Track] = []
        self.track_height: int = track_height

    def add(self, element: Union[None, Track, TrackElement]) -> None:
        """Initialise a new track and add the optional elements to it"""
        if element is None:
            self.tracks.append(Track())
        elif isinstance(element, Track):
            self.tracks.append(element)
        elif isinstance(element, TrackElement):
            track = Track()
            track.add(element)
            self.tracks.append(track)

    def __get__(self, index: int) -> Track:  # TODO: check correctness
        return self.tracks[index]

    def get_width(self) -> int:
        return max([track._max_width() for track in self.tracks])

    def get_height(self) -> int:
        return sum([track.height() for track in self.tracks])

    def draw(
        self, width: Union[None, int] = None, height: Union[None, int] = None
    ) -> Drawing:
        if width is None:
            width = self.get_width()

        xscale: float = width / self.get_width()

        if height is None:
            height = self.track_height * len(self.tracks)

        drawing = Drawing(width, height)

        for index, track in enumerate(self.tracks):
            drawing.append(
                track.draw(
                    translation=Coord(0, self.track_height * index), xscale=xscale
                )
            )

        # drawing.set_viewbox(0, 0, self.get_width(), height)
        return drawing

    def show(self, width: Union[None, int] = None, height: Union[None, int] = None):
        svg = self.draw(width=width, height=height).to_svg()
        png = cairosvg.svg2png(bytestring=svg.encode("utf-8"))
        display_png(png, raw=True)


def hiv_figure(track_height: int):
    fig: Figure = Figure(track_height=track_height)
    third = [
        (2085, 5096, "pol", SvgColor.ORANGE),
        (5559, 5850, "vpr", SvgColor.TURQUOISE),
        (5970, 6045, "rev", SvgColor.YELLOWGREEN),
        (6225, 8795, "env", SvgColor.SALMON),
    ]

    second = [
        (5831, 6045, "tat", SvgColor.PLUM),
        (6062, 6310, "vpu", SvgColor.RED),
        (8379, 8653, "rev", SvgColor.YELLOWGREEN),
        (9086, 9719, "3' LTR", SvgColor.DARKGREY),
    ]

    first = [
        (0, 634, "5' LTR", SvgColor.DARKGREY),
        (790, 2292, "gag", SvgColor.LIGHTBLUE),
        (5041, 5619, "vif", SvgColor.STEELBLUE),
        (8379, 8469, "tat", SvgColor.PLUM),
        (8797, 9417, "nef", SvgColor.MEDIUMAQUAMARINE),
    ]

    # Initialize track

    for frame in [first, second, third]:
        track = Track()
        # Add segments to track

        for start, end, label, color in frame:
            track.add(Segment(start, end, color=color).add(Label(0, label)))
            # track.add(Segment(start, end, color=color))
            # track.add(Label(start, label, color=SvgColor.BLACK))
            # track.add(Tick(start, color=SvgColor.BLACK))
            track.add(Tick(end - 50, color=SvgColor.RED))
        fig.add(track)
    return fig
