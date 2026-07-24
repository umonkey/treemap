import argparse
import sys

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
            args.video_path, timestamp=args.timestamp
        )
        writer = Writer(distance=args.distance, folder=args.output_folder)

        for index, frame, frame_offset_seconds, current_real_time in tqdm(
            reader.read(), total=reader.total_frames, desc="Processing frames"
        ):
            try:
                lookup_offset = frame_offset_seconds + args.offset
                lat, lon, gps_time = locator.locate(lookup_offset)
                frame_time = current_real_time
                writer.write_frame(index, frame, frame_time or gps_time, lat, lon, gps_time)
            except NoCoordinates:
                # print(f"No coordinates for frame {index}")
                pass
    except UsageException as e:
        print(f"Error: {e}", file=sys.stderr)
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
