"""Basic drawing elements for gene track figures
"""
from abc import ABC, abstractmethod
from pathlib import Path
import drawsvg as draw  # type: ignore

from .colors import Color, SvgColor

# pylint: disable=invalid-name
# pylint: disable=too-many-arguments
# pylint: disable=too-few-public-methods

Direction = str
Interval = tuple[float, float, str]
Tick = float


class Element(ABC):
    """Baseclass for drawable element"""

    def __init__(
        self, x: float, y: float, height: float = 10, width: float = 0, gap: float = 0
    ):
        self.x: float = x
        self.y: float = y
        self.height: float = height
        self.width: float = width
        self.gap: float = gap

    @abstractmethod
    def _draw_elements(self, group: draw.Group, xscale: float) -> draw.Group:
        pass

    def draw(self, x: float = 0, y: float = 0, xscale: float = 1.0) -> draw.Group:
        """Call the rendering library"""
        x = x * xscale
        y = y - self.gap

        group = draw.Group(transform=f"translate({float(x)} {float(y)})")
        return self._draw_elements(group, xscale)


class Label(Element):
    """Wrap a text label"""

    def __init__(self, x: float, text: str, font_size: float = 10, offset: float = 0):
        super().__init__(x, 0, width=x)
        self.font_size = font_size
        self.offset = offset
        self.text: str = str(text)

    def _draw_elements(self, group: draw.Group, xscale: float) -> draw.Group:
        offset = 0.0
        if self.offset is not None:
            offset = self.offset

        group.append(
            draw.Text(
                self.text,
                self.font_size,
                self.width,
                offset,
                font_family="monospace",
                text_anchor="middle",
            )
        )
        return group


class Figure:
    """Genetracks Figure"""

    def __init__(self, padding: float | None = None, track_height: float = 10):
        self.track_height: float = track_height
        self.padding: float

        if padding is None:
            self.padding = track_height / 2
        else:
            self.padding = padding

        self.elements: list[tuple[float, Element]] = []
        self.width: float = 0
        self.height: float = self.padding

    def add(self, element: Element, gap: float = 10, padding: float | None = None):
        """Add an element to the figure.

        :param element: a new Track or other element to add
        :param gap: the distance to leave below the new track
        :param padding: the distance to leave above the new track
        """
        if padding is not None:
            self.height += padding - self.padding
        self.elements.append((self.height + element.height, element))
        self.height += element.height + gap
        self.width = max(self.width, element.width)
        self.padding = gap

    def show(self, w: float | None = None, h: float | None = None) -> draw.Drawing:
        """Show the rendered figure"""

        xscale = 1.0
        if h is None:
            h = self.height
        if w is None:
            w = self.width
        else:
            xscale = w / self.width

        if h is None:
            h = self.height

        d = draw.Drawing(
            self.width * xscale, h, origin=(0, 0), context=draw.Context(invert_y=True)
        )
        for y, element in self.elements:
            d.append(element.draw(xscale=xscale, y=y - h))

        #        d.setRenderSize(w, h)
        return d

    def to_svg(self, path: Path, w: float | None = None, h: float | None = None):
        """Save figure to SVG file"""
        self.show(w=w, h=h).save_svg(path, context=draw.Context(invert_y=True))

    def to_png(self, path: Path, w: float | None = None, h: float | None = None):
        """Safe figure to PNG file"""
        self.show(w=w, h=h).save_png(path, context=draw.Context(invert_y=True))


class Track(Element):
    """Track representing an interval of a genomic sequence"""

    def __init__(
        self,
        a: float,
        b: float,
        height: float = 10,
        label: str | Label | None = None,
        color: Color = SvgColor.LIGHTGREY,
        ticks: list[Tick] | None = None,
        regions: list[Interval] | None = None,
        direction: Direction = "",
    ):
        super().__init__(a, b, height=height, width=b)
        self.color: Color = color

        if "f" in direction:
            self.width += 5

        self.ticks = ticks if ticks is not None else []
        self.label: str | Label | None = label
        self.direction: Direction = direction
        self.regions = regions if regions is not None else []

    def add_tick(self, tick: Tick) -> None:
        """Apply a tick mark to the track"""
        self.ticks.append(tick)

    def _draw_elements(self, group: draw.Group, xscale: float) -> draw.Group:
        a: float = self.x * xscale
        b: float = self.y * xscale

        group.append(
            draw.Rectangle(a, 0, b - a, self.height, fill=self.color, stroke=self.color)
        )

        if "f" in self.direction:
            group.append(
                draw.Lines(
                    b,
                    0,
                    b + 5,
                    (self.height / 2),
                    b,
                    self.height,
                    fill=self.color,
                    stroke=self.color,
                )
            )
        if "r" in self.direction:
            group.append(
                draw.Lines(
                    a,
                    0,
                    a - 5,
                    (self.height / 2),
                    a,
                    self.height,
                    fill=self.color,
                    stroke=self.color,
                )
            )

        for r_a, r_b, color in self.regions:
            r_a = r_a * xscale
            r_b = r_b * xscale
            group.append(
                draw.Rectangle(r_a, 0, r_b - r_a, self.height, fill=color, stroke=color)
            )

        for tick in self.ticks:
            tick = tick * xscale
            group.append(draw.Lines(tick, 0, tick, self.height, stroke="red"))

        if self.label:
            label: Label
            # font_size: float = 10
            # offset: float = self.height + font_size

            if isinstance(self.label, Label):
                label = self.label

            elif isinstance(self.label, str):
                label = Label(0, self.label)

            group.append(label.draw(x=(b + a) / 2))
        return group


class Coverage(Element):
    """Coverage graph"""

    def __init__(
        self,
        a: float,
        b: float,
        ys: list[float],
        height: float = 10,
        color: Color = SvgColor.BLUE,
        opacity: str = "1.0",
    ):
        super().__init__(a, b, width=b, height=height)
        self.color: Color = color
        self.opacity: str = opacity
        self.ys: list[float] = ys

    def _draw_elements(self, group: draw.Group, xscale: float):
        yscale = self.height / max(self.ys)
        a: float = self.x * xscale
        # b: float = self.y * xscale

        i: float
        v: float
        for i, v in enumerate(self.ys):
            group.append(
                draw.Rectangle(
                    a + (i * xscale),
                    0,
                    xscale,
                    v * yscale,
                    fill=self.color,
                    fill_opacity=self.opacity,
                )
            )  # , stroke=self.color))
        return group


class Alignment(Element):
    """Link two tracks to illustrate similar regions"""

    def __init__(
        self,
        track1: Track,
        track2: Track,
        connections: list[tuple[float, float]],
        # text: str | None = None,
        # style: str | None = None,
        gap: float = 30,
        color: Color = SvgColor.BLACK,
    ):
        height = track1.height + track2.height + gap
        end = max(track1.y, track2.y)
        start = min(track1.x, track2.x)
        width = max(track1.width, track2.width)

        super().__init__(start, end, width=width, height=height, gap=gap)

        self.color = color
        self.t1 = track1
        self.t2 = track2
        self.connections = connections

    def _draw_elements(self, group: draw.Group, xscale: float):
        group.append(self.t1.draw(xscale=xscale))
        group.append(self.t2.draw(y=self.t1.height + self.gap, xscale=xscale))

        for bottom, top in self.connections:
            bottom = bottom * xscale
            top = top * xscale
            group.append(draw.Lines(bottom, 0, top, -self.gap, stroke=self.color))
            group.append(
                draw.Lines(bottom, self.t1.height, bottom, 0, stroke=self.color)
            )
            group.append(
                draw.Lines(
                    top, -self.gap, top, -(self.gap + self.t2.height), stroke=self.color
                )
            )
        return group


class Multitrack(Element):
    """Pack multiple tracks onto a line"""

    def __init__(self, tracks: list[Track], join: bool = False):
        height: float = max(track.height for track in tracks)
        width: float = max(track.width for track in tracks)
        length: float = max(track.x for track in tracks)

        super().__init__(0, length, width=width, height=height)

        self.tracks: list[Track] = tracks
        self.join: bool = join

    def _draw_elements(self, group: draw.Group, xscale: float) -> draw.Group:
        if self.join:
            start = min(t.x for t in self.tracks) * xscale
            end = max(t.x for t in self.tracks) * xscale
            group.append(
                draw.Lines(
                    start, self.height / 2, end, self.height / 2, stroke="lightgrey"
                )
            )
        for track in self.tracks:
            group.append(track.draw(xscale=xscale))
        return group
