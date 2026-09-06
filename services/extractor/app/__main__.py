import argparse
import json
import os
import sys

import av

from . import Reader, Writer
from .exceptions import UsageException
from .map_match import run_map_match
from .masks import create_masks
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
        try:
            frame_interval = int(os.environ.get("FRAME_INTERVAL", "10"))
        except ValueError:
            raise UsageException("FRAME_INTERVAL must be an integer")
        if frame_interval <= 0:
            raise UsageException("FRAME_INTERVAL must be greater than 0")

        reader = Reader(args.video_path)
        target_indices = list(range(0, reader.total_frames, frame_interval))

        writer = Writer(
            folder=args.output_folder,
            total_frames=reader.total_frames,
        )

        missing_targets = []
        for i, target_idx in enumerate(target_indices):
            out_idx = i + 1
            filename = os.path.join(args.output_folder, f"frame_{out_idx:06d}.jpg")
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


def main():
    parser = argparse.ArgumentParser(description="Extract video frames.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    extract_parser = subparsers.add_parser("extract", help="Extract frames from video")
    extract_parser.add_argument("video_path", help="Path to the video file")
    extract_parser.add_argument("output_folder", help="Folder to save extracted frames")
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
