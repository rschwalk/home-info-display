# home_info_display

Program to display various information on a wall mounted raspberry pi 3 with display.
The controlling will be done remotely over TCP.

### TODO
- [ ] TCP Connection
    - [x] Accepting client connection
    - [ ] Accepting more then one client
    - [ ] Define command handling
        - [x] Quit the application
        - [ ] Shut down the device
        - [ ] Restart the device (should automatically start our application)
    - [ ] Encrypt the data transfer
- [ ] Display the infos using SDL - Direct buffer rendering
    - [ ] SDL2
        - [x] "Hello World" SDL
        - [ ] Font rendering
        - [ ] Screen layout
    - [ ] ToDo Lists
    - [ ] Appointments
    - [ ] Time / Date
    - [ ] Weather from openweather.org
        - [ ] Current weather
        - [ ] Forecast
- [ ] Setup cross compiling

