"""
Images are sent as a series of digits that each represent the color of a single
pixel. The digits fill each row of the image left-to-right, then move downward
to the next row, filling rows top-to-bottom until every pixel of the image is
filled.

Each image actually consists of a series of identically-sized layers that are
filled in this way. So, the first digit corresponds to the top-left pixel of
the first layer, the second digit corresponds to the pixel to the right of that
on the same layer, and so on until the last digit, which corresponds to the
bottom-right pixel of the last layer.

For example, given an image 3 pixels wide and 2 pixels tall, the image data
123456789012 corresponds to the following image layers:

Layer 1: 123 456

Layer 2: 789 012

The image you received is 25 pixels wide and 6 pixels tall.

"""
from pathlib import Path
from textwrap import fill
from typing import List

img_data = Path("input8.txt").read_text().rstrip()

l_width = 25
l_height = 6
l_size = l_width * l_height


def make_img_layers(img_data: str, l_size: int) -> List[str]:
    return [img_data[i : i + l_size] for i in range(0, len(img_data), l_size)]


img_layers = make_img_layers(img_data, l_size)

counts = [
    (layer.count("0"), layer.count("1"), layer.count("2"))
    for layer in img_layers
]

counts.sort(key=lambda x: x[0])

print(counts[0][1] * counts[0][2])

"""
Now you're ready to decode the image. The image is rendered by stacking the
layers and aligning the pixels with the same positions in each layer. The
digits indicate the color of the corresponding pixel: 0 is black, 1 is white,
and 2 is transparent.

The layers are rendered with the first layer in front and the last layer in
back. So, if a given position has a transparent pixel in the first and second
layers, a black pixel in the third layer, and a white pixel in the fourth
layer, the final image would have a black pixel at that position.
"""


def make_visibile_img(img_layers: List[List[str]]) -> str:
    img_visible = ""
    for window in zip(*img_layers):
        out = "2"
        for pixel in window:
            if pixel in ("0", "1"):
                out = pixel
                break
        img_visible += out
    return img_visible


img_visible = make_visibile_img(img_layers)

print(fill(img_visible.replace("1", "#").replace("0", " "), l_width))
