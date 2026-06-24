import argparse
import subprocess
import sys
from datetime import timedelta

from tqdm import tqdm  # type: ignore

from . import Locator, Reader, Writer
from .exceptions import UsageException
from .locator import NoCoordinates
from .map_match import run_map_match


def handle_match(args):
    try:
        run_map_match(args.input_dir, args.output_dir, valhalla_url=args.url)
    except Exception as e:
        print(f"Error during map matching: {e}", file=sys.stderr)
        sys.exit(1)


def handle_extract(args):
    try:
        locator = Locator(args.gpx_path)
        reader = Reader(
            args.video_path, offset_seconds=args.offset, timestamp=args.timestamp
        )
        writer = Writer(distance=args.distance, folder=args.output_folder)

        for index, frame, frame_time, progress in tqdm(
            reader.read(), total=reader.total_frames, desc="Processing frames"
        ):
            try:
                lat, lon, gps_time = locator.locate(frame_time)
                writer.write_frame(index, frame, frame_time, lat, lon, gps_time)
            except NoCoordinates:
                # print(f"No coordinates for frame {index}")
                pass
    except UsageException as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


def handle_synchronize(args):
    try:
        locator = Locator(args.gpx_path)
        reader = Reader(args.video_path, timestamp=args.timestamp)

        # Calculate estimated time of the frame
        fps = float(reader._stream.average_rate)
        frame_offset = timedelta(seconds=args.frame / fps)

        estimated_time = None
        if reader._creation_time:
            estimated_time = reader._creation_time + frame_offset
            print(f"Video start time: {reader._creation_time.isoformat()}")
            print(f"Frame {args.frame} offset: {frame_offset}")
            print(f"Estimated frame time: {estimated_time.isoformat()}")
        else:
            print("Video creation_time missing. Searching across the whole track.")
            print(f"Frame {args.frame} offset: {frame_offset}")

        # Find the actual GPX timestamp for those coordinates
        gpx_timestamp, distance = locator.find_closest(
            args.lat,
            args.lon,
            timestamp_hint=estimated_time,
            search_range=args.range,
        )

        print(
            f"Found closest GPX point at: {gpx_timestamp.isoformat()} "
            f"(distance: {distance:.2f}m)"
        )

        # Calculate new creation time
        new_creation_time = gpx_timestamp - frame_offset
        print(f"New video creation_time should be: {new_creation_time.isoformat()}")

        # Update metadata using ffmpeg
        output_path = args.video_path.rsplit(".", 1)
        output_path = f"{output_path[0]}.synced.{output_path[1]}"

        # Format timestamp for FFmpeg (ISO 8601)
        ffmpeg_timestamp = new_creation_time.isoformat().replace("+00:00", "Z")

        cmd = [
            "ffmpeg",
            "-y",
            "-i",
            args.video_path,
            "-c",
            "copy",
            "-map_metadata",
            "0",
            "-metadata",
            f"creation_time={ffmpeg_timestamp}",
            output_path,
        ]

        print(f"Running: {' '.join(cmd)}")
        subprocess.run(cmd, check=True)
        print(f"Successfully created synchronized video: {output_path}")

    except NoCoordinates as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    except UsageException as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    except subprocess.CalledProcessError as e:
        print(f"Error running ffmpeg: {e}", file=sys.stderr)
        sys.exit(1)


def main():
    parser = argparse.ArgumentParser(description="Map video frames to GPX coordinates.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    extract_parser = subparsers.add_parser(
        "extract", help="Extract frames from video and map to GPX"
    )
    extract_parser.add_argument("video_path", help="Path to the video file")
    extract_parser.add_argument("gpx_path", help="Path to the GPX file")
    extract_parser.add_argument("output_folder", help="Folder to save extracted frames")
    extract_parser.add_argument(
        "--offset", type=float, default=0.0, help="Time offset in seconds"
    )
    extract_parser.add_argument(
        "--distance",
        type=float,
        default=3.0,
        help="Minimum distance between frames in meters",
    )
    extract_parser.add_argument(
        "--timestamp",
        type=str,
        help="Manually specify the video creation date (e.g., 2026-04-27T12:44:15Z)",
    )
    extract_parser.set_defaults(func=handle_extract)

    sync_parser = subparsers.add_parser(
        "synchronize", help="Synchronize video with GPX"
    )
    sync_parser.add_argument("video_path", help="Path to the video file")
    sync_parser.add_argument("gpx_path", help="Path to the GPX file")
    sync_parser.add_argument(
        "--frame",
        type=int,
        required=True,
        help="The 0-indexed frame number from the video (e.g. from mpv)",
    )
    sync_parser.add_argument(
        "--lat", type=float, required=True, help="Latitude of the frame"
    )
    sync_parser.add_argument(
        "--lon", type=float, required=True, help="Longitude of the frame"
    )
    sync_parser.add_argument(
        "--timestamp",
        type=str,
        help="Manually specify the current video creation date if missing",
    )
    sync_parser.add_argument(
        "--range",
        type=int,
        default=20,
        help="Search range in meters (default: 20)",
    )
    sync_parser.set_defaults(func=handle_synchronize)

    match_parser = subparsers.add_parser(
        "match", help="Map match geotagged images to road network"
    )
    match_parser.add_argument("input_dir", help="Directory with geotagged images")
    match_parser.add_argument("output_dir", help="Directory to save matched images")
    match_parser.add_argument(
        "--url",
        default="http://localhost:8002/trace_attributes",
        help="Valhalla trace_attributes URL",
    )
    match_parser.set_defaults(func=handle_match)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
