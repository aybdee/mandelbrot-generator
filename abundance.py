from PIL import Image
import colorsys
import math
import os

# frame parameters
width = 100  # pixels
x = -0.65
y = 0
xRange = 3.4
aspectRatio = 4 / 3

precision = 100

height = round(width / aspectRatio)
yRange = xRange / aspectRatio
# minX = x - xRange / 2
# maxX = x + xRange / 2
# minY = y - yRange / 2
# maxY = y + yRange / 2

minX = -1.5
maxX = 2.0
minY = -1.12
maxY = 1.12

img = Image.new("RGB", (width, height), color="black")
pixels = img.load()


def is_divergent(x, y, max_iterations):
    x_curr = 0.0
    y_curr = 0.0
    for i in range(max_iterations):
        x_temp = (x_curr**2 - y_curr**2) + x
        y_temp = (2 * x_curr * y_curr) + y
        x_curr = x_temp
        y_curr = y_temp
        if x_curr**2 + y_curr**2 > 4.0:
            return True
    return False


def scale_x_value(range, value, x_min, x_max):
    MAX_X = x_max
    MIN_X = x_min
    div = (MAX_X - MIN_X) / range
    return MIN_X + (div * float(value))


def scale_y_value(range, value, y_min, y_max):
    MAX_Y = y_max
    MIN_Y = y_min
    div = (MAX_Y - MIN_Y) / range
    return MIN_Y + (div * float(value))


def logColor(distance, base, const, scale):
    color = -1 * math.log(distance, base)
    rgb = colorsys.hsv_to_rgb(const + scale * color, 0.8, 0.9)
    return tuple(round(i * 255) for i in rgb)


def powerColor(distance, exp, const, scale):
    color = distance**exp
    rgb = colorsys.hsv_to_rgb(const + scale * color, 1 - 0.6 * color, 0.9)
    return tuple(round(i * 255) for i in rgb)


for row in range(height):
    for col in range(width):
        x_0 = scale_x_value(width, col, minX, maxX)
        y_0 = scale_y_value(height, row, minY, maxY)
        diverges = not (is_divergent(x_0, y_0, 100))
        if diverges:
            pixels[col, row] = (0, 0, 0)
        else:
            pixels[col, row] = (255, 255, 255)


img.save("output.png")
