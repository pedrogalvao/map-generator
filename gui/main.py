import json
import sys
from PyQt5.QtWidgets import QApplication, QMainWindow, QVBoxLayout, QFileDialog, QTabWidget, QSplashScreen
from PyQt5.QtGui import QIcon, QPixmap
from PyQt5.QtCore import Qt
import re
import subprocess
import requests

from initial_menu import InitialMenu
from world_tab import WorldTab
from menu_bar import TopMenuBar
from new_view_menu import NewViewMenu


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

        main_layout = QVBoxLayout()
        self.tabs = QTabWidget()
        self.tabs.setMovable(True)
        self.tabs.setTabsClosable(True)
        self.tabs.tabCloseRequested.connect(self.close_tab)
        self.setStyleSheet("""

            QTabBar::tab {
                background: #ddd;
                padding: 8px;
                min-width: 100px;
            }

            QTabBar::tab:selected {
                background: #fff;
            }

            QTabBar::tab:hover {
                background: #eee;
            }

            QTabBar {
                qproperty-expanding: 1;
            }
        """)
    
        main_layout.addWidget(self.tabs)
        self.setLayout(main_layout)
        self.setCentralWidget(self.tabs)
        self.setMenuBar(TopMenuBar(self))

    def new_tab(self, name):
        current_tab = WorldTab(name, self)
        self.current_world_name = name
        self.tabs.addTab(current_tab, self.current_world_name)
        self.tabs.setCurrentWidget(current_tab)

    def close_tab(self, index):
        self.tabs.removeTab(index)

    def closeEvent(self, event):
        self.backend_process.kill()
        event.accept()

    def load_map(self):
        file_dialog = QFileDialog(self)
        file_dialog.setNameFilter("Binaries or images (*.bin *.png *.jpg )");
        if file_dialog.exec():
            file = file_dialog.selectedFiles()[0]
            headers = {'Content-Type': 'application/json'}
            if file[-4:] == ".bin":
                self.new_tab(file[:-4].split("/")[-1])
                json_data = json.dumps({"world_name":file[:-4].split("/")[-1], "file": file, "shape": "Globe"})
                print(json_data)
                response = requests.post(self.backend_address + "load", data=json_data, headers=headers)
                self.tabs.currentWidget().map_viewer.directory = "out"
                if response.status_code == 200:
                    NewViewMenu(self).exec()
                print(f'View Response status code: {response.status_code}')
                print(f'View Response JSON: {response.json()}')
            elif file[-4:] == ".png":
                self.new_tab(file[:-4].split("/")[-1])
                json_data = json.dumps({"world_name":file[:-4].split("/")[-1], "file": file, "shape": "Globe"})
                print(json_data)
                response = requests.post(self.backend_address + "generate_from_image", data=json_data, headers=headers)
                self.tabs.currentWidget().map_viewer.directory = "out"
                if response.status_code == 200:
                    NewViewMenu(self).exec()
                print(f'View Response status code: {response.status_code}')
                print(f'View Response JSON: {response.json()}')

    def save_map(self):
        options = QFileDialog.Options()
        options |= QFileDialog.DontUseNativeDialog
        filepath, _ = QFileDialog.getSaveFileName(self, 
            "Save File", "", "binary (*.bin)", options = options)
        if filepath:
            headers = {'Content-Type': 'application/json'}
            json_data = json.dumps({"world_name": self.selected_world(), "path": filepath})
            print(json_data)
            response = requests.post(self.backend_address + "save", data=json_data, headers=headers)
            print(f'Save Response status code: {response.status_code}')
            print(f'Save Response JSON: {response.json()}')

    def selected_world(self):
        if len(self.tabs) == 0:
            return None
        return self.tabs.currentWidget().name

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