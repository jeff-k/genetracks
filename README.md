# genetracks
Generate track diagrams for genomic alignments

see [examples/Genetracks.ipynb](examples/Genetracks.ipynb) for use cases

## Example

```python
from genetracks import Figure, Track
import pysam
import sys

f = Figure()

for alignment in pysam.AlignmentFile(sys.argv[1]):
  f.add(Track(alignment.query_start, alignment.query_end))

f.show(w=900).save_svg("alignment.svg")
```

## JSON serialisation

...
