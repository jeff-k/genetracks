"""
basic elements of gene track figures
"""
import drawSvg as draw

class Figure:
    """
    figure class
    """
    def __init__(self, tracks=[], height=50, width=700, size=1000):
        self.padding = 10
        self.tracks = tracks
        self.height = height
        self.size = size
        self.track_height = 10
        self.width = width # horizontal scaling

    def to_svg(self, g):
        pass

    def add_track(self, track):
        self.tracks.append(track)

    def draw(self, i, j, alignments=[]):
        top = self.height
        d = draw.Drawing(self.size, top, origin=(0, 0))
        h = self.track_height

        
        d.append(i.draw(h=h, w=self.width))
            
        d.append(j.draw(h=h, w=self.width))

###        for aln in alignments:
  #          for e in aln.draw(h=h, w=self.width):
  #              d.append(e)

        d.setRenderSize(self.size)
        return d

    def to_png(self, d, path):
        d.savePng(path)

class Track:
    """
    individual gene track
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

    def draw(self, h=10, w=700):
        #r = (self.b + x) / w
        #self.x = x
        #self.y = y

        d = draw.Group()
        d.append(draw.Rectangle(self.a, 0, self.b - self.a, h, fill=self.color,
            stroke=self.color))

        for tick in self.ticks:
            d.append(draw.Lines(self.a + tick, 0, self.a + tick,  stroke='red'))

        if self.label:
            d.append(draw.Text(self.label, h, (self.b + self.a) / 2, h + 2))

        if 'f' in self.direction:
            d.append(draw.Lines(self.b, 0, self.b + 5, h / 2, self.b, h,
                fill=self.color, stroke=self.color))
        if 'r' in self.direction:
            d.append(draw.Lines(self.a, 0, self.a - 5, h / 2, self.a, h,
                fill=self.color, stroke=self.color))

        for a, b, color in self.regions:
            d.append(draw.Rectangle(a, 0, b - a, h, fill=color))

        return d


class Label:
    """
    label wrapper
    """
    def __init__(self, text, style=None):
        pass


class Alignment:
    """
    expressing the alignment of two tracks
    """
    def __init__(self, track1, track2, connections, text=None, style=None):
        self.t1 = track1
        self.t2 = track2
        self.connections = connections

#    def add_alignment(self, a, b):
#        pass

    def draw(self, h=10, w=1):
        d = []
        for bottom, top in self.connections:
            d.append(draw.Lines(w / bottom, self.t1.y + h,
                                w / top, self.t2.y, stroke='black'))
            d.append(draw.Lines(w / bottom, self.t1.y,
                                w / bottom, self.t1.y + h, stroke='black'))
            d.append(draw.Lines(w / top, self.t2.y,
                                w / top, self.t2.y + h, stroke='black'))
        return d


class Tick:
    """
    wrapper for tick
    """
    def __init__(self, pos, color='red'):
        self.pos = pos
