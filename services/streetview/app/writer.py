"""
Contains the code that writes frames to files.
"""

import datetime
import io
import math
from fractions import Fraction

import piexif  # type: ignore
from tqdm import tqdm  # type: ignore


class Writer:
    def __init__(self, folder, distance=1.0):
        self._last = (None, None)
        self._distance = distance
        self._folder = folder
        self._index = 1

    def write_frame(self, index, frame, frame_time, lat, lon, gps_time):
        if self._should_write(lat, lon):
            self._last = (lat, lon)
            self._write(index, frame, frame_time, lat, lon, gps_time)

    def _to_rational(self, number):
        f = Fraction(str(number)).limit_denominator(1000000)
        return (f.numerator, f.denominator)

    def _write(self, index, frame, timestamp, lat, lon, gps_time):
        img = frame.to_image()

        lat_deg, lat_ref = self._loc_to_deg(lat, ["S", "N"])
        lon_deg, lon_ref = self._loc_to_deg(lon, ["W", "E"])

        # GPS time should be in UTC
        gps_time_utc = gps_time.astimezone(datetime.timezone.utc)
        gps_date_stamp = gps_time_utc.strftime("%Y:%m:%d")

        # Calculate fractional seconds
        gps_seconds = gps_time_utc.second + gps_time_utc.microsecond / 1_000_000

        gps_time_stamp = (
            (gps_time_utc.hour, 1),
            (gps_time_utc.minute, 1),
            self._to_rational(gps_seconds),
        )

        exif_dict = {
            "0th": {
                # Optional: Add software name or camera model here if you want
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
                piexif.ExifIFD.SubSecTimeOriginal: f"{timestamp.microsecond // 10000:02d}".encode(
                    "utf-8"
                ),
                piexif.ExifIFD.SubSecTimeDigitized: f"{timestamp.microsecond // 10000:02d}".encode(
                    "utf-8"
                ),
            },
            "GPS": {
                piexif.GPSIFD.GPSLatitudeRef: lat_ref,
                piexif.GPSIFD.GPSLatitude: lat_deg,
                piexif.GPSIFD.GPSLongitudeRef: lon_ref,
                piexif.GPSIFD.GPSLongitude: lon_deg,
                piexif.GPSIFD.GPSVersionID: (2, 2, 0, 0),  # Standard version
                piexif.GPSIFD.GPSTimeStamp: gps_time_stamp,
                piexif.GPSIFD.GPSDateStamp: gps_date_stamp.encode("utf-8"),
            },
            "1st": {},
            "thumbnail": self._get_thumbnail(img),
        }

        filename = self._get_filename()

        exif_bytes = piexif.dump(exif_dict)
        img.save(filename, "JPEG", exif=exif_bytes, quality=95)

        tqdm.write(f"Writing frame {index} as {filename} @ {lat},{lon}")

    def _get_thumbnail(self, img):
        thumb_io = io.BytesIO()
        thumbnail = img.copy()
        thumbnail.thumbnail((256, 256))
        thumbnail.save(thumb_io, format="JPEG")
        return thumb_io.getvalue()

    def _should_write(self, lat, lon):
        if self._last[0] is None:
            return True

        llat, llon = self._last

        distance = self._get_distance(llat, llon, lat, lon)

        return distance >= self._distance

    def _get_distance(self, lat1, lon1, lat2, lon2):
        """
        Get distance in meters between two GPS points, in meters.
        """
        R = 6371000

        # Convert decimal degrees to radians
        phi1, phi2 = math.radians(lat1), math.radians(lat2)
        dphi = math.radians(lat2 - lat1)
        dlambda = math.radians(lon2 - lon1)

        # Haversine formula
        a = (
            math.sin(dphi / 2) ** 2
            + math.cos(phi1) * math.cos(phi2) * math.sin(dlambda / 2) ** 2
        )

        c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))

        return R * c

    def _loc_to_deg(self, value, loc):
        """
        Converts decimal coordinates to EXIF-ready (degrees, minutes, seconds)
        rational tuples.
        loc: ["lat" or "lon"] used to determine the Reference (N/S or E/W).
        """
        if value < 0:
            loc_value = loc[0]  # "S" or "W"
        else:
            loc_value = loc[1]  # "N" or "E"

        abs_value = abs(value)
        deg = int(abs_value)
        t1 = (abs_value - deg) * 60
        min = int(t1)
        sec = round((t1 - min) * 60, 4)  # Round to 4 decimals for reasonable precision

        return (
            self._to_rational(deg),
            self._to_rational(min),
            self._to_rational(sec),
        ), loc_value.encode("utf-8")

    def _get_filename(self):
        filename = f"{self._folder}/frame_{self._index:06d}.jpg"
        self._index += 1
        return filename
