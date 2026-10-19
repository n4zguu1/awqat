# Awqat

Awqat is a terminal user interface (TUI) application that helps Muslims track
prayer times while working at a computer.

## Goals and principles

- Calculate prayer times locally.
- Keep configuration recoverable.
- Work offline by default.

## Supported platforms

- Linux
- Windows

## Planned features

### Core experience

- Display Gregorian and Hijri dates.
- Display the day's prayer times and the next prayer.
- Select a city and show its location.
- Support configurable calculation methods and madhabs.
- Support 12-hour and 24-hour time formats.

### Convenience features

- Show prayer times for Makkah, Madinah, and Al-Quds.
- Provide a scrollable prayer table with a way to return to today.
- Display Islamic events.
- Provide theme selection and system-theme adaptation.
- Support keyboard shortcuts and multiple left-to-right languages.
- Display weather information.

### Integrations

- Provide notifications on Linux and Windows where supported.
- Provide a Hyprland tray integration showing the time remaining until the next prayer.
- Include an animated sun view.

## User flow

1. **Install** — The user chooses an automated setup script or manual download
   and compilation.
2. **Set up** — The first launch opens a setup screen where the user selects a
   city and confirms the date.
3. **Configure** — The user can change the city, Hijri correction, language,
   notification settings, and prayer-time preferences.
4. **Use** — Subsequent launches open directly on the main screen using the
   saved configuration.
5. **Quit** — The user can exit the application normally.

## Open questions

- Which data sources provide reliable Hijri dates?
- Should the date browser be finite or effectively infinite?
- How should automatic Hijri updates be scheduled?
- How should regional variations, daylight saving time, and duplicated city
  names be represented?
- Which high-latitude rules should be available?
- How should Windows key press, release, and hold behavior be handled?
- Which languages can be supported reliably by terminal emulators?
- Is continuous delivery worth the maintenance cost?

## Audience and distribution

Awqat targets people who spend time in a terminal, especially Linux users.
Windows users may need additional onboarding because TUIs are less familiar on
that platform.

Potential distribution channels include Muslim technology communities on
Reddit and Facebook, project announcement accounts, and Arabic technology news
pages.