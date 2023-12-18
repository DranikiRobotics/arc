from arc.math import Pos2D as _Pos2D
import abc as _abc

class Drive(_abc.ABC):
    """Represents a drive system."""
    @_abc.abstractmethod
    def forward(self, distance: float) -> None:
        """Drives forward the specified distance."""
        pass

    @_abc.abstractmethod
    def backward(self, distance: float) -> None:
        """Drives backward the specified distance."""
        pass
    
    @_abc.abstractmethod
    def turn_left(self, angle: float) -> None:
        """Turns left the specified angle."""
        pass

    @_abc.abstractmethod
    def turn_right(self, angle: float) -> None:
        """Turns right the specified angle."""
        pass

class StrafingDrive(Drive):
    """Represents a drive system that can also strafe."""
    @_abc.abstractmethod
    def left(self, distance: float) -> None:
        """Strafes left the specified distance."""

    @_abc.abstractmethod
    def right(self, distance: float) -> None:
        """Strafes right the specified distance."""

class AdvancedDrive(StrafingDrive):
    """Represents a drive system that can also move to a position."""
    @_abc.abstractmethod
    def moveto(self, pos: _Pos2D) -> None:
        """Moves to the specified position."""
        pass

class PositionAwareDrive(_abc.ABC):
    """Represents a drive system that can also return its position."""
    @_abc.abstractproperty
    def position(self) -> _Pos2D:
        """The current position of the robot."""
        return None # type: ignore
