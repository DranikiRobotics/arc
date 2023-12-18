from .drive import AdvancedDrive, PositionAwareDrive

class MecanumDrive(AdvancedDrive, PositionAwareDrive):
    def forward(self, distance: float) -> None:
        pass

    def backward(self, distance: float) -> None:
        pass
    
    def turn_left(self, angle: float) -> None:
        pass

    def turn_right(self, angle: float) -> None:
        pass