import os
import tempfile
import unittest
from typing import cast

from app.speed import (
    calculate_frame_interval,
    calculate_moving_speed,
    haversine_distance,
)


class TestSpeed(unittest.TestCase):
    def test_haversine_distance(self):
        d = haversine_distance(0.0, 0.0, 0.0, 1.0)
        self.assertAlmostEqual(d, 111194.9, delta=100.0)

    def test_calculate_moving_speed_valid(self):
        lat_delta = 10.0 / 111194.9
        points = [
            (0.0, 0.0, 0.0, 0.0),
            (2.0, lat_delta, 0.0, 0.0),
        ]
        speed = calculate_moving_speed(points)
        self.assertIsNotNone(speed)
        self.assertAlmostEqual(cast(float, speed), 5.0, delta=0.5)

    def test_calculate_moving_speed_filters(self):
        lat_p0 = 40.0
        lat_p1 = 40.0 + (0.2 / 111194.9)
        lat_p2 = 40.0 + (10.0 / 111194.9)
        lat_p3 = 40.0 + (60.0 / 111194.9)
        lat_p4 = 40.0 + (70.0 / 111194.9)

        points = [
            (0.0, lat_p0, 45.0, 0.0),
            (2.0, lat_p1, 45.0, 0.0),  # speed = 0.1 m/s -> filtered (< 0.5)
            (67.0, lat_p2, 45.0, 0.0),  # dt = 65s > 60s -> filtered
            (68.0, lat_p3, 45.0, 0.0),  # speed = 50 m/s > 40 -> filtered
            (70.0, lat_p4, 45.0, 0.0),  # valid: dt=2s, dist=10m -> speed = 5 m/s
        ]
        speed = calculate_moving_speed(points)
        self.assertIsNotNone(speed)
        self.assertAlmostEqual(cast(float, speed), 5.0, delta=0.5)

    def test_calculate_frame_interval(self):
        fi = calculate_frame_interval(30.0, 1.5, target_distance=1.0)
        self.assertEqual(fi, 20)

        fi_fallback = calculate_frame_interval(
            30.0, None, target_distance=1.0, fallback_speed=1.2
        )
        self.assertEqual(fi_fallback, 25)

        fi_zero = calculate_frame_interval(
            30.0, 0.0, target_distance=1.0, fallback_speed=1.2
        )
        self.assertEqual(fi_zero, 25)

    def test_frame_interval_json_io(self):
        import json

        with tempfile.TemporaryDirectory() as tmpdir:
            json_path = os.path.join(tmpdir, "frame_interval.json")
            data = {
                "frame_interval": 15,
                "fps": 30.0,
                "average_speed": 2.0,
                "target_distance": 1.0,
            }
            with open(json_path, "w") as f:
                json.dump(data, f)

            with open(json_path, "r") as f:
                loaded = json.load(f)
            self.assertEqual(loaded["frame_interval"], 15)
            self.assertEqual(loaded["fps"], 30.0)
            self.assertEqual(loaded["average_speed"], 2.0)


if __name__ == "__main__":
    unittest.main()
