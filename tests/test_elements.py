"""Unit tests for genetracks. Run `pytest` in the root directory."""
from genetracks.elements import Figure, Track, Alignment, Multitrack, Label
from genetracks.plasmid import Plasmid, Region
from genetracks.colors import SvgColor, HexColor

# pylint: disable=invalid-name


def test_add_track() -> None:
    """Test adding tracks to a figure"""
    figure: Figure = Figure()
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
    drawing = figure.show()
    assert drawing.width == 410.0
    assert drawing.height == 45.0


def test_multitrack() -> None:
    """Test adding tracks to a multitrack"""
    figure = Figure()
    for i in range(0, 10):
        figure.add(
            Multitrack(
                [
                    Track(i, i + 10, direction="f", label=f"Track {i}F"),
                    Track(i + 20, i + 30, direction="r", label=f"Track {i}R"),
                ],
                join=True,
            )
        )
    drawing = figure.show()
    assert drawing.width == 39.0
    assert drawing.height == 205.0


def test_multitrack_directions() -> None:
    """Test drawing direction arrows"""
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
            color=SvgColor.new("salmon"),
            regions=[(75, 225, "lightgrey")],
        )
    )
    figure.add(Track(50, 300, label="Reads overlap"))
    drawing = figure.show()
    assert drawing.width == 350.0
    assert drawing.height == 65.0


def test_alignment() -> None:
    """Test drawing alignment between tracks"""
    f = Figure()
    f.add(
        Alignment(
            Track(50, 310, direction="r", regions=[(110, 300, "lightblue")]),
            Track(100, 360, direction="f", regions=[(110, 300, "salmon")]),
            [(110, 300), (300, 110)],
        )
    )
    drawing = f.show()
    assert drawing.width == 365.0
    assert drawing.height == 65.0


def test_labels() -> None:
    """Test drawing labels on tracks"""

    def draw_hiv_genes(f: Figure) -> Figure:
        """HIV gene regions"""
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
                        Track(
                            l,
                            r,
                            label=Label(0, text, offset=1),
                            color=SvgColor.new(color),
                        )
                        for l, r, text, color in reading_frame
                    ]
                ),
                gap=0,
            )
        return f

    f = draw_hiv_genes(Figure())

    unscaled = f.show()
    assert unscaled.width == 9719.0
    assert unscaled.height == 35.0

    drawing = f.show(w=900)
    assert drawing.width == 900.0
    assert drawing.height == 35.0


def test_circular() -> None:
    """Test circular genome figures"""
    p: Plasmid = Plasmid(
        360,
        regions=[
            Region(100, 101, color=SvgColor.new("orange")),
            Region(110, 280, color=SvgColor.new("salmon")),
            Region(230, 275, color=SvgColor.new("firebrick")),
            Region(320, 20, color=SvgColor.new("lightblue")),
            Region(20, 50, color=SvgColor.new("slateblue")),
        ],
    )
    drawing = p.show()
    # surely this is wrong?
    assert drawing.width == 1.4
    assert drawing.height == 1.4


def test_colors() -> None:
    """Test Hex color constructor"""
    c: HexColor = HexColor("#AB2201")
    c.g = 10000
    assert f"{c}" == "#ABFF01FF"
    c.a = 16
    assert f"{c}" == "#ABFF0110"
    c.r = -100
    assert f"{c}" == "#00FF0110"
    c.b += 2
    assert f"{c}" == "#00FF0310"
