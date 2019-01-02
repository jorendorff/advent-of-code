When I went to solve this, I thought,
well, maybe part of the solution is to characterize the regions of overlap using inequalities.

There are four coordinates given by

    h = x + y + z
    i = x + y - z
    j = x - y + z
    k = x - y - z

corresponding to the four pairs of parallel faces on an octahedron.

Any bot's range can be represented as a 4-tuple of ranges in this space,
i.e. with 4 pairs of numbers.
Of course it's a little redundant to represent points in 3-space using 4 axes,
but bear with me.

Then computing intersections is boxlike: simply compute intersections
along the h, i, j, and k axes. I think every intersection of such
octahedrons can be represented as an "octobox" in this way.

Now there are a few problems left:

*   It's possible for four nonempty ranges to produce an empty
    "octobox". Any three nonempty ranges produces a parallelopiped, and
    then the fourth range can just select a slab of space that misses
    the parallelopiped. This is a problem because we're concerned with
    whether or not regions overlap, i.e. whether they're empty, and
    that's not simple to compute.

*   Once we have computed the final "octobox", convert back to 3d
    coordinates.

*   Determine which octohedrons to intersect to produce the final
    region.

I did all three of these in the jankiest possible way.

In particular, the last task,
deciding which set of octohedrons was best, turned out to be easy:
the puzzle input was quite special.

The vast majority of the input bot ranges all overlap exactly each other;
that is, for some largeish number N,
the top N regions that overlap the greatest number of other regions
all overlap each other, and some of them overlap *only* the others in that set,
so it's clear there's no way to grow the set.
And it turns out the region of overlap does contain a grid point.
This made things easy.

It's a good thing. I think the general problem might be NP-complete...
