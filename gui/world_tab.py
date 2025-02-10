from PyQt5.QtWidgets import QFrame, QVBoxLayout, QHBoxLayout, QWidget, QSplitter
from PyQt5.QtCore import Qt

from viewer import MapViewer
from view_menu import ViewMenu


class WorldTab(QFrame):
    def __init__(self, name, main_window):
        super().__init__()
        self.name = name
        self.map_viewer = MapViewer()

        main_layout = QVBoxLayout()
        main_layout.addWidget(self.map_viewer)

        container = QWidget()
        container.setLayout(main_layout)

        layout = QHBoxLayout()

        self.view_side_menu = ViewMenu(main_window, self)

        splitter = QSplitter(Qt.Horizontal)
        splitter.addWidget(self.view_side_menu)
        splitter.addWidget(container)
        splitter.setSizes([100, 700])
        splitter.setStretchFactor(1, 0) # Side menu keeps its size
        splitter.setStretchFactor(1, 1) # Map stretches

        layout.addWidget(splitter)

        container = QWidget()
        container.setLayout(layout)
        self.setLayout(layout)
        self.map_viewer.display_image()

    def select_view(self, view_name:str):
        self.map_viewer.select_view(view_name)

    def load_images(self, view_name: str):
        self.map_viewer.load_images(view_name)

    def display_image(self, view_name: str):
        self.map_viewer.display_image(view_name)
