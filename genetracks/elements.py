"""Basic drawing elements for gene track figures
"""
import drawSvg as draw

class Figure:
    """Genetracks Figure
    """
    def __init__(self, tracks=[], height=75, width=700, size=700):
        self.padding = 10
        self.tracks = tracks
        self.height = 0
        self.size = size
        self.track_height = 10
        self.width = width # horizontal scaling
        #self.d = draw.Drawing(origin=(0,0))
        self.elements = []
#        self.d = draw.Drawing(self.size, self.height, origin=(0, 0))
        self.tracks = 0
        self.y_max = width

    def to_svg(self, g):
        pass

    def add_track(self, track, gap=30, h=None):
        h = self.track_height
        track.set_y(-(self.tracks * gap))
        track.set_h(h)
        self.elements.append(track)
        self.height += h
        self.y_max = max(self.y_max, track.b)
#        self.d.append(track.draw(h=h, y=-(self.tracks * gap, yscale=self.width)))
        self.tracks += 1

    def add_alignment(self, alignment):
        alignment.set_h(10)
        self.height += 30
        self.elements.append(alignment)
#        self.d.append(alignment.draw(h=10, yscale=self.width))

    def add_coverage(self, coverage, gap=30, h=None, offset=0):
#        self.d.append(coverage.draw(y=-(self.tracks * gap) + offset, h=10))
        h = self.track_height
        self.elements.append(coverage)
        self.height += h
        self.tracks += 1

    def show(self):
        d = draw.Drawing(self.width, self.height, origin=(0,0))
        yscale = self.width / self.y_max
        for element in self.elements:
            d.append(element.draw(yscale=yscale))

        d.setRenderSize(self.size)
        return d

    def to_png(self, path):
        self.show().savePng(path)

class Element:
    """Baseclass for drawable element
    """
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.y_max = y

    def set_y(self, y):
        self.y = y

    def set_h(self, y):
        self.h = h

    def draw(self, x=0, y=0, xscale=1.0):
        h = self.h
        y = self.y

class Track(Element):
    """Track representing an interval of a genomic sequence
    """
    def __init__(self, a, b, label=None, color='lightgrey', ticks=[],
                 regions=[], direction=""):
        self.color = color
        self.a = a
        self.b = b
        self.x = 0
        self.y = 0
        self.h = 10
        self.ticks = ticks
        self.label = label
        self.direction = direction
        self.regions = regions

    def add_tick(self, tick):
        self.ticks.append(tick)

    def draw(self, x=0, y=0, h=10, yscale=1.0):
        h = self.h
        y = self.y
        self.a = self.a * yscale
        self.b = self.b * yscale
        d = draw.Group(transform="translate({} {})".format(x, y))
        d.append(draw.Rectangle(self.a, 0, self.b - self.a, h,
                                fill=self.color, stroke=self.color))

        if 'f' in self.direction:
            d.append(draw.Lines(self.b, 0, self.b + 5, h / 2, self.b, h,
                                fill=self.color, stroke=self.color))
        if 'r' in self.direction:
            d.append(draw.Lines(self.a, 0, self.a - 5, h / 2, self.a, h,
                                fill=self.color, stroke=self.color))

        for a, b, color in self.regions:
            d.append(draw.Rectangle(a, 0, b - a, h, fill=color, stroke=color))

        for tick in self.ticks:
            d.append(draw.Lines(tick, 0, tick, h, stroke='red'))

        if self.label:
            label = self.label
            font_size = 10
            offset = h + font_size
            if isinstance(self.label, Label):
                label = self.label.text
                font_size = self.label.font_size
#                font_family = self.label.font_family
                if self.label.offset is not None:
                    offset = self.label.offset

            d.append(draw.Text(label, font_size, (self.b + self.a) / 2,
                               offset, font_family='monospace', center=True))

        return d


class Coverage:
    """Coverage graph
    """
    def __init__(self, a, b, ys, height = 10, color='blue', opacity='1.0'):
        self.color = color
        self.opacity = opacity
        self.a = a
        self.b = b
        self.ys = ys
        self.height = height

    def draw(self, x=0, y=0, h=100):
        assert isinstance(x, int) and isinstance(y, int)
        d = draw.Group(transform="translate({} {})".format(x, y))

        scale = max(self.ys) / self.height

        for i, y in enumerate(self.ys):
            d.append(draw.Rectangle(self.a + (i * self.b), 0, self.b, y / scale,
                                    fill=self.color, fill_opacity=self.opacity))#, stroke=self.color))
        return d


class Label:
    """Wrap a text label
    """
    def __init__(self, text, font_size=10, offset=None):
        self.font_size = font_size
        self.offset = offset
        self.text = text


class Alignment:
    """Link two tracks to illustrate similar regions
    """
    def __init__(self, track1, track2, connections, text=None, style=None):
        self.t1 = track1
        self.t2 = track2
        self.connections = connections

    def draw(self, x=0, y=0, h=10, gap=50, yscale=1.0):
        g = draw.Group(transform="translate({} {})".format(x, y))
        g.append(self.t1.draw(x=0, y=0))
        g.append(self.t2.draw(x=0, y=-gap))

        for bottom, top in self.connections:
            g.append(draw.Lines(bottom, h, top, gap, stroke='black'))
            g.append(draw.Lines(bottom, 0, bottom, h, stroke='black'))
            g.append(draw.Lines(top, gap, top, gap + h, stroke='black'))
        return g


class Multitrack(Element):
    """Pack multiple tracks onto a line
    """
    def __init__(self, tracks, join=False):
        self.tracks = tracks
        self.join = join
        self.y = 0
        self.height = 10
        self.b = 0

    def set_y(self, y):
        self.y = y

    def set_h(self, h):
        self.height = h

    def draw(self, x=0, y=0, h=10, yscale=1.0):
        y = self.y
        h = self.height
        g = draw.Group(transform="translate({} {})".format(x, y))
        if self.join:
            start = min([t.a for t in self.tracks])
            end = max([t.b for t in self.tracks])
            g.append(draw.Lines(start, h / 2, end, h / 2, stroke='lightgrey'))
        for track in self.tracks:
            g.append(track.draw(h=h, yscale=yscale))

        return g


class Tick:
    """
    wrapper for tick
    """
    def __init__(self, pos, color='red'):
        self.pos = pos
