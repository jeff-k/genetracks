"""Circularized genome figures
"""
import drawsvg as draw  # type: ignore

Color = str


class Region:
    def __init__(self, a: float, b: float, color: Color = "lightgrey"):
        self.color = color
        self.a = a
        self.b = b

    def draw(self, circ: float) -> draw.Arc:
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
        regions: list[Region] = [],
        size: float = 400,
        color: Color = "lightgrey",
    ):
        self.size = size
        self.regions = regions
        self.length = length
        self.color = color

    def show(self) -> draw.Drawing:
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
