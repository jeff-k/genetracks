"""Basic drawing elements for gene track figures
"""
import drawSvg as draw

class Figure:
    """Genetracks Figure
    """
    def __init__(self):
        self.padding = 10
        self.height = 0
        self.track_height = 10
        self.elements = []
        self.x_max = 0
        self.tracks = 0

    def to_svg(self, g):
        pass

    def add_track(self, track, gap=30, h=None):
        h = self.track_height
        #track.set_y(-(self.tracks * gap))
        track.y = -(self.tracks * gap)
        self.elements.append(track)
        self.height += track.h + gap
        self.x_max = max(self.x_max, track.b)
        self.tracks += 1

    def add_alignment(self, alignment):
        self.height += alignment.h
        self.x_max = max(self.x_max, alignment.b)
        self.elements.append(alignment)

    def add_coverage(self, coverage, gap=30, h=None, offset=0):
        h = self.track_height
        self.elements.append(coverage)
        self.height += coverage.h
        self.tracks += 1

    def show(self, w=None, h=None):
        width = self.x_max

        xscale=1.0
        if w is None:
            w = width
        else:
            xscale = w / self.x_max

        if h is None:
            h = self.height

        d = draw.Drawing(width * xscale, self.height, origin=(0,0))
        for element in self.elements:
            d.append(element.draw(xscale=xscale))

#        d.setRenderSize(w, h)
        return d

    def to_png(self, path):
        self.show().savePng(path)

class Element:
    """Baseclass for drawable element
    """
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.h = 10 
        self.x_max = x 

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

    def draw(self, x=0, y=0, xscale=1.0):
        h = self.h
        y = self.y
        a = self.a * xscale
        b = self.b * xscale
        x = x * xscale
        
        #assert isinstance(x, int) and isinstance(y, int)
        d = draw.Group(transform="translate({} {})".format(x, y))
        d.append(draw.Rectangle(a, 0, b - a, h,
                                fill=self.color, stroke=self.color))

        if 'f' in self.direction:
            d.append(draw.Lines(b, 0, b + (5 * xscale), h / 2, b, h,
                                fill=self.color, stroke=self.color))
        if 'r' in self.direction:
            d.append(draw.Lines(a, 0, a - 5, h / 2, a, h,
                                fill=self.color, stroke=self.color))

        for r_a, r_b, color in self.regions:
            r_a = r_a * xscale
            r_b = r_b * xscale
            d.append(draw.Rectangle(r_a, 0, r_b - r_a, h, fill=color, stroke=color))

        for tick in self.ticks:
            tick = tick * xscale
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

            d.append(draw.Text(label, font_size, (b + a) / 2,
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
        #assert isinstance(x, int) and isinstance(y, int)
        d = draw.Group(transform="translate({} {})".format(x, y))

        for i, y in enumerate(self.ys):
            d.append(draw.Rectangle(self.a + (i * self.b), 0, self.b, y,
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
    def __init__(self, track1, track2, connections, text=None, style=None,
            gap=50):
        self.t1 = track1
        self.t2 = track2
        self.connections = connections
        self.h = track1.h + track2.h + gap
        self.b = max(track1.b, track2.b)

    def draw(self, x=0, y=0, h=10, gap=50, xscale=1.0):
        g = draw.Group(transform="translate({} {})".format(x, y))
        g.append(self.t1.draw(x=0, y=0, xscale=xscale))
        g.append(self.t2.draw(x=0, y=-gap, xscale=xscale))

        for bottom, top in self.connections:
            bottom = bottom * xscale
            top = top * xscale
            g.append(draw.Lines(bottom, h, top, gap, stroke='black'))
            g.append(draw.Lines(bottom, 0, bottom, h, stroke='black'))
            g.append(draw.Lines(top, gap, top, gap + h, stroke='black'))
        return g


class Multitrack(Element):
    """Pack multiple tracks onto a line
    """
    def __init__(self, tracks, h=10, join=False):
        self.tracks = tracks
        self.join = join
        self.y = 0
        self.h = h 
        self.b = max(map(lambda x: x.b, tracks))

    def set_y(self, y):
        self.y = y

    def set_h(self, h):
        self.height = h

    def draw(self, x=0, y=0, xscale=1.0):
        y = self.y
        h = self.h
        #assert isinstance(x, int) and isinstance(y, int)
        g = draw.Group(transform="translate({} {})".format(x, y))
        if self.join:
            start = min([t.a for t in self.tracks]) * xscale
            end = max([t.b for t in self.tracks]) * xscale
            g.append(draw.Lines(start, h / 2, end, h / 2, stroke='lightgrey'))
        for track in self.tracks:
            g.append(track.draw(xscale=xscale))

        return g


class Tick:
    """
    wrapper for tick
    """
    def __init__(self, pos, color='red'):
        self.pos = pos
