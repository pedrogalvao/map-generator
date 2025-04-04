from PyQt5.QtWidgets import QLabel, QSizePolicy
from PyQt5.QtGui import QPixmap, QCursor
from PyQt5.QtCore import Qt, QPoint
import re

from sortedcontainers import SortedDict

from legend import generate_heightmap_legend_pixmap
from view_menu import ViewMenu
from constants import COLOR_SCHEMES


class ViewImages:

    def __init__(self):
        self.data = SortedDict()  # Automatically keeps keys sorted
    
    def add(self, rotation, pixmap):
        self.data[rotation] = pixmap
    
    def get(self, number):
        keys = self.data.keys()
        if len(keys) == 0:
            return QPixmap()
        idx = self.data.bisect_left(number)

        # Handle edge cases
        if idx == 0:
            return self.data[keys[0]]
        if idx == len(keys):
            return self.data[keys[-1]]

        # Find closest key
        before = keys[idx - 1]
        after = keys[idx]

        if abs(before - number) <= abs(after - number):
            return self.data[before]
        else:
            return self.data[after]

def natural_sort(file_paths):
    def atoi(text):
        return int(text) if text.isdigit() else text

    def natural_keys(text):
        return [atoi(c) for c in re.split(r'(\d+)', text)]

    return sorted(file_paths, key=natural_keys)


class DraggableResizableLabel(QLabel):
    def __init__(self, pixmap, parent=None):
        super().__init__(parent)
        self.setPixmap(pixmap)
        self.setScaledContents(True)
        self.resize(pixmap.size())  # Default size of the overlay

        # State variables
        self.dragging = False
        self.resizing = False
        self.mouse_start_pos = QPoint()
        self.start_geometry = self.geometry()

    def mousePressEvent(self, event):
        if event.button() == Qt.LeftButton:
            if self.is_in_resize_zone(event.pos()):  # Check if we are in the resize area
                self.resizing = True
            else:
                self.dragging = True
            self.mouse_start_pos = event.globalPos()
            self.start_geometry = self.geometry()

    def mouseMoveEvent(self, event):
        if self.dragging:
            delta = event.globalPos() - self.mouse_start_pos
            self.move(self.start_geometry.topLeft() + delta)
        elif self.resizing:
            delta = event.globalPos() - self.mouse_start_pos
            new_width = max(20, self.start_geometry.width() + delta.x())  # Prevent shrinking too much
            new_height = max(20, self.start_geometry.height() + delta.y())
            self.resize(new_width, new_height)

    def mouseReleaseEvent(self, event):
        self.dragging = False
        self.resizing = False

    def is_in_resize_zone(self, pos):
        """ Check if the mouse is near the bottom-right corner for resizing """
        return pos.x() > self.width() - 15 and pos.y() > self.height() - 15

    def enterEvent(self, event):
        """ Change cursor when hovering over the resize zone """
        if self.is_in_resize_zone(self.mapFromGlobal(QCursor.pos())):
            self.setCursor(Qt.SizeFDiagCursor)
        else:
            self.setCursor(Qt.OpenHandCursor)

    def leaveEvent(self, event):
        self.setCursor(Qt.ArrowCursor)

class MapViewer(QLabel):
    def __init__(self, main_window):
        super().__init__()

        self.view_side_menu = ViewMenu(main_window, self)

        self.setAlignment(Qt.AlignCenter)
        self.setSizePolicy(QSizePolicy.Ignored, QSizePolicy.Ignored)
        self.setStyleSheet("background-color: black;")
        self.zoom_factor = 1
        
        # Create legend image
        # TODO: generate one legend per view
        legend_pixmap = generate_heightmap_legend_pixmap(COLOR_SCHEMES["atlas"]["points"]).scaled(50, 250, Qt.KeepAspectRatio, Qt.SmoothTransformation)
        self.legend_label = DraggableResizableLabel(legend_pixmap, self)
        self.legend_label.setPixmap(legend_pixmap)
        self.legend_label.setScaledContents(True)
        self.legend_label.move(10, self.height() - legend_pixmap.height() - 10)

        self.dragging = False
        self.dragging_right = False
        self.drag_start_position = QPoint()

        self.directory = "img/planet.png"
        self.setFocusPolicy(Qt.StrongFocus)
        self.setFocus()
        self.images = {}
        self.images[""] = ViewImages()
        self.current_latitude = 0
        self.current_longitude = 0
        self.current_index_z = 0
        self.current_view = ""
        self.image_center_x = self.current_image().width() / 2
        self.image_center_y = self.current_image().height() / 2
        self.display_image()

    def select_view(self, view_name:str):
        if len(self.images[view_name].data) > 0:
            self.current_view = view_name
            self.image_center_x = self.current_image().width() / 2
            self.image_center_y = self.current_image().height() / 2
            self.display_image()
        else:
            # TODO: display loading screen
            pass

    # def load_images_thread(self, view:str):
    #     print("Load:")
    #     print(view + "/*.png")
    #     img_paths = natural_sort(glob.glob(view + "/*.png"))
    #     print(img_paths)
    #     view_name = view.split("/")[-1]
    #     for image_path in img_paths:
    #         self.images[view_name].append(QPixmap(image_path))

    # def load_images(self, view:str):
    #     view_name = view.split("/")[-1]
    #     self.images[view_name] = []
    #     t = Thread(target=self.load_images_thread, args=[view])
    #     t.start()

    def current_image(self):
        return self.images[self.current_view].get(self.current_longitude)
        # TODO
        return self.images[self.current_view][self.current_latitude][self.current_longitude][self.current_index_z]

    def current_view_images(self):
        return list(self.images[self.current_view].data.values())

    def display_image(self):
        if self.images[self.current_view]:
            if self.zoom_factor > 1:
                width = self.current_image().width()
                height = self.current_image().height()
                X2 = width / self.zoom_factor
                Y2 = height / self.zoom_factor
                label_width = self.width()
                label_height = self.height()
                if X2 / Y2 > label_width / label_height:
                    Y2 = label_height * X2 / label_width
                elif X2 / Y2 < label_width / label_height:
                    X2 = label_width * Y2 / label_height
                X1 = self.image_center_x - X2 / 2
                Y1 = self.image_center_y - Y2 / 2
                Y1 = max(0, Y1)
                Y1 = min(height - Y2, Y1)
                X1 = max(0, X1)
                X1 = min(width - X2, X1)
                self.image_center_y = Y1 + Y2 / 2
                self.image_center_x = X1 + X2 / 2
                pixmap = self.current_image().copy(int(X1), int(Y1), int(X2), int(Y2))
            else:
                pixmap = self.current_image()
            self.setPixmap(pixmap.scaled(self.size(), Qt.KeepAspectRatio, Qt.SmoothTransformation))
    
    def keyPressEvent(self, event):
        match event.key():
            case Qt.Key_Right:
                self.rotate_east()
            case Qt.Key_Left:
                self.rotate_west()
            case Qt.Key_Up | Qt.Key_Z:
                self.image_center_y -= 20
                self.display_image()
            case Qt.Key_Down | Qt.Key_S:
                self.image_center_y += 20
                self.display_image()
            case Qt.Key_D:
                self.image_center_x += 20
                self.display_image()
            case Qt.Key_Q:
                self.image_center_x -= 20
                self.display_image()

    def rotate_east(self):
        self.current_longitude = (self.current_longitude + 5) % 360
        if self.current_longitude > 180:
            self.current_longitude -= 360
        self.display_image()

    def rotate_west(self):
        self.current_longitude = (self.current_longitude - 5) % 360
        if self.current_longitude > 180:
            self.current_longitude -= 360
        self.display_image()

    def mousePressEvent(self, event):
        if not self.dragging and not self.dragging_right:
            if event.button() == Qt.LeftButton:
                self.start_longitude = self.current_longitude
                self.drag_start_position = event.pos()
                self.last_mouse_position = self.drag_start_position
                self.dragging = True
            elif event.button() == Qt.RightButton:
                self.start_longitude = self.current_longitude
                self.drag_start_position = event.pos()
                self.last_mouse_position = self.drag_start_position
                self.dragging_right = True

    def mouseMoveEvent(self, event):
        if self.dragging:
            # rotate projection
            move = self.drag_start_position.x() - event.pos().x()
            width = self.frameGeometry().width()
            self.current_longitude = self.start_longitude + move * 360 / (width * self.zoom_factor)
            self.current_longitude = self.current_longitude % 360
            if self.current_longitude > 180:
                self.current_longitude -= 360
            
            # move image up and down
            move_y = self.last_mouse_position.y() - event.pos().y()
            self.image_center_y += move_y * 3 / self.zoom_factor
            self.image_center_y = max(self.image_center_y, 0)
            self.image_center_y = min(self.image_center_y, self.current_image().height())

            self.last_mouse_position = event.pos()
            self.display_image()
        elif self.dragging_right:
            # move image to the sides
            move_x = self.last_mouse_position.x() - event.pos().x()
            self.image_center_x += move_x * 3 / self.zoom_factor
            self.image_center_x = max(self.image_center_x, 0)
            self.image_center_x = min(self.image_center_x, self.current_image().width())
            
            # move image up and down
            move_y = self.last_mouse_position.y() - event.pos().y()
            self.image_center_y += move_y * 3 / self.zoom_factor
            self.image_center_y = max(self.image_center_y, 0)
            self.image_center_y = min(self.image_center_y, self.current_image().height())

            self.last_mouse_position = event.pos()
            self.display_image()

    def mouseReleaseEvent(self, event):
        if event.button() == Qt.LeftButton:
            self.dragging = False
        if event.button() == Qt.RightButton:
            self.dragging_right = False

    def resizeEvent(self, event):
        self.display_image()
        super().resizeEvent(event)

    def zoom_image(self, zoom_factor):
        self.zoom_factor = zoom_factor
        self.display_image()

    def wheelEvent(self, event):
        # Zoom factor adjustment based on wheel delta
        zoom_step = 0.002
        self.zoom_factor *= 1.0 - zoom_step * event.angleDelta().y()
        self.zoom_factor = max(self.zoom_factor, 1.0)
        self.zoom_factor = min(self.zoom_factor, self.current_image().width() / 400)
        self.zoom_image(self.zoom_factor)
