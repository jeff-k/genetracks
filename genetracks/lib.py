class Colour:
    pass

class Figure:
    def __init__(self):
        self.tracks: list[Track] = []

    def to_svg(self) -> Xml:
        pass

class Track:
    def __init__(self):
        self.elems: list[Elem] = []

    def draw(self, row: int, scale: float) -> str:
        s = []
        s.append(f"<g transform=translate(0 {row})")
        for elem in self.elems:
            s.append(elem.draw(scale, self.height))
        s.append(f"</g>")
        return "\n".join(s)

class Elem:
    def __init__(self, start, length, colour, label=None):
        self.start: float = start
        self.length: float = length
        self.colour: Colour = colour
        self.label: Option[str] = label

    def draw(self, scale: float, height: float) -> str:
        end: float = self.length / scale
        midpoint: float = self.length / 2.0 / scale
        start: float = self.start / scale
        s.append(f"<g transform=translate({start} 0)>")
        s += self._geom(start, midpoint, end, height, colour)
        if self.label:
            s.append(f'<text x={midpoint} y={height / 2.0} font-size="12" font-family="monospace" text-anchor="middle" dominant-baseline="middle">{self.label}</text>')
        s.append(f"</g>")
        return "".join(s)

    def _geom(start, midpoint, end, height, colour) -> list[str]:
        return []

class Rect(Elem):
    def _geom(start, midpoint, end, height, colour) -> list[str]:
        return [f'<path d="M0.0,0.0 L0.0,{height}" stroke={colour} />',]

class Line(Elem):
    pass

class Bar(Elem):
    pass

class Left(Elem):
    pass

class Right(Elem):
    pass
