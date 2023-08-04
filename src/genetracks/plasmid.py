"""Circularized genome figures
"""
import drawsvg as draw  # type: ignore
from .elements import Color

# pylint: disable=invalid-name
# pylint: disable=too-few-public-methods


class Region:
    """An arc of a circular track"""

    def __init__(self, a: float, b: float, color: Color | str = Color.LIGHTGREY):
        self.color: Color = Color(color)
        self.a = a
        self.b = b

    def draw(self, circ: float) -> draw.Arc:
        """Render the arc"""
        return draw.Arc(
            0,
            0,
            0.5,
            self.b / circ * 360,
            self.a / circ * 360,
            stroke_width="0.04",
            stroke=self.color,
            fill_opacity="0.0",
        )


class Plasmid:
    """Cicular genome figure"""

    def __init__(
        self,
        length: float,
        regions: list[Region] | None = None,
        size: float = 400,
        color: Color | str = Color.LIGHTGREY,
    ):
        self.size = size
        self.regions = regions if regions is not None else []
        self.length = length
        self.color: Color = Color(color)

    def show(self) -> draw.Drawing:
        """Display the rendered figure"""
        d = draw.Drawing(1.4, 1.4, origin="center", context=draw.Context(invert_y=True))

        d.append(
            draw.Circle(
                0, 0, 0.5, stroke_width="0.005", stroke=self.color, fill_opacity="0.0"
            )
        )

        for region in self.regions:
            d.append(region.draw(self.length))

        d.set_render_size(self.size)
        return d
