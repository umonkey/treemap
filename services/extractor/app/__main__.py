import argparse
import json
import os
import sys

import av

from . import Reader, Writer
from .exceptions import UsageException
from .map_match import run_map_match
from .masks import create_masks
from .speed import (
    calculate_frame_interval,
    calculate_moving_speed,
    get_video_fps,
    parse_gpx,
)
from .trajectory import run_align_trajectory


def handle_match(args):
    try:
        run_map_match(args.input_dir, args.output_dir, valhalla_url=args.url)
    except Exception as e:
        print(f"Error during map matching: {e}", file=sys.stderr)
        sys.exit(1)


def handle_create_overrides(args):
    try:
        with av.open(args.video_path) as container:
            stream = container.streams.video[0]
            width = stream.width
            height = stream.height
        data = {
            "all": {
                "projection_type": "spherical",
                "width": width,
                "height": height,
            }
        }
        print(json.dumps(data))
    except Exception as e:
        print(f"Error reading video metadata: {e}", file=sys.stderr)
        sys.exit(1)


def handle_create_masks(args):
    try:
        create_masks(args.dataset_path, mask_size=args.mask_size)
    except UsageException as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error during mask creation: {e}", file=sys.stderr)
        sys.exit(1)


def handle_align_trajectory(args):
    try:
        run_align_trajectory(args.dataset_path)
    except UsageException as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error during trajectory alignment: {e}", file=sys.stderr)
        sys.exit(1)


def handle_extract(args):
    try:
        video_path = args.video_path
        output_folder = args.output_folder
        dataset_path = os.path.dirname(output_folder)

        gpx_path = args.gpx
        if not gpx_path:
            candidate1 = os.path.join(dataset_path, "track.gpx")
            candidate2 = os.path.join(os.path.dirname(video_path), "track.gpx")
            if os.path.exists(candidate1):
                gpx_path = candidate1
            elif os.path.exists(candidate2):
                gpx_path = candidate2
                dataset_path = os.path.dirname(video_path)

        reader = Reader(video_path)
        fps = reader.fps
        if fps <= 0:
            fps = get_video_fps(video_path)

        speed = None
        if gpx_path and os.path.exists(gpx_path):
            points = parse_gpx(gpx_path)
            speed = calculate_moving_speed(points)

        frame_interval = calculate_frame_interval(fps, speed, target_distance=1.0)
        avg_speed = speed if speed is not None else 1.2

        print(f"Video FPS: {fps:.2f}")
        print(f"Average Speed: {avg_speed:.2f} m/s (from GPX: {speed is not None})")
        print(f"Calculated Frame Interval: {frame_interval} (target distance: 1.0m)")

        os.makedirs(dataset_path, exist_ok=True)
        target_indices = list(range(0, reader.total_frames, frame_interval))

        writer = Writer(
            folder=output_folder,
            total_frames=reader.total_frames,
        )

        missing_targets = []
        for i, target_idx in enumerate(target_indices):
            out_idx = i + 1
            filename = os.path.join(output_folder, f"frame_{out_idx:06d}.jpg")
            if not os.path.exists(filename):
                missing_targets.append((target_idx, out_idx))

        if len(missing_targets) < len(target_indices):
            print(
                f"Resuming: found "
                f"{len(target_indices) - len(missing_targets)} "
                "existing frames, skipping..."
            )

        print(f"Extracting {len(target_indices)} frames...")
        if missing_targets:
            missing_indices = [t[0] for t in missing_targets]
            for (target_idx, out_idx), (index, frame, frame_time) in zip(
                missing_targets,
                reader.read(indices=missing_indices),
            ):
                writer.write_frame(
                    index,
                    frame,
                    frame_time,
                    output_index=out_idx,
                )
    except UsageException as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error during extraction: {e}", file=sys.stderr)
        sys.exit(1)


def handle_calculate_frame_interval(args):
    try:
        video_path = args.video_path
        fps = get_video_fps(video_path)
        gpx_path = args.gpx
        if not gpx_path and os.path.exists(
            os.path.join(os.path.dirname(video_path), "track.gpx")
        ):
            gpx_path = os.path.join(os.path.dirname(video_path), "track.gpx")

        speed = None
        if gpx_path and os.path.exists(gpx_path):
            points = parse_gpx(gpx_path)
            speed = calculate_moving_speed(points)

        frame_interval = calculate_frame_interval(fps, speed, target_distance=1.0)
        avg_speed = speed if speed is not None else 1.2

        print(f"Video FPS: {fps:.2f}")
        print(f"Average Speed: {avg_speed:.2f} m/s")
        print(f"Frame Interval: {frame_interval}")
    except Exception as e:
        print(f"Error calculating frame interval: {e}", file=sys.stderr)
        sys.exit(1)


def main():
    parser = argparse.ArgumentParser(description="Extract video frames.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    extract_parser = subparsers.add_parser("extract", help="Extract frames from video")
    extract_parser.add_argument("video_path", help="Path to the video file")
    extract_parser.add_argument("output_folder", help="Folder to save extracted frames")
    extract_parser.add_argument("--gpx", help="Path to GPX track file")
    extract_parser.set_defaults(func=handle_extract)

    calc_fi_parser = subparsers.add_parser(
        "calculate-frame-interval", help="Calculate frame interval from video and GPX"
    )
    calc_fi_parser.add_argument("video_path", help="Path to the video file")
    calc_fi_parser.add_argument("--gpx", help="Path to GPX track file")
    calc_fi_parser.set_defaults(func=handle_calculate_frame_interval)

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

    create_overrides_parser = subparsers.add_parser(
        "create-camera-overrides", help="Create camera models overrides JSON from video"
    )
    create_overrides_parser.add_argument("video_path", help="Path to the video file")
    create_overrides_parser.set_defaults(func=handle_create_overrides)

    create_masks_parser = subparsers.add_parser(
        "create-masks", help="Create image masks for OpenSfM"
    )
    create_masks_parser.add_argument("dataset_path", help="Path to dataset directory")
    create_masks_parser.add_argument(
        "--mask-size",
        type=float,
        default=float(os.environ.get("MASK_SIZE", 0.35)),
        help="Height fraction of the black mask part",
    )
    create_masks_parser.set_defaults(func=handle_create_masks)

    align_trajectory_parser = subparsers.add_parser(
        "align-trajectory", help="Align SfM reconstruction trajectory with GPS track"
    )
    align_trajectory_parser.add_argument(
        "dataset_path", help="Path to dataset directory"
    )
    align_trajectory_parser.set_defaults(func=handle_align_trajectory)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
