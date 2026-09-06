"""
This class implements a video frame reader.
"""

from datetime import datetime, timedelta, timezone

import av


class Reader:
    def __init__(self, video_path):
        self._container = av.open(video_path)
        self._stream = self._container.streams.video[0]
        self._stream.thread_type = "AUTO"
        self._base_time = self._get_creation_time(self._container)

        self.total_frames = self._get_total_frames()
        print(f"Opening {video_path} to read {self.total_frames} video frames.")

    def _get_total_frames(self):
        """
        Returns the total number of frames in the video stream.
        Uses the metadata if available, otherwise estimates from duration
        and frame rate.
        """
        if self._stream.frames:
            return self._stream.frames

        if self._stream.duration and self._stream.average_rate:
            return int(
                self._stream.duration
                * self._stream.time_base
                * self._stream.average_rate
            )

        return 0

    @property
    def fps(self):
        return float(self._stream.average_rate)

    def read(self, indices=None):
        if indices is None:
            for index, frame in enumerate(self._container.decode(self._stream)):
                yield self._format_frame(index, frame)
        else:
            current_pos = None
            decoder = self._container.decode(self._stream)

            for target_idx in indices:
                if (
                    current_pos is None
                    or target_idx - current_pos > 50
                    or target_idx < current_pos
                ):
                    target_time = target_idx / self.fps if self.fps > 0 else 0.0
                    time_base = (
                        self._stream.time_base
                        if self._stream.time_base is not None
                        else 1
                    )
                    timestamp = int(target_time / time_base)
                    self._container.seek(timestamp, backward=True, stream=self._stream)
                    decoder = self._container.decode(self._stream)
                    current_pos = None

                for frame in decoder:
                    pts = frame.pts if frame.pts is not None else 0
                    time_base = (
                        self._stream.time_base
                        if self._stream.time_base is not None
                        else 1
                    )
                    frame_offset_seconds = float(pts * time_base)
                    frame_idx = (
                        int(round(frame_offset_seconds * self.fps))
                        if self.fps > 0
                        else 0
                    )

                    current_pos = frame_idx

                    if frame_idx >= target_idx:
                        yield self._format_frame(target_idx, frame)
                        break

    def _format_frame(self, index, frame):
        pts = frame.pts if frame.pts is not None else 0
        time_base = self._stream.time_base if self._stream.time_base is not None else 1
        frame_offset_seconds = float(pts * time_base)
        frame_time = self._base_time + timedelta(seconds=frame_offset_seconds)

        return index, frame, frame_time

    def _get_progress(self, index):
        """
        Calculates current progress percentage.
        """
        if self.total_frames > 0:
            return (index + 1) / self.total_frames * 100
        return 0.0

    def _get_creation_time(self, container):
        creation_time = container.metadata.get("creation_time")

        if creation_time is not None:
            try:
                return self._parse_timestamp(creation_time)
            except Exception:
                pass

        return datetime(1970, 1, 1, 0, 0, 0, tzinfo=timezone.utc)

    def _parse_timestamp(self, timestamp_str):
        timestamp_str = timestamp_str.replace("Z", "+00:00")
        dt = datetime.fromisoformat(timestamp_str)
        if dt.tzinfo is None:
            dt = dt.replace(tzinfo=timezone.utc)
        return dt
