# Algorithm
*This document assumes that you have a basic understanding of how nonograms work and how to solve them.*

## Overview
Each cell of a nonogram can either be a box (filled) a space ("X") or empty.
For each vertical and horizontal line we have a list of numbers.
I will refer to these numbers as chains (of boxes).

The goal is to narrow down the possible locations of each chain until we can draw conclusions.
Let's look at an example:
We have one chain with a length of four inside the given range.

![](OverviewExample.svg)

We can say for sure that the two cells outside the range must be spaces.
Also, as the chain has a length of four, regardless how far to the left or right the chain is, the center will always be covered.

![](OverviewSolution.svg)