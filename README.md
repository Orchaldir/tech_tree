# Tech Tree Visualizer

![fmt & clippy](https://github.com/Orchaldir/tech_tree/actions/workflows/check-and-lint.yaml/badge.svg)
![tests](https://github.com/Orchaldir/tech_tree/actions/workflows/test.yaml/badge.svg)
[![codecov](https://codecov.io/gh/Orchaldir/tech_tree/branch/main/graph/badge.svg?token=SLIHSUWHT2)](https://codecov.io/gh/Orchaldir/tech_tree)

This project allows the user to define tech trees in yaml files and visualize them as SVG.

An example tech tree can be found in [example.yaml](resources/example.yaml).
It can be visualized with: 

```terminal
tech_tree_cli resources/example.yaml
```

The output is an SVG image:

![SVG Image](resources/example.svg)