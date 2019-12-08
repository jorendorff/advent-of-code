"""Day 8: Space Image Format

The Elves' spirits are lifted when they realize you have an opportunity to
reboot one of their Mars rovers, and so they are curious if you would spend a
brief sojourn on Mars. You land your ship near the rover.

When you reach the rover, you discover that it's already in the process of
rebooting! It's just waiting for someone to enter a BIOS password. The Elf
responsible for the rover takes a picture of the password (your puzzle input)
and sends it to you via the Digital Sending Network.

Unfortunately, images sent via the Digital Sending Network aren't encoded with
any normal encoding; instead, they're encoded in a special Space Image
Format. None of the Elves seem to remember why this is the case. They send you
the instructions to decode it.

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

    Layer 1: 123
             456

    Layer 2: 789
             012

The image you received is 25 pixels wide and 6 pixels tall.

To make sure the image wasn't corrupted during transmission, the Elves would
like you to find the layer that contains the fewest 0 digits. On that layer,
what is the number of 1 digits multiplied by the number of 2 digits?
"""

def snap(s, n):
    return [s[start:start + n] for start in range(0, len(s), n)]

class SpaceImage:
    def __init__(self, layers):
        self.layers = layers

    @classmethod
    def load(cls, filename, width, height):
        with open(filename) as f:
            text = f.read().strip()
        cls.from_text(width, height, text)

    @classmethod
    def from_text(cls, width, height, text):
        layer_size = width * height
        if len(text) % layer_size != 0:
            raise ValueError("text must be multiple of {} characters".format(layer_size))
        layers = [snap(layer, width) for layer in snap(text, layer_size)]
        return cls(layers)


def main():
    img = SpaceImage.load("puzzle-input.txt")

    def count_digits_in_layer(layer_index, digit):
        return sum(1 for row in img.layers[i] for c in row if c == digit))
    
    # find which layer contains the fewest 0 digits.
    idx = min(range(len(img.layers)),
              key=lambda i: count_digits_in_layer(i, '0'))

    # find the number of 1 digits * number of 2 digits
    print(count_digits_in_layer(idx, '1') * count_digits_in_layer(idx, '2'))


main()
