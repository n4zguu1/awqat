# end user features

- display time and date in both Gregorian and Hijri
- next prayer
- day prayers and time
- display times in mecca and medina and quds
- support notifications in linux
- option to display scrollable prayers in table
    - also have some functionality like back to today if user scrolled too much.
- select city
- interactive animated view of the sun
- since hijri dates are moon based, user can also update hijri date based on scheduled update option
- display islamic special dates
- parameters for changing 24-hour and 12-hour displaying or just a shortcut
- supported on all terminals (windows and linux)
- select theme, or adapt to system or default to emulator theme
- shortcuts of mostly used functions
- multilingual
- display weather information,
- hyperland trays, displays the next prayer remaining time

# user flow

- installation
  user gets the choice for automated setup script, or download and compile manually.
- setup
  user runs the app by typing awqat in terminal, a setup screen opens for selecting city and confirms the date. clicks
  okay the app displays the times and content
- configure
  if an automated script failed from fetching and updating hijri date, a popup shows for making user update the hijri
  based on his local calculations.
  user can update manually the date or city.
  user can stop/start notification from app, also set enable/disable notifications for all prayer times, also can make
  notification time interval depending on the prayer time
- quit
  user can quit the app
- user can change the language
- reopen
  if user opens the application again, not city slection nothing, just displaying the times and main screen

# problems

- where do I get a reliable entry for hijri dates
- is the scrollable area for dates finite or infinite
- based on what I schedule the hijri updates, is the app context aware about the month or any reasonable anchor
- the most valuable thing is the dataset, it should contain everything the app needs
- many countries have different sets of time in different states
- many countries have summer time (DST)
- what about exceptions like the north pole where sun never goes up for long times? the calculation logic needs angles
- the dataset needs to be accurate for satisfying as many users as I can
- some people don't love install scripts
- handle the city duplications
- windows also can be handy in terms of terminal API. usually crossterm handles this, but details like key
  press/release/hold need to take care of.
- since we covering every city in the world, data shouldnt be hardcoded
- arabic isnt supported in terminal emulators, only Latin langs
- shoult the app have CD piplines? does it worth it?

# user nature

- not so many Muslims are technical persons. the fact that it's a TUI app makes it have a small user base
- windows users aren't familiar with TUIs as much as Linux users
- expect Arabs to be the least percentage of the whole base

# where to announce

- reddit muslim tech communities (although you won't find much for these specifics)
- facebook muslim tech groups
- persons who announce other projects
- news pages like facebook arab tech pages