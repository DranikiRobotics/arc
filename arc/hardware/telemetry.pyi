class Telemetry(object):
    """
    The telemetry system.

    This is used to send messages to the driver station.
    """

    def debug(self, *message: object) -> None:
        """
        Sends a debug message to the telemetry system.

        This always sends the message, regardless whether or not the client is going to log it.
        """
        ...
    
    def log(self, *message: object) -> None:
        """
        Sends a log message to the telemetry system.

        This always sends the message, and the client will also always log it.
        """
        ...
