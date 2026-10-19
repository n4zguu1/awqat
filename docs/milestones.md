# Roadmap

1. **v1**
   * display times and dates in both hijri and gregorian
   * configuration file
   * location selection
   * next prayer
   * support all methods and madhabs
   * support both terminals
2. **v2**
   * scrollable times table
   * notification
   * weather integration
   * hyperland trays
3. **v3**
   * Islamic events
   * azkar notifications
   * sun angle ui
   * automatic hijri date updates
   * multilingual support (LTR langs)

## Milestones

### M0 — Gather data

Collect and normalize the city, country, timezone, and calculation-method data
required by the application.

### M1 — Core library

Deliver the calculations and data access required by the application:

- Prayer times for a day and a month.
- The next prayer for the current or following day.
- Hijri and Gregorian dates.
- Local time.
- Country, city, and timezone information.

### M2 — TUI

Deliver the terminal interface and its integration with the core library.

### M3 — Notifications

Deliver the notification daemon and its configuration.

### M4 — Continuous delivery and updates

Deliver release automation and the application update layer.

### Future — Islamic events

Add Islamic events and related calendar features.
