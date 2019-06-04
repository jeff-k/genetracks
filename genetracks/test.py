import unittest
from genetracks import Figure, Track, Alignment, Multitrack, Label

class TestFigureInit(unittest.TestCase):
    def test_add_track(self):
        figure = Figure()
        figure.add_track(Track(50, 300, direction='f', label="Another\
            sequence", regions=[(50, 100, 'lightblue')]))
        figure.add_track(Track(110, 410, direction='r', label="Sequence 1",\
            regions=[(150, 200, 'salmon')]))
        figure.show()

    def test_multitrack(self):
        figure = Figure()
        for i in range(0, 10):
            figure.add_track(Multitrack([
                Track(i, i+10, direction='f', label='Track {}F'.format(i)),
                Track(i+20, i+30, direction='r', label='Track {}R'.format(i))],
                join=True))
        figure.show()

if __name__ == '__main__':
    unittest.main()
