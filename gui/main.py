import json
import sys
from PyQt5.QtWidgets import QApplication, QMainWindow, QVBoxLayout, QHBoxLayout, QWidget, QFileDialog, QSplitter, QTabWidget, QSplashScreen
from PyQt5.QtGui import QIcon, QPixmap
from PyQt5.QtCore import Qt
import re
import subprocess
import requests

from initial_menu import InitialMenu
from menu_bar import TopMenuBar
from viewer import MapViewer
from view_menu import ViewMenu


IMAGES_DIR_PATH = "images/earth"


def natural_sort(file_paths):
    def atoi(text):
        return int(text) if text.isdigit() else text

    def natural_keys(text):
        return [atoi(c) for c in re.split(r'(\d+)', text)]

    return sorted(file_paths, key=natural_keys)


class MapMaker(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowIcon(QIcon('img/rose.png'))
        self.setWindowTitle("Map Maker")
        self.setGeometry(100, 100, 800, 600)

        self.backend_process = subprocess.Popen(["../target/release/rest_api.exe"])
        self.backend_address = "http://127.0.0.1:8000/"

        self.map_viewer = MapViewer()

        main_layout = QVBoxLayout()
        main_layout.addWidget(self.map_viewer)

        container = QWidget()
        container.setLayout(main_layout)

        layout = QHBoxLayout()

        side_menu_tabs = QTabWidget()
        self.view_side_menu = ViewMenu(self)
        side_menu_tabs.addTab(self.view_side_menu, "View")

        splitter = QSplitter(Qt.Horizontal)
        splitter.addWidget(side_menu_tabs)
        splitter.addWidget(container)
        splitter.setSizes([100, 700])
        splitter.setStretchFactor(1, 0) # Side menu keeps its size
        splitter.setStretchFactor(1, 1) # Map stretches

        layout.addWidget(splitter)

        container = QWidget()
        container.setLayout(layout)

        self.setCentralWidget(container)

        self.setMenuBar(TopMenuBar(self))
        self.map_viewer.display_image()

    def load_map(self):
        file_dialog = QFileDialog(self)
        file_dialog.setNameFilter("Binaries or images (*.bin *.png *.jpg )");
        if file_dialog.exec():
            file = file_dialog.selectedFiles()[0]
            headers = {'Content-Type': 'application/json'}
            if file[-4:] == ".bin":
                json_data = json.dumps({"file": file, "shape": "Globe"})
                print(json_data)
                response = requests.post(self.backend_address + "load", data=json_data, headers=headers)
                self.map_viewer.directory = "out"
                print(f'View Response status code: {response.status_code}')
                print(f'View Response JSON: {response.json()}')
            elif file[-4:] == ".png":
                json_data = json.dumps({"file": file, "shape": "Globe"})
                print(json_data)
                response = requests.post(self.backend_address + "generate_from_image", data=json_data, headers=headers)
                self.map_viewer.directory = "out"
                print(f'View Response status code: {response.status_code}')
                print(f'View Response JSON: {response.json()}')

    def save_map(self):
        options = QFileDialog.Options()
        options |= QFileDialog.DontUseNativeDialog
        filepath, _ = QFileDialog.getSaveFileName(self, 
            "Save File", "", "binary (*.bin)", options = options)
        if filepath:
            headers = {'Content-Type': 'application/json'}
            json_data = json.dumps({"path": filepath})
            print(json_data)
            response = requests.post(self.backend_address + "save", data=json_data, headers=headers)
            print(f'Save Response status code: {response.status_code}')
            print(f'Save Response JSON: {response.json()}')

    def select_view(self, view_name:str):
        self.map_viewer.select_view(view_name)

    def load_images(self, view_name: str):
        self.map_viewer.load_images(view_name)

    def display_image(self, view_name: str):
        self.map_viewer.display_image(view_name)

    def closeEvent(self, event):
        self.backend_process.kill()
        event.accept()


def main():
    app = QApplication(sys.argv)

    # window = InitialMenu()
    # window.show()

    pixmap = QPixmap("img/rose.png")  # Load your splash image
    splash = QSplashScreen(pixmap)
    splash.show()
    app.processEvents()  # Process events to receive mouse clicks

    # Perform initialization tasks here
    window = MapMaker()
    window.show()

    splash.finish(window)  # Hide the splash screen
    return app.exec_()

if __name__ == "__main__":
    main()