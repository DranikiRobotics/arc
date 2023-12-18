from arc.hardware import *
from arc.math import *
from arc import *

# You are REQUIRED to have a main() function in your program.
# and you MUST NOT call it yourself.
@Auto("My Auto")
def main(op: Op):
    print("Starting...")

    holding_a = False
    while op.running:
        if op.gamepad.a:
            if not holding_a:
                print("A pressed!")
                holding_a = True
        else:
            if holding_a:
                print("A released!")
            holding_a = False

    print("Done!")

    return OK
