# gathering_data.md

### Status:

### context

we have variation in the data we want, we arent gonna find it in one place, we need to gather it from multiple sources,
and we need to make sure its accurate and up to date.

### plan

- largest city dataset
  found [here](https://hub.huwise.com/explore/assets/geonames-all-cities-with-a-population-1000/view/). contains
  timezone, coordinates, city name, alternate city name, country name. it need transfer phase to emit the boilerplate.
- salah crate supports fixed set of methods/madhabs. if turned out some countries use separate ways, we fall back into
  angle calculations

