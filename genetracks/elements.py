import xml.etree.ElementTree as ET

class Figure:
    def __init__(self, items=[]):
        self.items = items

    def to_svg(self, g):
        pass

    def show(self):
        svg = ET.Element('svg', xmlns="http://www.w3.org/2000/svg",
                version="1.1")
        g = ET.SubElement(svg,"g",style="fill-opacity:1.0; stroke:black;")
        for item in self.items:
            item.to_svg(g)
        return ET.tostring(svg)

class Box(Figure):
    def __init__(self):
        self.polys = [
            [(0, 500), (10, 510), (0, 10), (500, 510)],]

class Track(Figure):
    def __init__(self, label=None, style='clothesline'):
        pass

class Label(Figure):
    def __init__(self, text, style=None):
        pass

class Alignment(Figure):
    def __init__(self, text, style=None):
        pass
