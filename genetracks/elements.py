import drawSvg as draw

class Figure:
    def __init__(self, tracks=[]):
        self.tracks = tracks
        right = 0
        left = 1000

        for track in self.tracks:
            (a, b) = track.interval
            left = min(a, left)
            right = max(b, right)

        self.height = len(tracks) * 20
        self.width = right


    def to_svg(self, g):
        pass

    def add_track(self, track):
        self.tracks.append(track)

    def show(self):
        d = draw.Drawing(self.width, self.height, origin='center')

        for i, track in enumerate(self.tracks):
            d.append(draw.Rectangle(i * 10,
                (i * 10) + 10,
                track.width,
                10))
        return d

class Track(Figure):
    def __init__(self, a, b, label=None, style='clothesline'):
        self.interval = (a, b)
        self.width = b - a

class Label(Figure):
    def __init__(self, text, style=None):
        pass

class Alignment(Figure):
    def __init__(self, text, style=None):
        pass
