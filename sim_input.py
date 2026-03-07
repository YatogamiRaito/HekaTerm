import uinput
import time

def main():
    device = uinput.Device([
        uinput.KEY_H,
        uinput.KEY_E,
        uinput.KEY_L,
        uinput.KEY_O,
        uinput.KEY_ENTER,
    ])
    
    time.sleep(2) # wait for wezterm to start
    
    for key in [uinput.KEY_H, uinput.KEY_E, uinput.KEY_L, uinput.KEY_L, uinput.KEY_O, uinput.KEY_ENTER]:
        device.emit_click(key)
        time.sleep(0.1)

if __name__ == "__main__":
    main()
