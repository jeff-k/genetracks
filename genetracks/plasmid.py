import drawSvg as draw

class Plasmid:
    def __init__(self, length, regions=[], size=400, color='lightgrey', title=""):
        self.size = size
        self.regions = regions
        self.length = length
        self.color = color

    def show(self):
        d = draw.Drawing(1.4, 1.4, origin='center')

        d.append(draw.Circle(0,0,0.5, stroke_width='0.005', stroke=self.color,
            fill_opacity='0.0'))

        for region in self.regions:
            d.append(region.draw(self.length))

#        d.append(draw.Text("asdf"))
        d.setRenderSize(self.size)
        return d

class Region:
    def __init__(self, a, b, color='lightgrey'):
        self.color = color
        self.a = a
        self.b = b

    def draw(self, circ):
        return draw.Arc(0,0,0.5,self.a, self.b, stroke_width='0.04',
                stroke=self.color, fill_opacity='0.0')
