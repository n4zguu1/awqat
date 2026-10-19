# Architecture

## Data model

The bundled database contains the following logical tables.

### `cities`

| Column | Description |
| --- | --- |
| `id` | City identifier (primary key). |
| `name` | City name. |
| `alt_name` | Alternative city names. |
| `tz` | IANA timezone identifier. |
| `country` | ISO 3166-1 numeric country code (foreign key). |
| `coordinates` | Latitude and longitude. |

### `countries`

| Column | Description |
| --- | --- |
| `id` | ISO 3166-1 numeric country code (primary key). |
| `name` | Country name. |
| `default_madhab` | Default madhab, such as Shafi'i. |
| `default_method_id` | Default calculation method (foreign key). |
| `high_latitude_rule` | Default high-latitude rule, such as `middle_of_night`. |

### `methods`

| Column | Description |
| --- | --- |
| `id` | Calculation method identifier. |
| `name` | Method name. |
| `fajr_angle` | Fajr angle in degrees. |
| `isha_rule` | Whether Isha uses an angle or a fixed offset. |
| `isha_value` | Angle or offset value, depending on `isha_rule`. |
| `ramadan_offset` | Ramadan-specific Isha offset. A value of zero uses the regular offset. |

## Codebase

The codebase is organized into two primary areas:

- `core/` — connections, calculations, embedded data, and input/output.
- `tui/` — terminal user interface code.
