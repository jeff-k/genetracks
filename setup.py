import setuptools

with open("README.md", 'r') as fh:
    long_description = fh.read()

setuptools.setup(
    name="genetracks",
    version="0.0.1",
    author="jeff-k",
    author_email="jeff_k@fastmail.com",
    description="Generate track diagrams for genomic alignments",
    url="https://github.com/jeff-k/genetracks",
    packages=setuptools.find_packages(),
        #classifiers = [],
    install_requires=['drawSvg',],
)
