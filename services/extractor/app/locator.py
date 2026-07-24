"""
This service gets exact GPS coordinates from a GPX track by a timestamp.
Uses interpolation to accurately tag frames between GPS data points.
"""

import datetime

import gpxpy
import gpxpy.geo


class NoCoordinates(Exception):
    pass


class Locator:
    def __init__(self, gpx_path, hz=10):
        points = self._load_points(gpx_path)
        print(f"Found {len(points)} points in {gpx_path}")

        self._points = self.upsample(points, hz)
        print(f"Upsampled to {len(self._points)} points at {hz} Hz")

    def upsample(self, points, hz):
        if not points or hz <= 0:
            return points

        interval = 1.0 / hz
        upsampled = []
        for i in range(len(points) - 1):
            p1 = points[i]
            p2 = points[i + 1]
            upsampled.append(p1)

            curr_t = p1[0] + datetime.timedelta(seconds=interval)
            while (p2[0] - curr_t).total_seconds() > 0.001:
                lat, lon = self._interpolate(curr_t, p1, p2)
                upsampled.append((curr_t, lat, lon))
                curr_t += datetime.timedelta(seconds=interval)

        upsampled.append(points[-1])
        return upsampled

    def find_closest(
        self, lat, lon, timestamp_hint=None, search_range=20
    ) -> tuple[datetime.datetime, float]:
        """
        Find the closest points within the search_range.
        If there are multiple intersections, use timestamp_hint to choose one.
        """
        candidates = self._find_closest_groups(lat, lon, search_range)

        if not candidates:
            raise NoCoordinates(
                f"No points found within {search_range}m of {lat}, {lon}"
            )

        if timestamp_hint:
            return min(
                candidates, key=lambda x: abs((x[0] - timestamp_hint).total_seconds())
            )

        if len(candidates) == 1:
            return candidates[0]

        print(f"Found {len(candidates)} possible intersections:")
        for t, d in candidates:
            print(f" - {t.isoformat()} (distance: {d:.2f}m)")

        print(
            "\nPlease provide a timestamp hint using --timestamp to choose "
            "the correct one."
        )

        print(f"Example: --timestamp {candidates[0][0].isoformat()}")

        raise NoCoordinates("Multiple intersections found. Please disambiguate.")

    def _find_closest_groups(
        self, lat, lon, search_range
    ) -> list[tuple[datetime.datetime, float]]:
        """
        Find contiguous groups of points within search_range and return the
        closest point from each.
        """
        candidates = []
        current_group = []

        for p_time, p_lat, p_lon in self._points:
            distance = gpxpy.geo.haversine_distance(lat, lon, p_lat, p_lon)
            if distance <= search_range:
                current_group.append((p_time, distance))
            else:
                if current_group:
                    candidates.append(min(current_group, key=lambda x: x[1]))
                    current_group = []

        if current_group:
            candidates.append(min(current_group, key=lambda x: x[1]))

        return candidates

    def locate(self, time):
        """
        Get precise GPS coordinates by time.
        """
        prev, next = self._find_points(time)

        if prev is None or next is None:
            print(f"No coordinates for {time}")
            raise NoCoordinates()

        lat, lon = self._interpolate(time, prev, next)

        return lat, lon, time

    def _load_points(self, gpx_path):
        with open(gpx_path, "r") as gpx_file:
            gpx = gpxpy.parse(gpx_file)

        points = []

        for track in gpx.tracks:
            for segment in track.segments:
                for point in segment.points:
                    if point.time is None:
                        continue

                    points.append(
                        (
                            point.time,
                            point.latitude,
                            point.longitude,
                        )
                    )

        points.sort()

        return points

    def _find_points(self, current_real_time):
        prev = self._points[0]

        for next in self._points[1:]:
            if next[0] < current_real_time:
                prev = next
                continue

            return prev, next

        return None, None

    def _interpolate(self, frame_time, prev, next):
        t1, lat1, lon1 = prev
        t2, lat2, lon2 = next

        time_delta = (t2 - t1).total_seconds()

        if time_delta == 0:
            return lat1, lon1

        time_elapsed = (frame_time - t1).total_seconds()
        fraction = time_elapsed / time_delta

        interp_lat = lat1 + (lat2 - lat1) * fraction
        interp_lon = lon1 + (lon2 - lon1) * fraction

        return interp_lat, interp_lon
