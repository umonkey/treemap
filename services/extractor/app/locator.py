"""
This service gets exact GPS coordinates from a GPX track by a time offset.
Uses interpolation to accurately tag frames between GPS data points.
"""

import datetime

import gpxpy
import gpxpy.geo


class NoCoordinates(Exception):
    pass


class Locator:
    def __init__(self, gpx_path):
        self._start_time = None
        self._points = self._load_points(gpx_path)
        print(f"Found {len(self._points)} points in {gpx_path}")

    def locate(self, offset_seconds):
        """
        Get precise GPS coordinates by offset (seconds).
        Returns (lat, lon, gps_time).
        """
        if not self._points:
            raise NoCoordinates("No GPS points loaded")

        prev, next_pt = self._find_points(offset_seconds)

        if prev is None or next_pt is None:
            print(f"No coordinates for offset {offset_seconds}")
            raise NoCoordinates()

        lat, lon = self._interpolate(offset_seconds, prev, next_pt)
        gps_time = self._start_time + datetime.timedelta(seconds=offset_seconds)

        return lat, lon, gps_time

    def _load_points(self, gpx_path):
        with open(gpx_path, "r") as gpx_file:
            gpx = gpxpy.parse(gpx_file)

        raw_points = []

        for track in gpx.tracks:
            for segment in track.segments:
                for point in segment.points:
                    if point.time is None:
                        continue

                    raw_points.append(
                        (
                            point.time,
                            point.latitude,
                            point.longitude,
                        )
                    )

        raw_points.sort()

        if not raw_points:
            self._start_time = None
            return []

        self._start_time = raw_points[0][0]
        points = []
        for p_time, lat, lon in raw_points:
            offset_seconds = (p_time - self._start_time).total_seconds()
            points.append((offset_seconds, lat, lon))

        return points

    def _find_points(self, current_offset):
        if current_offset <= self._points[0][0]:
            return self._points[0], self._points[0]

        if current_offset >= self._points[-1][0]:
            return self._points[-1], self._points[-1]

        prev = self._points[0]

        for next_pt in self._points[1:]:
            if next_pt[0] < current_offset:
                prev = next_pt
                continue

            return prev, next_pt

        return None, None

    def _interpolate(self, offset, prev, next_pt):
        o1, lat1, lon1 = prev
        o2, lat2, lon2 = next_pt

        time_delta = o2 - o1

        if time_delta == 0:
            return lat1, lon1

        time_elapsed = offset - o1
        fraction = time_elapsed / time_delta

        interp_lat = lat1 + (lat2 - lat1) * fraction
        interp_lon = lon1 + (lon2 - lon1) * fraction

        return interp_lat, interp_lon
