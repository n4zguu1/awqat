# Calculate on Open vs. One Config File

## status: proposed

## context

do we save the calculated prayer times in a config file or do we calculate them every time the app opens.
we need to consider the trade-offs between performance, accuracy, and user experience.

## decision

### Option 1: Calculate on Open

- **Pros:**
  - Always provides the most accurate prayer times based on the current date and location.
  - No need to manage a config file for prayer times, reducing complexity.
  - no extra storage is needed for prayer times, as they are calculated on-the-fly.

- **Cons:**
  - Slightly slower app startup time, as calculations are performed each time the app opens.
  - Requires access to location services and date/time information every time the app is opened.

### Option 2: One Config File

- **Pros:**
  - Faster app startup time, as prayer times are read from a pre-calculated config file.
  - Reduces the need for repeated calculations, which can be beneficial for devices with limited processing power.
  - Can work offline if the config file is already present.
- **Cons:**
  - Prayer times may become outdated if the config file is not updated regularly.
  - transition between summer and winter time may not be reflected if the config file is not handled properly.
  - extra i/o operations.

decision: We will go with **Option 1: Calculate on Open**. due to high speed modern devices, the performance impact is
negligible. 