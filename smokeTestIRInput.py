import RPi.GPIO as GPIO

# Pin for reading IR signal
INPUT = 11

# Setup
GPIO.setmode(GPIO.BOARD)

GPIO.setup(INPUT, GPIO.IN, pull_up_down=GPIO.PUD_UP)

GPIO.add_event_detect(INPUT, GPIO.BOTH)

def handler(data):
    print("Signal Recieved on: ", str(data))

GPIO.add_event_callback(INPUT, handler)
try: 
    while True:
        print("-")
except KeyboardInterrupt:
    GPIO.cleanup()
