"""Basic drawing elements for gene track figures
"""
import drawsvg as draw  # type: ignore
from pathlib import Path

Color = str
Direction = str
Interval = tuple[float, float, str]
Tick = float


class Element:
    """Baseclass for drawable element"""

    def __init__(self, x: float, y: float, h: float = 10, w: float = 0):
        self.x: float = x
        self.y: float = y
        self.h: float = h
        self.w: float = w

    def draw(self, x: float = 0, y: float = 0, xscale: float = 1.0):
        pass


class Label(Element):
    """Wrap a text label"""

    def __init__(self, x: float, text: str, font_size: float = 10, offset: float = 0):
        self.font_size = font_size
        self.offset = offset
        self.text: str = str(text)
        self.h: float = font_size
        self.w = x  # it would be cool to know how wide the text is

    def draw(
        self, x: float | None = None, y: float = 0, xscale: float = 1.0
    ) -> draw.Group:
        #           font_family = self.label.font_family
        if self.offset is not None:
            offset = self.offset

        if x is None:
            x = self.w * xscale

        d = draw.Group(transform="translate({} {})".format(x, y))
        d.append(
            draw.Text(
                self.text,
                self.font_size,
                self.w,
                offset,
                font_family="monospace",
                text_anchor="middle",
            )
        )
        return d


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
        self.w: float = 0
        self.h: float = self.padding

    def add(self, element: Element, gap: float = 10, padding: float | None = None):
        """Add an element to the figure.

        :param element: a new Track or other element to add
        :param gap: the distance to leave below the new track
        :param padding: the distance to leave above the new track
        """
        if padding is not None:
            self.h += padding - self.padding
        self.elements.append((self.h + element.h, element))
        self.h += element.h + gap
        self.w = max(self.w, element.w)
        self.padding = gap

    def show(self, w: float | None = None, h: float | None = None):
        xscale = 1.0
        if h is None:
            h = self.h
        if w is None:
            w = self.w
        else:
            xscale = w / self.w

        if h is None:
            h = self.h

        d = draw.Drawing(
            self.w * xscale, h, origin=(0, 0), context=draw.Context(invert_y=True)
        )
        for y, element in self.elements:
            d.append(element.draw(xscale=xscale, y=y - h))

        #        d.setRenderSize(w, h)
        return d

    def to_svg(self, path: Path, w: float | None = None, h: float | None = None):
        self.show(w=w, h=h).save_svg(path, context=draw.Context(invert_y=True))

    def to_png(self, path: Path, w: float | None = None, h: float | None = None):
        self.show(w=w, h=h).save_png(path, context=draw.Context(invert_y=True))


class Track(Element):
    """Track representing an interval of a genomic sequence"""

    def __init__(
        self,
        a: float,
        b: float,
        h: float = 10,
        label: str | Label | None = None,
        color: Color = "lightgrey",
        ticks: list[Tick] = [],
        regions: list[Interval] = [],
        direction: Direction = "",
    ):
        self.color: Color = color
        self.a: float = a
        self.b: float = b
        self.w: float = b
        if "f" in direction:
            self.w += 5
        self.h = h
        self.ticks = ticks
        self.label: str | Label | None = label
        self.direction: Direction = direction
        self.regions = regions

    def add_tick(self, tick: Tick) -> None:
        self.ticks.append(tick)

    def draw(self, x: float = 0, y: float = 0, xscale: float = 1.0) -> draw.Group:
        h: float = self.h
        a: float = self.a * xscale
        b: float = self.b * xscale
        x = x * xscale

        # assert isinstance(x, float) and isinstance(y, float)
        d = draw.Group(transform="translate({} {})".format(x, y))
        d.append(draw.Rectangle(a, 0, b - a, h, fill=self.color, stroke=self.color))

        if "f" in self.direction:
            d.append(
                draw.Lines(
                    b, 0, b + 5, (h / 2), b, h, fill=self.color, stroke=self.color
                )
            )
        if "r" in self.direction:
            d.append(
                draw.Lines(
                    a, 0, a - 5, (h / 2), a, h, fill=self.color, stroke=self.color
                )
            )

        for r_a, r_b, color in self.regions:
            r_a = r_a * xscale
            r_b = r_b * xscale
            d.append(draw.Rectangle(r_a, 0, r_b - r_a, h, fill=color, stroke=color))

        for tick in self.ticks:
            tick = tick * xscale
            d.append(draw.Lines(tick, 0, tick, h, stroke="red"))

        if self.label:
            label: Label
            font_size: float = 10
            offset: float = h + font_size

            if isinstance(self.label, Label):
                label = self.label

            elif isinstance(self.label, str):
                label = Label(0, self.label)

            d.append(label.draw(x=(b + a) / 2))
        return d


class Coverage(Element):
    """Coverage graph"""

    def __init__(
        self,
        a: float,
        b: float,
        ys: list[float],
        height: float = 10,
        color: Color = "blue",
        opacity: str = "1.0",
    ):
        self.color: Color = color
        self.opacity: str = opacity
        self.a = a
        self.b = b
        self.h = height
        self.ys: list[float] = ys
        self.w = b

    def draw(self, x: float = 0, y: float = 0, xscale: float = 1.0) -> draw.Group:
        # assert isinstance(x, int) and isinstance(y, int)
        h = self.h
        a: float = self.a * xscale
        b: float = self.b * xscale
        x = x * xscale
        d = draw.Group(transform="translate({} {})".format(x, y))
        yscale = self.h / max(self.ys)
        i: float
        v: float
        for i, v in enumerate(self.ys):
            d.append(
                draw.Rectangle(
                    a + (i * xscale),
                    0,
                    xscale,
                    v * yscale,
                    fill=self.color,
                    fill_opacity=self.opacity,
                )
            )  # , stroke=self.color))
        return d


class Alignment(Element):
    """Link two tracks to illustrate similar regions"""

    def __init__(
        self,
        track1: Track,
        track2: Track,
        connections: list[tuple[float, float]],
        text: str | None = None,
        style: str | None = None,
        gap: float = 30,
        color: Color = "black",
    ):
        self.t1 = track1
        self.t2 = track2
        self.color = color
        self.connections = connections
        self.gap = gap
        self.h = track1.h + track2.h + gap
        self.b = max(track1.b, track2.b)
        self.a = min(track1.a, track2.a)
        self.w = max(track1.w, track2.w)

    def draw(self, x: float = 0, y: float = 0, xscale: float = 1.0) -> draw.Group:
        d = draw.Group(transform="translate({} {})".format(x, y - self.gap))
        d.append(self.t1.draw(xscale=xscale))
        d.append(self.t2.draw(y=self.t1.h + self.gap, xscale=xscale))

        for bottom, top in self.connections:
            bottom = bottom * xscale
            top = top * xscale
            d.append(draw.Lines(bottom, 0, top, -self.gap, stroke=self.color))
            d.append(draw.Lines(bottom, self.t1.h, bottom, 0, stroke=self.color))
            d.append(
                draw.Lines(
                    top, -self.gap, top, -(self.gap + self.t2.h), stroke=self.color
                )
            )
        return d


class Multitrack(Element):
    """Pack multiple tracks onto a line"""

    def __init__(self, tracks: list[Track], join: bool = False):
        self.tracks: list[Track] = tracks
        self.join: bool = join
        self.h: float = max(map(lambda x: x.h, tracks))
        self.w: float = max(map(lambda x: x.b, tracks))
        self.b: float = max(map(lambda x: x.b, tracks))

    def draw(self, x: float = 0, y: float = 0, xscale: float = 1.0) -> draw.Group:
        h = self.h
        # assert isinstance(x, int) and isinstance(y, int)
        g = draw.Group(transform="translate({} {})".format(x, y))
        if self.join:
            start = min([t.a for t in self.tracks]) * xscale
            end = max([t.b for t in self.tracks]) * xscale
            g.append(draw.Lines(start, h / 2, end, h / 2, stroke="lightgrey"))
        for track in self.tracks:
            g.append(track.draw(xscale=xscale))

        return g


# class Tick:
#    """Wrapper for tick
#    """
#    def __init__(self, x: float, color: Color = 'red'):
#        self.x: float = x
