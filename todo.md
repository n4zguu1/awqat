- [x] rework on the core types
- [x] reconstruct the dataset to contain the region
- [x] some calculations methods doesn require angles, instead it uses a fixed timestamp , like oumalqura , isha time is
  after 60min, in certain month (ramadan) it uses 120 min
- [x] a fallback mechanism for months like ramadan, to make the calandar accurate for user.
- [x] a propper DB indexing, cuz its over 150,664 rows, used FTS5
- [ ] the dataset i hve mightbe inaccurate for some people, if so, create a issue template for people to help identifying errors
- [ ] improve the database to contains more precise cities,
- [ ] add heuristics to find neighbor cities to select in search results
- [ ] in later version, we should support color pallets of the os, like btop where on omarchy, the ui adapts