from PIL import Image
import copy


def change_color(img, old_color, new_color):
    new_img = copy.deepcopy(img)
    width, height = new_img.size
    for y in range(0, height):
        for x in range(0, width):
            pixel = new_img.getpixel((x, y))
            if pixel[0] != 0 or pixel[0] != 0 or pixel[2] != 0:
                new_pixel_color = [0, 0, 0, pixel[3]]
                for i in range(3):
                    new_pixel_color[i] = int(new_color[i] * pixel[i] / old_color[i])
                new_img.putpixel((x, y), tuple(new_pixel_color))
    return new_img
