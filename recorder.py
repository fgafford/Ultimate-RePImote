import RPi.GPIO as GPIO
from datetime import datetime
from datetime import timedelta

# Pin for reading IR signal
INPUT = 11
on = False
times = []

# Setup
GPIO.setmode(GPIO.BOARD)

GPIO.setup(INPUT, GPIO.IN, pull_up_down=GPIO.PUD_UP)

GPIO.add_event_detect(INPUT, GPIO.BOTH)

def handler(data):
    global times
    global on
    time = datetime.now()
    on = not on
    times.append(time)

GPIO.add_event_callback(INPUT, handler)
try: 
    while True:
        a = 1+1
except KeyboardInterrupt:
    print("Signals:")
    #print(times)
    for i, t in enumerate(times):
        if t != times[0]:
            elapse = times[i] - times[i-1]
            state = "off:" if i % 2 == 0 else "on:"
            print(state, elapse,"μs")
    GPIO.cleanup()
