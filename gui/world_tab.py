from PyQt5.QtWidgets import QFrame, QVBoxLayout, QHBoxLayout, QWidget, QPushButton
from PyQt5.QtCore import Qt

from viewer import MapViewer
from view_menu import ViewMenu


class WorldTab(QFrame):
    def __init__(self, name, main_window):
        super().__init__()

        self.name = name
        self.map_viewer = MapViewer(main_window)

        main_layout = QVBoxLayout()
        main_layout.addWidget(self.map_viewer)

        container = QWidget()
        container.setLayout(main_layout)
    
        self.setLayout(main_layout)
        self.map_viewer.display_image()

    def select_view(self, view_name:str):
        self.map_viewer.select_view(view_name)

    def load_images(self, view_name: str):
        self.map_viewer.load_images(view_name)

    def display_image(self, view_name: str):
        self.map_viewer.display_image(view_name)
