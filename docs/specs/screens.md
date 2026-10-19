# main screen

## goal

what screen do we have

> note: in the first version we dont need to have beautiful ui. we care to deliver as much feature as we can, then UI is
> another last step.

### main screen

- wall clock bases on timezone
- prayer all five prayer times highlighting the next prayer
- remaining time to next prayer
- location information
- Hijri and Gregorian dates
- calculation methods
- change configuration access

### setups screen

- screen for user to input his location data
  - input elements, user types, a table bellow shows the matched results

### error screen

- displays any kind of error that may happen in the program, like location not found, or configuration file not found,
  or any other error.

### Table view screen

- a scrollable table area where use can see full view of the prayer times -> dynamic loading mechanism considered (lazy
  load)

### settings screens

- for user to change: language (only LTR)/location/madhab-method/prayer times tweaks for notifs
