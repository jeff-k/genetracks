"""
basic elements of gene track figures
"""
import drawSvg as draw

class Figure:
    """Genetracks Figure
    """
    def __init__(self, tracks=[], height=75, width=700, size=700):
        self.padding = 10
        self.tracks = tracks
        self.height = height
        self.size = size
        self.track_height = 10
        self.width = width # horizontal scaling
        self.d = draw.Drawing(self.size, self.height, origin=(0, 0))
        self.tracks = 0

    def to_svg(self, g):
        pass

    def add_track(self, track):
        self.d.append(track.draw(y=-(self.tracks * 30)))
        self.tracks += 1

    def add_alignment(self, alignment):
        self.d.append(alignment.draw(h=10))

    def show(self):
        self.d.setRenderSize(self.size)
        return self.d

    def to_png(self, path):
        self.d.savePng(path)


class Track:
    """Track representing an interval of a genomic sequence
    """
    def __init__(self, a, b, label=None, color='lightgrey', ticks=[],
                 regions=[], direction=""):
        self.color = color
        self.a = a
        self.b = b
        self.x = 0
        self.y = 0
        self.ticks = ticks
        self.label = label
        self.direction = direction
        self.regions = regions

    def add_tick(self, tick):
        self.ticks.append(tick)

    def draw(self, x=0, y=0, h=10, w=700):

        d = draw.Group(transform="translate({} {})".format(x, y))
        d.append(draw.Rectangle(self.a, 0, self.b - self.a, h, fill=self.color,
                                stroke=self.color))

        for tick in self.ticks:
            d.append(draw.Lines(self.a + tick, 0, self.a + tick, stroke='red'))

        if self.label:
            d.append(draw.Text(self.label, h, (self.b + self.a) / 2, h * 2,
                               center=True))

        if 'f' in self.direction:
            d.append(draw.Lines(self.b, 0, self.b + 5, h / 2, self.b, h,
                                fill=self.color, stroke=self.color))
        if 'r' in self.direction:
            d.append(draw.Lines(self.a, 0, self.a - 5, h / 2, self.a, h,
                                fill=self.color, stroke=self.color))

        for a, b, color in self.regions:
            d.append(draw.Rectangle(a, 0, b - a, h, fill=color, stroke=color))

        return d


class Label:
    """Wrap a text label
    """
    def __init__(self, text, style=None):
        pass


class Alignment:
    """Link two tracks to illustrate similar regions
    """
    def __init__(self, track1, track2, connections, text=None, style=None):
        self.t1 = track1
        self.t2 = track2
        self.connections = connections

    def draw(self, x=0, y=0, h=10, gap=50):
        g = draw.Group(transform="translate({} {})".format(x, y))
        g.append(self.t1.draw(x=0, y=0))
        g.append(self.t2.draw(x=0, y=-gap))

        for bottom, top in self.connections:
            g.append(draw.Lines(bottom, h, top, gap, stroke='black'))
            g.append(draw.Lines(bottom, 0, bottom, h, stroke='black'))
            g.append(draw.Lines(top, gap, top, gap + h, stroke='black'))
        return g

class Multitrack:
    """Pack multiple tracks onto a line
    """
    def __init__(self, tracks, join=False):
        self.tracks = tracks
        self.join = join

    def draw(self, x=0, y=0, h=10):
        g = draw.Group(transform="translate({} {})".format(x, y))
        if self.join:
            start = min([t.a for t in self.tracks])
            end = max([t.b for t in self.tracks])
            g.append(draw.Lines(start, h / 2, end, h / 2, stroke='lightgrey'))
        for track in self.tracks:
            g.append(track.draw())

        return g

class Tick:
    """
    wrapper for tick
    """
    def __init__(self, pos, color='red'):
        self.pos = pos
