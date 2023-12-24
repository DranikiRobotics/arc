from arc.hardware import HardwareComponent as _HardwareComponent

class DcMotor(_HardwareComponent):
    @property
    def uuid(self) -> str: ...
    def power(self, power: float) -> None:
        """
        Set the power of the motor.

        If the power is negative, the motor will spin backwards.
        """