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
        self.gap: float = 0.0

    # abstract method
    def _draw_elements(self, group: draw.Group, xscale: float) -> draw.Group:
        pass

    def draw(self, x: float = 0, y: float = 0, xscale: float = 1.0) -> draw.Group:
        x = x * xscale
        y = y - self.gap

        group = draw.Group(transform=f"translate({float(x)} {float(y)})")
        return self._draw_elements(group, xscale)


class Label(Element):
    """Wrap a text label"""

    def __init__(self, x: float, text: str, font_size: float = 10, offset: float = 0):
        self.font_size = font_size
        self.offset = offset
        self.text: str = str(text)
        self.h: float = font_size
        self.w = x  # it would be cool to know how wide the text is
        self.gap: float = 0.0

    def _draw_elements(self, group: draw.Group, xscale: float) -> draw.Group:
        offset = 0.0
        if self.offset is not None:
            offset = self.offset

        group.append(
            draw.Text(
                self.text,
                self.font_size,
                self.w,
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

    def show(self, w: float | None = None, h: float | None = None) -> draw.Drawing:
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
        self.gap = 0

    def add_tick(self, tick: Tick) -> None:
        self.ticks.append(tick)

    def _draw_elements(self, group: draw.Group, xscale: float) -> draw.Group:
        a: float = self.a * xscale
        b: float = self.b * xscale

        group.append(
            draw.Rectangle(a, 0, b - a, self.h, fill=self.color, stroke=self.color)
        )

        if "f" in self.direction:
            group.append(
                draw.Lines(
                    b,
                    0,
                    b + 5,
                    (self.h / 2),
                    b,
                    self.h,
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
                    (self.h / 2),
                    a,
                    self.h,
                    fill=self.color,
                    stroke=self.color,
                )
            )

        for r_a, r_b, color in self.regions:
            r_a = r_a * xscale
            r_b = r_b * xscale
            group.append(
                draw.Rectangle(r_a, 0, r_b - r_a, self.h, fill=color, stroke=color)
            )

        for tick in self.ticks:
            tick = tick * xscale
            group.append(draw.Lines(tick, 0, tick, self.h, stroke="red"))

        if self.label:
            label: Label
            font_size: float = 10
            offset: float = self.h + font_size

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
        self.gap = 0

    def _draw_elements(self, group: draw.Group, xscale: float):
        yscale = self.h / max(self.ys)
        a: float = self.a * xscale
        b: float = self.b * xscale

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

    def _draw_elements(self, group: draw.Group, xscale: float):
        group.append(self.t1.draw(xscale=xscale))
        group.append(self.t2.draw(y=self.t1.h + self.gap, xscale=xscale))

        for bottom, top in self.connections:
            bottom = bottom * xscale
            top = top * xscale
            group.append(draw.Lines(bottom, 0, top, -self.gap, stroke=self.color))
            group.append(draw.Lines(bottom, self.t1.h, bottom, 0, stroke=self.color))
            group.append(
                draw.Lines(
                    top, -self.gap, top, -(self.gap + self.t2.h), stroke=self.color
                )
            )
        return group


class Multitrack(Element):
    """Pack multiple tracks onto a line"""

    def __init__(self, tracks: list[Track], join: bool = False):
        self.tracks: list[Track] = tracks
        self.join: bool = join
        self.h: float = max(map(lambda x: x.h, tracks))
        self.w: float = max(map(lambda x: x.b, tracks))
        self.b: float = max(map(lambda x: x.b, tracks))
        self.gap = 0

    def _draw_elements(self, group: draw.Group, xscale: float) -> draw.Group:
        if self.join:
            start = min([t.a for t in self.tracks]) * xscale
            end = max([t.b for t in self.tracks]) * xscale
            group.append(
                draw.Lines(start, self.h / 2, end, self.h / 2, stroke="lightgrey")
            )
        for track in self.tracks:
            group.append(track.draw(xscale=xscale))
        return group


# class Tick:
#    """Wrapper for tick
#    """
#    def __init__(self, x: float, color: Color = 'red'):
#        self.x: float = x
