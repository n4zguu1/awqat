# Prayer times

## Goal

Show today's prayer times and identify the next upcoming prayer.

## Background

Prayer times are based on the Sun's elevation above the local horizon, ranging
from 0° at sunrise or sunset to 90° when directly overhead. The calculation
uses four core inputs:

- Coordinates: latitude and longitude.
- Solar declination: the angle between the Sun's rays and the Earth's
  equatorial plane, which changes daily.
- Equation of time: an offset that accounts for the Earth's orbital eccentricity
  and axial tilt.
- Calculation-method conventions: regional choices for twilight angles and
  fixed offsets.

Different authorities use different twilight angles, generally between 12° and
19.5°, or fixed offsets. Common conventions include the Muslim World League,
Umm al-Qura, the Egyptian General Authority, and ISNA.

## Special cases

### Extreme latitudes

At high latitudes, the Sun may not reach the angle required for Fajr or Isha.
The corresponding trigonometric solution is then outside the valid range for
`acos`. Calculation engines can use fallback rules:

- **Nearest latitude** — Clamp the calculation to a reference latitude such as
  45° or 48°.
- **Middle of the night** — Set Fajr and Isha relative to the midpoint of the
  night.
- **One-seventh of the night** — Allocate the first and last seventh of the
  night to Isha and Fajr.
- **Nearest city** — Use a timetable from the nearest city where the angles
  remain valid.

### Altitude and refraction

At high altitudes, the visible horizon is lower than at sea level. Standard
algorithms account for this with a horizon adjustment of approximately -0.833°,
including solar semi-diameter and atmospheric refraction.

### Fixed offsets

Some methods define Isha as a fixed interval after Maghrib rather than an
angle. For example, Umm al-Qura uses 90 minutes after Maghrib, extended to
120 minutes during Ramadan. Authorities may also apply safety buffers of two
to five minutes to Dhuhr or Maghrib.

## Calculation

The `salah` crate performs most of the astronomical calculations.

### Inputs

- Latitude and longitude.
- Madhab.
- Calculation method.
- High-latitude rule.
- Isha interval, when applicable.
- Timezone.

### Outputs

- Fajr.
- Sunrise.
- Dhuhr.
- Asr.
- Maghrib.
- Isha.
- The next prayer and remaining time.
