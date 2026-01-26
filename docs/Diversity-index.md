# Diversity index

We use the Gini-Simpson index to calculate the tree species diversity.  It produces a value from 0 to 1, where 0 is monoculture (bad) and 1 is infinite diversity (good).  The API endpoint to retrieve the index is:

## Getting the data

```
GET /v1/stats/diversity
```

## Interpreting the data

The response is a floating point value between 0 and 1.  The scale is more or less this:

- 0.00-0.50: critical.  A green desert, usually >50% of one species, exteme disease risk.
- 0.50-0.70: caution.  Two or three species dominate, significant pest risk.
- 0.70-0.90: good.  Likely 1 or 2 species are slightly overplanted (15-20%), but the canopy is safe.
- 0.90-1.00: ideal.  Santamour Compliant. No species exceeds 10%. (Extremely rare in real cities).


## Sources

- [Gini-Simpson Index][1] in Wikipedia
- [Diversitree by Senseable City Lab][2]

[1]: https://en.wikipedia.org/wiki/Diversity_index#Gini%E2%80%93Simpson_index
[2]: https://senseable.mit.edu/diversitree/
