import datetime
import os
import shutil
import tempfile
import unittest
from unittest.mock import MagicMock

import piexif
from PIL import Image

from app.reader import Reader
from app.writer import Writer


class TestExtractor(unittest.TestCase):
    def setUp(self):
        self.test_dir = tempfile.mkdtemp()

    def tearDown(self):
        shutil.rmtree(self.test_dir)

    def test_writer_exif(self):
        writer = Writer(folder=self.test_dir, total_frames=1)
        mock_frame = MagicMock()
        img = Image.new("RGB", (100, 100), color="red")
        mock_frame.to_image.return_value = img

        dt = datetime.datetime(
            2026, 6, 7, 12, 34, 56, 123456, tzinfo=datetime.timezone.utc
        )
        writer.write_frame(0, mock_frame, dt, output_index=1)

        filepath = os.path.join(self.test_dir, "frame_000001.jpg")
        self.assertTrue(os.path.exists(filepath))

        exif_dict = piexif.load(filepath)
        self.assertEqual(exif_dict["GPS"], {})

        exif_ifd = exif_dict["Exif"]
        dt_orig = exif_ifd[piexif.ExifIFD.DateTimeOriginal].decode("utf-8")
        self.assertEqual(dt_orig, "2026:06:07 12:34:56")

        sub_sec = exif_ifd[piexif.ExifIFD.SubSecTimeOriginal].decode("utf-8")
        self.assertEqual(sub_sec, "12")  # 123456 // 10000 = 12

    def test_reader_parse_timestamp(self):
        reader = Reader.__new__(Reader)
        dt = reader._parse_timestamp("2026-04-27T12:44:15Z")
        self.assertEqual(dt.year, 2026)
        self.assertEqual(dt.tzinfo, datetime.timezone.utc)

        dt_naive = reader._parse_timestamp("2026-04-27T12:44:15")
        self.assertEqual(dt_naive.year, 2026)
        self.assertEqual(dt_naive.tzinfo, datetime.timezone.utc)

    def test_frame_interval_logic(self):
        total_frames = 25
        frame_interval = 10
        target_indices = list(range(0, total_frames, frame_interval))
        self.assertEqual(target_indices, [0, 10, 20])


if __name__ == "__main__":
    unittest.main()
