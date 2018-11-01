import xml.etree.ElementTree as ET

class diagram:
    def __init__(self):
        self.polys = [
            [(0, 500), (10, 510), (0, 10), (500, 510)],]

    def draw(self):
        svg = ET.Element('svg', xmlns="http://www.w3.org/2000/svg",
                version="1.1")
        g = ET.SubElement(svg,"g",style="fill-opacity:1.0; stroke:black;")
        for item in self.items:
            item.to_svg(g)
        return ET.tostring(svg)

class track:
    def __init__(self, style='clothesline'):
        pass

class label:
    def __init__(self, text, style=None):
        pass

class alignment:
    def __init__(self, text, style=None):
        pass
