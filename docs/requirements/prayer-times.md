# Prayer times

## goal

shows today's prayer times and identify the next upcoming prayer

## research

the core factor that define a prayer times is solar elevation in angle (the angular height of the sun above the local
horizon, ranging from 0° at sunrise or sunset to 90° when directly overhead).
so to calculate this programmatically, algorithms take four core inputs:

- coordinates: latitude, and longitude
- Declination of the Sun ($\delta$): The angle between the rays of the Sun and the plane of the Earth's equator (changes
  daily).
- Equation of Time ($EoT$): An offset that accounts for the eccentricity of Earth's orbit and tilt, compensating for the
  difference between solar time and standard clock time.
- Calculation Method Conventions: Different regions

> **why there is calculation methods**
> The different methods exist because Islamic authorities digitized classical visual observations of twilight into fixed
> solar angles ($12^\circ$ to $19.5^\circ$) to provide consistent, automated schedules that account for regional
> atmospheric baselines and latitude differences.
>
> ---
>
> Each country or regional ministry adopts a specific astronomical parameter—either designing its own official system or
> aligning with major global standards (such as the *Muslim World League*, *Umm al-Qura*, *Egyptian General Authority*,
> or
> *ISNA*).
>
> ---
>
> **special cases**
>
> 1. **Extreme High Latitudes (Polar & Near-Polar Regions):**
>    * **The Problem:** Above $\approx 48^\circ$ N/S in summer, the sun stays close to the horizon all night and never
       drops below the required $15^\circ$ or $18^\circ$ threshold. The trigonometric
       term $\cos H = \frac{\sin (-\alpha) - \sin\phi \cdot \sin\delta}{\cos\phi \cdot \cos\delta}$ results in a value
       outside $[-1, 1]$, making $\arccos$ mathematically undefined (no real solution).
>    * **The Solution:** Calculation engines switch to rule-based fallback approximations:
>      * **Nearest Latitude:** Clamps the latitude calculation to a reference parallel (e.g., $45^\circ$ or $48^\circ$).
>      * **Middle of the Night ($Nisf\ al-Layl$):** Sets the end of Isha and start of Fajr at the exact midpoint between
         Sunset and Sunrise.
>      * **One-Seventh of the Night ($Sub'a\ al-Layl$):** Allocates the final $1/7^{\text{th}}$ of total nighttime for
         Fajr and the first $1/7^{\text{th}}$ for Isha.
>      * **Nearest City:** Uses the exact prayer timetable of the closest city where solar angles remain valid.
>
> 2. **High Altitude & Atmospheric Refraction:**
>    * **Elevation Differences:** At high altitudes (e.g., mountains or tall structures like the Abraj Al Bait in
       Makkah), the observer sees the horizon lower than someone at sea level (dip of the horizon). This shifts Sunset
       ($Maghrib$) and Sunrise ($Shorooq$) by several minutes.
>    * **Standard Refraction Allowance:** Standard algorithms apply a constant $-0.833^\circ$ adjustment at the horizon
       ($0.533^\circ$ for the sun’s semi-diameter + $0.30^\circ$ for atmospheric light bending at sea level).
>
> 3. **Fixed Minute Offsets (Custom Rule Models):**
>    * **Fixed Isha Window:** Standard angle formulas are overridden in certain systems. For instance, the **Umm
       al-Qura** authority defines Isha not by an angle, but as a fixed **90-minute offset** after Maghrib (extended to
       **120 minutes** during Ramadan).
>    * **Safety Buffers ($Ihtiyat$):** Many national ministries add $2$ to $5$ manual padding minutes to Dhuhr (to
       ensure the sun has completely passed the meridian) or Maghrib (to guarantee total solar immersion).

## calculation

salah crate handle most of the calculations for us

### inputs

- latitude
- longitude
- madhab
- method
- high latitude rule (solves the case nbr 1)
- isha interval
- timezone

### output

- fajr prayer time
- sunrise
- duhr
- asr
- maghrib
- ishaa
- next prayer and remaining time 

