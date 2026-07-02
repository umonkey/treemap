import json
import os
import shutil
from datetime import datetime
from fractions import Fraction
from glob import glob

import piexif  # type: ignore
import requests
from tqdm import tqdm  # type: ignore


def dms_to_decimal(dms, ref):
    def to_float(num, den):
        return num / den

    degrees = to_float(*dms[0])
    minutes = to_float(*dms[1])
    seconds = to_float(*dms[2])

    decimal = degrees + minutes / 60.0 + seconds / 3600.0
    if ref in [b"S", b"W"]:
        decimal = -decimal
    return decimal


def decimal_to_dms(value):
    abs_value = abs(value)
    deg = int(abs_value)
    t1 = (abs_value - deg) * 60
    min = int(t1)
    sec = round((t1 - min) * 60, 4)

    def to_rational(number):
        f = Fraction(str(number)).limit_denominator(1000000)
        return (f.numerator, f.denominator)

    return (to_rational(deg), to_rational(min), to_rational(sec))


def get_image_data(file_path):
    exif_dict = piexif.load(file_path)
    gps = exif_dict.get("GPS", {})

    if piexif.GPSIFD.GPSLatitude not in gps:
        return None

    lat = dms_to_decimal(
        gps[piexif.GPSIFD.GPSLatitude], gps[piexif.GPSIFD.GPSLatitudeRef]
    )
    lon = dms_to_decimal(
        gps[piexif.GPSIFD.GPSLongitude], gps[piexif.GPSIFD.GPSLongitudeRef]
    )

    # DateTimeOriginal
    exif = exif_dict.get("Exif", {})
    dt_str = exif.get(piexif.ExifIFD.DateTimeOriginal)
    if dt_str:
        dt = datetime.strptime(dt_str.decode("utf-8"), "%Y:%m:%d %H:%M:%S")
        timestamp = dt.timestamp()
    else:
        timestamp = os.path.getmtime(file_path)

    return {"lat": lat, "lon": lon, "time": timestamp, "path": file_path}


def map_match(points, valhalla_url="http://localhost:8002/trace_attributes"):
    shape = [{"lat": p["lat"], "lon": p["lon"], "time": p["time"]} for p in points]

    payload = {
        "shape": shape,
        "costing": "auto",
        "shape_match": "map_snap",
        "breakage_distance": 50,
        "trace_options": {
            "search_radius": 20,
            "gps_accuracy": 10,
        },
        "filters": {
            "attributes": [
                "matched.point",
                "matched.type",
                "matched.edge_index",
                "edge.begin_heading",
            ],
            "action": "include",
        },
    }

    response = requests.post(valhalla_url, data=json.dumps(payload))
    response.raise_for_status()
    return response.json()


def update_image_exif(file_path, lat, lon, heading=None):
    exif_dict = piexif.load(file_path)

    lat_dms = decimal_to_dms(lat)
    lon_dms = decimal_to_dms(lon)

    exif_dict["GPS"][piexif.GPSIFD.GPSLatitude] = lat_dms
    exif_dict["GPS"][piexif.GPSIFD.GPSLatitudeRef] = b"N" if lat >= 0 else b"S"
    exif_dict["GPS"][piexif.GPSIFD.GPSLongitude] = lon_dms
    exif_dict["GPS"][piexif.GPSIFD.GPSLongitudeRef] = b"E" if lon >= 0 else b"W"

    if heading is not None:
        # GPSImgDirection is a rational (numerator, denominator)
        f = Fraction(str(heading)).limit_denominator(100)
        exif_dict["GPS"][piexif.GPSIFD.GPSImgDirection] = (f.numerator, f.denominator)
        exif_dict["GPS"][piexif.GPSIFD.GPSImgDirectionRef] = b"T"  # True direction

    exif_bytes = piexif.dump(exif_dict)
    piexif.insert(exif_bytes, file_path)


def run_map_match(
    image_dir, output_dir, valhalla_url="http://localhost:8002/trace_attributes"
):
    # Health check for Valhalla service
    status_url = valhalla_url.replace("/trace_attributes", "/status")
    try:
        # Using a small timeout to avoid long hangs
        requests.get(status_url, timeout=2)
    except requests.exceptions.RequestException:
        print("\nError: Valhalla service is not responding at " + valhalla_url)
        print("Please ensure the service is running. You can start it with:")
        print("\n    docker compose up -d\n")
        return

    os.makedirs(output_dir, exist_ok=True)
    image_files = sorted(glob(os.path.join(image_dir, "*.jpg")))

    if not image_files:
        print(f"No JPEG images found in {image_dir}")
        return

    print(f"Reading EXIF from {len(image_files)} images...")
    points = []
    for f in tqdm(image_files):
        data = get_image_data(f)
        if data:
            points.append(data)

    if not points:
        print("No geotagged images found.")
        return

    # Dither timestamps to avoid collisions and restore velocity signal
    for i in range(1, len(points)):
        if points[i]["time"] <= points[i - 1]["time"]:
            points[i]["time"] = points[i - 1]["time"] + 0.1

    print(f"Sending {len(points)} points to Valhalla for map matching...")
    try:
        result = map_match(points, valhalla_url=valhalla_url)
    except Exception as e:
        print(f"Error during map matching: {e}")
        return

    matched_points = result.get("matched_points", [])
    edges = result.get("edges", [])

    print("Updating images with matched coordinates and bearing...")
    for i, matched in enumerate(tqdm(matched_points)):
        orig_path = points[i]["path"]
        filename = os.path.basename(orig_path)
        new_path = os.path.join(output_dir, filename)

        shutil.copy2(orig_path, new_path)

        lat = matched.get("lat", points[i]["lat"])
        lon = matched.get("lon", points[i]["lon"])

        heading = None
        edge_index = matched.get("edge_index")
        if edge_index is not None and edge_index < len(edges):
            heading = edges[edge_index].get("begin_heading")

        update_image_exif(new_path, lat, lon, heading)


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("input_dir")
    parser.add_argument("output_dir")
    parser.add_argument("--url", default="http://localhost:8002/trace_attributes")
    args = parser.parse_args()
    run_map_match(args.input_dir, args.output_dir, valhalla_url=args.url)
