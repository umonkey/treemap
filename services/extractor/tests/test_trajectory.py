import os
import tempfile
import unittest

import numpy as np

from app.trajectory import (
    Y_DOWN_FRAME,
    Z_UP_FRAME,
    component_summary,
    detect_sfm_frame,
    horizontal_axes,
    up_axis,
    up_sign,
    world_rotation,
)


class TestDetectSfmFrame(unittest.TestCase):
    def _dataset_with_config(self, content: str) -> str:
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        with open(os.path.join(directory.name, "config.yaml"), "w") as f:
            f.write(content)
        return directory.name

    def test_align_method_none_is_y_down(self):
        path = self._dataset_with_config("align_method: none\n")
        self.assertEqual(detect_sfm_frame(path), Y_DOWN_FRAME)

    def test_align_method_auto_is_z_up(self):
        path = self._dataset_with_config("align_method: auto\n")
        self.assertEqual(detect_sfm_frame(path), Z_UP_FRAME)

    def test_align_method_orientation_prior_is_z_up(self):
        path = self._dataset_with_config(
            "processes: 4\nalign_method: 'orientation_prior'\n"
        )
        self.assertEqual(detect_sfm_frame(path), Z_UP_FRAME)

    def test_missing_config_defaults_to_y_down(self):
        with tempfile.TemporaryDirectory() as path:
            self.assertEqual(detect_sfm_frame(path), Y_DOWN_FRAME)


class TestFrameAxes(unittest.TestCase):
    def test_axes(self):
        self.assertEqual(horizontal_axes(Y_DOWN_FRAME), (0, 2))
        self.assertEqual(horizontal_axes(Z_UP_FRAME), (0, 1))
        self.assertEqual(up_axis(Y_DOWN_FRAME), 1)
        self.assertEqual(up_axis(Z_UP_FRAME), 2)
        self.assertEqual(up_sign(Y_DOWN_FRAME), -1.0)
        self.assertEqual(up_sign(Z_UP_FRAME), 1.0)


class TestWorldRotation(unittest.TestCase):
    def test_y_down_identity_matches_legacy_matrix(self):
        np.testing.assert_allclose(
            world_rotation(0.0, Y_DOWN_FRAME),
            [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]],
        )

    def test_z_up_identity(self):
        np.testing.assert_allclose(world_rotation(0.0, Z_UP_FRAME), np.eye(3))

    def test_y_down_quarter_turn(self):
        np.testing.assert_allclose(
            world_rotation(np.pi / 2.0, Y_DOWN_FRAME),
            [[0.0, 0.0, -1.0], [1.0, 0.0, 0.0], [0.0, -1.0, 0.0]],
            atol=1e-12,
        )

    def test_z_up_quarter_turn(self):
        np.testing.assert_allclose(
            world_rotation(np.pi / 2.0, Z_UP_FRAME),
            [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
            atol=1e-12,
        )

    def test_y_down_maps_camera_frame_to_enu(self):
        c_sfm = np.array([0.0, -5.0, 10.0])
        scale = 2.0
        translation = np.array([1.0, 2.0, 3.0])
        c_enu = scale * (c_sfm @ world_rotation(0.0, Y_DOWN_FRAME).T) + translation
        np.testing.assert_allclose(c_enu, [1.0, 22.0, 13.0])


class TestComponentSummary(unittest.TestCase):
    def test_summary_reports_each_component(self):
        reconstructions = [
            {
                "shots": {
                    "frame_000002.jpg": {
                        "rotation": [0.0, 0.0, 0.0],
                        "translation": [1.0, 2.0, 3.0],
                    },
                    "frame_000001.jpg": {
                        "rotation": [0.0, 0.0, 0.0],
                        "translation": [4.0, 5.0, 6.0],
                    },
                },
                "points": {"a": {}, "b": {}},
            },
            {
                "shots": {
                    "frame_000010.jpg": {
                        "rotation": [0.0, 0.0, 0.0],
                        "translation": [-1.0, 0.0, 0.0],
                    },
                },
                "points": {"c": {}},
            },
        ]

        summary = component_summary(reconstructions, Z_UP_FRAME)

        self.assertEqual(summary["sfm_frame"], Z_UP_FRAME)
        self.assertEqual(len(summary["components"]), 2)

        main = summary["components"][0]
        self.assertEqual(main["shot_count"], 2)
        self.assertEqual(main["point_count"], 2)
        self.assertEqual(main["shot_ids"], ["frame_000001.jpg", "frame_000002.jpg"])
        self.assertEqual(main["shot_indices"], [1, 2])
        self.assertEqual(
            main["camera_centers"],
            [[-4.0, -5.0, -6.0], [-1.0, -2.0, -3.0]],
        )

        secondary = summary["components"][1]
        self.assertEqual(secondary["shot_count"], 1)
        self.assertEqual(secondary["point_count"], 1)
        self.assertEqual(secondary["shot_ids"], ["frame_000010.jpg"])
        self.assertEqual(secondary["camera_centers"], [[1.0, 0.0, 0.0]])
