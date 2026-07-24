import os
from glob import glob

from PIL import Image, ImageDraw

from .exceptions import UsageException


def create_masks(dataset_path: str, mask_size: float = 0.35):
    """
    Create masks for images in dataset_path/images and save them in dataset_path/masks.
    """
    images_dir = os.path.join(dataset_path, "images")
    if not os.path.isdir(images_dir):
        raise UsageException(f"Images directory not found: {images_dir}")

    masks_dir = os.path.join(dataset_path, "masks")
    os.makedirs(masks_dir, exist_ok=True)

    image_files = sorted(glob(os.path.join(images_dir, "*.jpg")))
    if not image_files:
        raise UsageException(f"No .jpg images found in {images_dir}")

    first_image_path = image_files[0]
    with Image.open(first_image_path) as img:
        width, height = img.size

    mask_path = os.path.join(masks_dir, "mask.png")
    black_height = int(height * mask_size)

    mask_img = Image.new("RGB", (width, height), (255, 255, 255))
    draw = ImageDraw.Draw(mask_img)
    draw.rectangle([0, height - black_height, width, height], fill=(0, 0, 0))
    mask_img.save(mask_path)

    print(
        f"Created mask.png ({width}x{height}, "
        f"black height: {black_height}px) in {masks_dir}"
    )

    for img_path in image_files:
        filename = os.path.basename(img_path)
        symlink_name = f"{filename}.png"
        symlink_path = os.path.join(masks_dir, symlink_name)

        if os.path.lexists(symlink_path):
            os.unlink(symlink_path)

        os.symlink("mask.png", symlink_path)

    print(f"Created {len(image_files)} mask symlinks in {masks_dir}")
