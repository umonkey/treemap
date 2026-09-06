"""
Speed calculation, GPX parsing, and frame interval computation utilities.
"""

import math
import xml.etree.ElementTree as ET
from datetime import datetime, timezone
from typing import List, Optional, Tuple

import av


def haversine_distance(lat1: float, lon1: float, lat2: float, lon2: float) -> float:
    """Calculates great-circle distance in meters between two lat/lon points."""
    r = 6371000.0
    dlat = math.radians(lat2 - lat1)
    dlon = math.radians(lon2 - lon1)
    a = (
        math.sin(dlat / 2) ** 2
        + math.cos(math.radians(lat1))
        * math.cos(math.radians(lat2))
        * math.sin(dlon / 2) ** 2
    )
    c = 2 * math.asin(math.sqrt(a))
    return r * c


def parse_iso_time(time_str: str) -> datetime:
    """Parse ISO 8601 timestamp string into a UTC datetime object."""
    clean_str = time_str.strip().replace("Z", "+00:00")
    dt = datetime.fromisoformat(clean_str)
    if dt.tzinfo is None:
        dt = dt.replace(tzinfo=timezone.utc)
    return dt.astimezone(timezone.utc)


def parse_gpx(gpx_path: str) -> List[Tuple[float, float, float, float]]:
    """
    Parse a GPX file using xml.etree.ElementTree.

    Returns:
        List of (timestamp_epoch_sec, lat, lon, ele) sorted by timestamp.
    """
    tree = ET.parse(gpx_path)
    root = tree.getroot()

    points: List[Tuple[float, float, float, float]] = []
    for elem in root.iter():
        if elem.tag.endswith("trkpt"):
            try:
                lat = float(elem.attrib["lat"])
                lon = float(elem.attrib["lon"])
            except (KeyError, ValueError):
                continue

            ele = 0.0
            t_epoch: Optional[float] = None

            for child in elem:
                tag = child.tag.split("}")[-1]
                if tag == "ele" and child.text:
                    try:
                        ele = float(child.text)
                    except ValueError:
                        pass
                elif tag == "time" and child.text:
                    try:
                        dt = parse_iso_time(child.text)
                        t_epoch = dt.timestamp()
                    except Exception:
                        pass

            if t_epoch is not None:
                points.append((t_epoch, lat, lon, ele))

    points.sort(key=lambda p: p[0])
    return points


def calculate_moving_speed(
    points: List[Tuple[float, float, float, float]],
    min_speed: float = 0.5,
    max_speed: float = 40.0,
    max_interval: float = 60.0,
) -> Optional[float]:
    """
    Computes average moving speed from trackpoints, filtering out stationary drift,
    teleportation glitches, and long pauses.
    """
    if len(points) < 2:
        return None

    total_distance = 0.0
    total_time = 0.0

    for i in range(len(points) - 1):
        t1, lat1, lon1, _ = points[i]
        t2, lat2, lon2, _ = points[i + 1]

        dt = t2 - t1
        if dt <= 0 or dt > max_interval:
            continue

        dist = haversine_distance(lat1, lon1, lat2, lon2)
        speed = dist / dt

        if speed < min_speed or speed > max_speed:
            continue

        total_distance += dist
        total_time += dt

    if total_time > 0:
        return total_distance / total_time
    return None


def get_video_fps(video_path: str) -> float:
    """Reads stream frame rate via PyAV, defaulting safely to 30.0."""
    try:
        with av.open(video_path) as container:
            stream = container.streams.video[0]
            if stream.average_rate:
                return float(stream.average_rate)
            if stream.guessed_rate:
                return float(stream.guessed_rate)
    except Exception:
        pass
    return 30.0


def calculate_frame_interval(
    fps: float,
    speed: Optional[float],
    target_distance: float = 1.0,
    fallback_speed: float = 1.2,
) -> int:
    """
    Calculates frame interval given fps, speed, and target spatial distance (meters).
    """
    s = speed if speed is not None and speed > 0 else fallback_speed
    if s <= 0 or fps <= 0:
        return 1
    val = fps * target_distance / s
    return max(1, round(val))


def get_track_average_speed(dataset_dir: str) -> Optional[float]:
    """
    Gets the track average speed (in m/s) from track.gpx.
    """
    import os

    gpx_path = os.path.join(dataset_dir, "track.gpx")
    if os.path.exists(gpx_path):
        try:
            points = parse_gpx(gpx_path)
            speed = calculate_moving_speed(points)
            if speed is not None:
                return speed
        except Exception:
            pass

    return None


def determine_track_distance(
    speed_mps: Optional[float], env_distance: Optional[str] = None
) -> Tuple[float, str, bool]:
    """
    Determines the extraction distance based on track average speed.
    - If env_distance is provided and valid, it overrides the default.
    - If speed_mps > 40 km/h (approx 11.111 m/s), it is a driving track -> default 5.0m.
    - Otherwise, it is a walking/other track -> default 3.0m.

    Returns:
        (min_distance, track_type, is_overridden)
    """
    if env_distance is not None and env_distance.strip() != "":
        try:
            return float(env_distance), "custom", True
        except ValueError:
            pass

    # Threshold: 40 km/h = 40 / 3.6 m/s = 11.1111... m/s
    is_driving = False
    if speed_mps is not None and speed_mps > (40.0 / 3.6):
        is_driving = True

    track_type = "driving" if is_driving else "walking"
    min_distance = 5.0 if is_driving else 3.0
    return min_distance, track_type, False
