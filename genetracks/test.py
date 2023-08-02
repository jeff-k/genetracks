import unittest
from genetracks import Figure, Track, Alignment, Multitrack, Label, Plasmid, Region


class TestFigureInit(unittest.TestCase):
    def test_add_track(self):
        figure = Figure()
        figure.add(
            Track(
                50,
                300,
                direction="f",
                label="Another\
            sequence",
                regions=[(50, 100, "lightblue")],
            )
        )
        figure.add(
            Track(
                110,
                410,
                direction="r",
                label="Sequence 1",
                regions=[(150, 200, "salmon")],
            )
        )
        figure.show()

    def test_multitrack(self):
        figure = Figure()
        for i in range(0, 10):
            figure.add(
                Multitrack(
                    [
                        Track(i, i + 10, direction="f", label="Track {}F".format(i)),
                        Track(
                            i + 20, i + 30, direction="r", label="Track {}R".format(i)
                        ),
                    ],
                    join=True,
                )
            )
        figure.show()

    def test_multitrack_directions(self):
        figure = Figure()
        figure.add(
            Multitrack(
                [
                    Track(0, 150, direction="f", label="Forward read"),
                    Track(200, 350, direction="r", label="Reverse read"),
                ],
                join=True,
            )
        )
        figure.add(
            Track(
                50,
                250,
                direction="fr",
                label="Read-through",
                color="salmon",
                regions=[(75, 225, "lightgrey")],
            )
        )
        figure.add(Track(50, 300, label="Reads overlap"))
        figure.show()

    def test_alignment(self):
        f = Figure()
        f.add(
            Alignment(
                Track(50, 310, direction="r", regions=[(110, 300, "lightblue")]),
                Track(100, 360, direction="f", regions=[(110, 300, "salmon")]),
                [(110, 300), (300, 110)],
            )
        )
        f.show()

    def test_labels(self):
        f = Figure()

        def draw_hiv_genes(f):
            third = [
                (2085, 5096, "pol", "orange"),
                (5559, 5850, "vpr", "turquoise"),
                (5970, 6045, "rev", "yellowgreen"),
                (6225, 8795, "env", "salmon"),
            ]
            second = [
                (5831, 6045, "tat", "plum"),
                (6062, 6310, "vpu", "red"),
                (8379, 8653, "rev", "yellowgreen"),
                (9086, 9719, "3' LTR", "darkgrey"),
            ]

            first = [
                (0, 634, "5' LTR", "darkgrey"),
                (790, 2292, "gag", "lightblue"),
                (5041, 5619, "vif", "steelblue"),
                (8379, 8469, "tat", "plum"),
                (8797, 9417, "nef", "mediumaquamarine"),
            ]

            for reading_frame in [first, second, third]:
                f.add(
                    Multitrack(
                        [
                            Track(l, r, label=Label(0, text, offset=1), color=color)
                            for l, r, text, color in reading_frame
                        ]
                    ),
                    gap=0,
                )

        draw_hiv_genes(f)
        f.show(w=900)

    def test_circular(self):
        p = Plasmid(
            360,
            regions=[
                Region(100, 101, color="orange"),
                Region(110, 280, color="salmon"),
                Region(230, 275, color="firebrick"),
                Region(320, 20, color="lightblue"),
                Region(20, 50, color="slateblue"),
            ],
        )
        p.show()


if __name__ == "__main__":
    unittest.main()
