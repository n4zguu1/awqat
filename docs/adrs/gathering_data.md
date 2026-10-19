# Gather data from multiple sources

- **Status:** Proposed

## Context

The required city and calculation data is not available from one source.
Sources must be combined, normalized, and checked for accuracy.

## Plan

- Start with a large city dataset such as
  [GeoNames cities with a population over 1,000](https://hub.huwise.com/explore/assets/geonames-all-cities-with-a-population-1000/view/).
- Import timezone, coordinates, city names, alternative names, regions, and
  country information into the bundled dataset.
- Use the methods and madhabs supported by the `salah` crate.
- Fall back to angle-based calculations when a country's convention is not
  directly represented.
