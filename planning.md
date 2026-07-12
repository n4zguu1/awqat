### main goal 
app for displaying and managing awqat al-salat, supports all times worldwide.
its main runtime is the terminal due to crossplatform support, but we should leave room for supporting platform independent functionality like 
- notification
- gui
- and more...
the base platform this app gonna be developed on is linux platform
### features
- display all salwat times 
- get the location from ip address 
- allow configure location address manually
- support the location till the last timinig block in muslim salawat system
- exclusive support for hyperland based distros, a waybar item do easy display the times (new experience, dealing with hyperland and waybar)
- room for adding more languages (RTL langs unfortanatly arent supported on populaair termiansl , so this will be dropped)

### cross platform features (next)
- push notification on desktop (new experience)
- custom times (before or after the salat time happens) 
- adhkar




### technical requirements
- ratatui as terminal tui library
- salah crate for times algorithms
- requests to for requesting ip info
- a background process for running desktop notifs (next versions...), uses systemd timers
- rust-i18n for internationalization


### user flow
- users install using curl command on the repo
- start the app using the command which been added on the path per default
- the tui shows on terminal

### Architecutal decision 
**toml or json for configs?**
**which crate to use for time algorithms**
**about the scheduling in different os targets**
- basically for linux we uses systemd timers, this decision is made cuz systemd is used in the major distros which mean no headaches
- windows have "Task Scheudling" that will do the word 
- about macos, i hve no idea, im not installing macos again for this

### CD pipeline
this is new experience for me.
CD means Continuous Developement


### CI pipeline
- cargo check
- cargo test
- cargo clippy

### ratatui
### user interface
### developement flow 
  start normally with implementing the cores to make the MVP, cargo run for feature + simple ci for github pushes-> ship the working project
  implement the CD pipeline and for cross compile all targets -> working  project for all targets
  add curl install for users to install -> installanble project

