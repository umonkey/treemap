"""
Contains the code that writes frames to files.
"""

import io

import piexif  # type: ignore


class Writer:
    def __init__(self, folder, total_frames=None):
        self._folder = folder
        self._index = 1
        self._total_frames = total_frames

    def write_frame(self, index, frame, timestamp, output_index=None):
        if output_index is not None:
            self._index = output_index

        img = frame.to_image()

        exif_dict = {
            "0th": {
                piexif.ImageIFD.Make: b"Python Extractor",
                piexif.ImageIFD.DateTime: timestamp.strftime(
                    "%Y:%m:%d %H:%M:%S"
                ).encode("utf-8"),
            },
            "Exif": {
                piexif.ExifIFD.DateTimeOriginal: timestamp.strftime(
                    "%Y:%m:%d %H:%M:%S"
                ).encode("utf-8"),
                piexif.ExifIFD.DateTimeDigitized: timestamp.strftime(
                    "%Y:%m:%d %H:%M:%S"
                ).encode("utf-8"),
                piexif.ExifIFD.SubSecTimeOriginal: (
                    f"{timestamp.microsecond:06d}".encode("utf-8")
                ),
                piexif.ExifIFD.SubSecTimeDigitized: (
                    f"{timestamp.microsecond:06d}".encode("utf-8")
                ),
            },
            "GPS": {},
            "1st": {},
            "thumbnail": self._get_thumbnail(img),
        }

        filename = self._get_filename()

        exif_bytes = piexif.dump(exif_dict)
        img.save(filename, "JPEG", exif=exif_bytes, quality=95)

        if self._total_frames:
            percent = min(100, int((index + 1) / self._total_frames * 100))
            print(
                f"Writing frame {index+1}/{self._total_frames} "
                f"({percent}%) as {filename}"
            )
        else:
            print(f"Writing frame {index+1} as {filename}")

    def _get_thumbnail(self, img):
        thumb_io = io.BytesIO()
        thumbnail = img.copy()
        thumbnail.thumbnail((256, 256))
        thumbnail.save(thumb_io, format="JPEG")
        return thumb_io.getvalue()

    def _get_filename(self):
        filename = f"{self._folder}/frame_{self._index:06d}.jpg"
        self._index += 1
        return filename
