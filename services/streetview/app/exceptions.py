class UsageException(Exception):
    """Base class for exceptions that should not show a stack trace."""

    pass


class CreationTimeMissing(UsageException):
    def __init__(self):
        message = (
            "The video file does not have creation_time set.  "
            "You should specify it using the --timestamp argument.  "
            "To get the timestamp, use this command on your source file (.osv):\n\n"
            'ffprobe -v quiet -show_entries format_tags=creation_time "source.OSV"'
        )
        super().__init__(message)
