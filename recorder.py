import RPi.GPIO as GPIO
from datetime import datetime

# Pin for reading IR signal
INPUT = 11
on = False
times = []

# Setup
GPIO.setmode(GPIO.BOARD)

GPIO.setup(INPUT, GPIO.IN, pull_up_down=GPIO.PUD_UP)

GPIO.add_event_detect(INPUT, GPIO.BOTH)

def handler(data):
    global on
    global times
    times.append(datetime.now().microsecond)
    on = not on
    print(str(on))

GPIO.add_event_callback(INPUT, handler)
try: 
    while True:
        a = 1+1
except KeyboardInterrupt:
    print(times)
    GPIO.cleanup()
