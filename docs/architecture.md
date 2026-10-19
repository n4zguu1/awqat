## data model

### Tables

| Table       | row                  | descirption                                                                          |
|-------------|----------------------|--------------------------------------------------------------------------------------|
| **cities**  |                      |                                                                                      |
| 0           | `id`                 | city id (PK)                                                                         |
| 1           | `name`               | name of the city                                                                     |
| 2           | `alt_name`           | other names of the city                                                              |
| 3           | `tz`                 | timezone                                                                             |
| 4           | `country`            | country id ISO 3166-1 numeric code (FK)                                              |
| 5           | `coordinates`        | lat and lon                                                                          |
| **country** |                      |                                                                                      |
| 0           | `id`                 | country ISO 3166-1 numeric code (PK)                                                 |
| 1           | `country`            | country name                                                                         |
| 2           | `default_madhab`     | e.g., shafi                                                                          |
| 3           | `default_method_id`  | methods.id                                                                           |
| 4           | `high_latitude_rule` | e.g., middle_of_night                                                                |
| **method**  |                      | ishaa_offset                                                                         |
| 0           | `id`                 | id                                                                                   |
| 1           | `name`               | method name                                                                          |
| 2           | `fajr_angle`         | fajr angle                                                                           |
| 3           | `ishaa_rule`         | ishaa rules (by angle or by offset)                                                  |
| 5           | `ishaa_rule`         | angle/offset                                                                         |
| 4           | `ishaa_value`        | either angle value or offset value                                                   |
| 5           | `ramadan_offset`     | if zero we follow same issha value. or we change ishaa offset in this specific month |

### core lib apis: data that need to expose to interface

- prayer times: day/month
- next prayer time: same day/next day
- hijri date/gregorian date
- time
- country/city/