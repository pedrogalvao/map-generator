import glob
import json
import os
from threading import Thread
from time import sleep
from PIL import Image
from PyQt5.QtWidgets import QLabel, QWidget, QVBoxLayout, QHBoxLayout, QDialog, QColorDialog, QPushButton, QDoubleSpinBox, QDialogButtonBox, QComboBox, QFormLayout, QSpinBox, QCheckBox, QLineEdit
from PyQt5.QtGui import QColor

import requests
from constants import COLOR_SCHEMES, LAYERS, PROJECTIONS
from change_color import change_color


class NewViewMenu(QDialog):
    def __init__(self, main_window):
        super().__init__()
        self.main_window = main_window

        self.setWindowTitle("Create New View")

        self.layout = QHBoxLayout()

        QBtn = QDialogButtonBox.Ok | QDialogButtonBox.Cancel

        self.buttonBox = QDialogButtonBox(QBtn)
        self.buttonBox.accepted.connect(self.request_view)
        self.buttonBox.rejected.connect(self.reject)

        self.left_layout = QFormLayout(self)

        def req_size():
            headers = {'Content-Type': 'application/json'}
            json_data = json.dumps({"world_name": self.main_window.selected_world()})
            print(json_data)
            response = requests.get(self.main_window.backend_address + "get_size", data=json_data, headers=headers)
            return json.loads(response.text)
        
        size = req_size()
        print()
        width = size["width"]
        height = size["height"]

        self.templates_combobox = QComboBox()
        self.templates_combobox.addItems(["Custom", "Atlas Height", "Orthographic Height", "Climate", "Old Azimuthal", "Old Equirectangular"])
        self.templates_combobox.currentTextChanged.connect(self.choose_template)
        self.left_layout.addRow("Template", self.templates_combobox)

        self.name_input = QLineEdit()
        self.left_layout.addRow("Name", self.name_input)

        self.projection_combobox = QComboBox()
        self.projection_combobox.addItems(PROJECTIONS)
        self.left_layout.addRow("Projection", self.projection_combobox)

        self.width_input = QSpinBox()
        self.width_input.setRange(10, 10000)
        self.width_input.setValue(width)
        self.left_layout.addRow("Width", self.width_input)

        self.height_input = QSpinBox()
        self.height_input.setRange(10, 5000)
        self.height_input.setValue(height)
        self.left_layout.addRow("Height", self.height_input)

        self.parallels_interval_input = QSpinBox()
        self.parallels_interval_input.setRange(0, 360)
        self.parallels_interval_input.setValue(30)
        self.left_layout.addRow("Parallels Interval", self.parallels_interval_input)

        self.center_latitude_input = QDoubleSpinBox()
        self.center_latitude_input.setRange(-90, 90)
        self.center_latitude_input.setValue(0)
        self.left_layout.addRow("Center Latitude", self.center_latitude_input)
    
        self.rotation_input = QDoubleSpinBox()
        self.rotation_input.setRange(-180, 180)
        self.rotation_input.setValue(0)
        self.left_layout.addRow("Rotation", self.rotation_input)

        self.land_color_button = QPushButton("#00000000")
        self.land_color_button.clicked.connect(self.choose_land_color)
        self.land_color = "#00000000"
        self.left_layout.addRow("Land Color", self.land_color_button)

        self.water_color_button = QPushButton("#00000000")
        self.water_color_button.clicked.connect(self.choose_water_color)
        self.water_color = "#00000000"
        self.left_layout.addRow("Water Color", self.water_color_button)

        self.contour_color_button = QPushButton("#00000000")
        self.contour_color_button.clicked.connect(self.choose_contour_color)
        self.contour_color = "#00000000"
        self.left_layout.addRow("Contour Color", self.contour_color_button)

        self.parallels_color_button = QPushButton("#00000000")
        self.parallels_color_button.clicked.connect(self.choose_parallels_color)
        self.parallels_color = "#00000000"
        self.left_layout.addRow("Parallels Color", self.parallels_color_button)

        self.height_color_combobox = QComboBox()
        self.height_color_combobox.addItems(list(COLOR_SCHEMES.keys()))
        self.left_layout.addRow("Height Color Scheme", self.height_color_combobox)

        self.right_layout = QVBoxLayout(self)
        self.right_layout.addWidget(QLabel("Layers:"))
        self.layers_checkboxes = {}
        for layer_name in LAYERS:
            if layer_name.lower() in ["contour", "parallels and meridians"]:
                continue
            self.layers_checkboxes[layer_name] = QCheckBox(layer_name)
            self.right_layout.addWidget(self.layers_checkboxes[layer_name])

        self.right_layout.addWidget(self.buttonBox)
    
        self.right_widget = QWidget()
        self.right_widget.setLayout(self.right_layout)
        self.left_widget = QWidget()
        self.left_widget.setLayout(self.left_layout)
        self.layout.addWidget(self.left_widget)
        self.layout.addWidget(self.right_widget)
        self.setLayout(self.layout)
    
    def choose_parallels_color(self):
        dialog = QColorDialog()
        dialog.setOption(QColorDialog.ShowAlphaChannel, on=True)
        self.parallels_color = dialog.getColor(options=QColorDialog.ShowAlphaChannel).name(QColor.HexArgb)
        self.parallels_color_button.setText(self.parallels_color)
        self.parallels_color_button.setStyleSheet("background-color:" + self.parallels_color)

    def choose_contour_color(self):
        dialog = QColorDialog()
        dialog.setOption(QColorDialog.ShowAlphaChannel, on=True)
        self.contour_color = dialog.getColor(options=QColorDialog.ShowAlphaChannel).name(QColor.HexArgb)
        self.contour_color_button.setText(self.contour_color)
        self.contour_color_button.setStyleSheet("background-color:" + self.contour_color)

    def choose_water_color(self):
        dialog = QColorDialog()
        dialog.setOption(QColorDialog.ShowAlphaChannel, on=True)
        self.water_color = dialog.getColor(options=QColorDialog.ShowAlphaChannel).name(QColor.HexArgb)
        self.water_color_button.setText(self.water_color)
        self.water_color_button.setStyleSheet("background-color:" + self.water_color)

    def choose_land_color(self):
        dialog = QColorDialog()
        dialog.setOption(QColorDialog.ShowAlphaChannel, on=True)
        self.land_color = dialog.getColor(options=QColorDialog.ShowAlphaChannel).name(QColor.HexArgb)
        self.land_color_button.setText(self.land_color)
        self.land_color_button.setStyleSheet("background-color:" + self.land_color)

    def choose_template(self):
        print("choose_template")
        self.center_latitude_input.setValue(0.0)
        self.rotation_input.setValue(0.0)
        if self.templates_combobox.currentText() == "Atlas Height":
            self.name_input.setText("Atlas Height")
            self.land_color = "#00000000"
            self.land_color_button.setText(self.land_color)
            self.land_color_button.setStyleSheet("background-color:" + self.land_color)
            self.water_color = "#00000000"
            self.water_color_button.setText(self.water_color)
            self.water_color_button.setStyleSheet("background-color:" + self.water_color)
            self.contour_color = "#ff000000"
            self.contour_color_button.setText(self.contour_color)
            self.contour_color_button.setStyleSheet("background-color:" + self.contour_color)
            self.parallels_color = "#8fffffff"
            self.parallels_color_button.setText(self.parallels_color)
            self.parallels_color_button.setStyleSheet("background-color:" + self.parallels_color)
            self.height_color_combobox.setCurrentText("Atlas2")
            for layer_name in self.layers_checkboxes:
                self.layers_checkboxes[layer_name].setChecked(False)
        elif self.templates_combobox.currentText() == "Orthographic Height":
            self.name_input.setText("Orthographic Height")
            self.land_color = "#00000000"
            self.land_color_button.setText(self.land_color)
            self.land_color_button.setStyleSheet("background-color:" + self.land_color)
            self.water_color = "#00000000"
            self.water_color_button.setText(self.water_color)
            self.water_color_button.setStyleSheet("background-color:" + self.water_color)
            self.contour_color = "#00000000"
            self.contour_color_button.setText(self.contour_color)
            self.contour_color_button.setStyleSheet("background-color:" + self.contour_color)
            self.parallels_color = "#00000000"
            self.parallels_color_button.setText(self.parallels_color)
            self.parallels_color_button.setStyleSheet("background-color:" + self.parallels_color)
            self.projection_combobox.setCurrentText("double orthographic")
            self.height_color_combobox.setCurrentText("Atlas Dark Water")
            self.center_latitude_input.setValue(25.0)
            self.rotation_input.setValue(25.0)
            for layer_name in self.layers_checkboxes:
                self.layers_checkboxes[layer_name].setChecked(False)
        elif self.templates_combobox.currentText() == "Climate":
            self.name_input.setText("Climate")
            self.land_color = "#00000000"
            self.land_color_button.setText(self.land_color)
            self.land_color_button.setStyleSheet("background-color:" + self.land_color)
            self.water_color = "#00000000"
            self.water_color_button.setText(self.water_color)
            self.water_color_button.setStyleSheet("background-color:" + self.water_color)
            self.contour_color = "#ff000000"
            self.contour_color_button.setText(self.contour_color)
            self.contour_color_button.setStyleSheet("background-color:" + self.contour_color)
            self.parallels_color = "#8fffffff"
            self.parallels_color_button.setText(self.parallels_color)
            self.parallels_color_button.setStyleSheet("background-color:" + self.parallels_color)
            self.height_color_combobox.setCurrentText("Atlas2")
            for layer_name in self.layers_checkboxes:
                self.layers_checkboxes[layer_name].setChecked(False)
            self.layers_checkboxes["Climate"].setChecked(True)
        elif self.templates_combobox.currentText() == "Old Azimuthal":
            self.name_input.setText("Old Azimuthal")
            self.width_input.setValue(3000)
            self.height_input.setValue(1500)
            self.projection_combobox.setCurrentText("double azimuthal")
            self.land_color = "#fffff3c7"
            self.land_color_button.setText(self.land_color)
            self.land_color_button.setStyleSheet("background-color:" + self.land_color)
            self.water_color = "#fff0ddb8"
            self.water_color_button.setText(self.water_color)
            self.water_color_button.setStyleSheet("background-color:" + self.water_color)
            self.contour_color = "#ff000000"
            self.contour_color_button.setText(self.contour_color)
            self.contour_color_button.setStyleSheet("background-color:" + self.contour_color)
            self.parallels_color = "#8f000000"
            self.parallels_color_button.setText(self.parallels_color)
            self.parallels_color_button.setStyleSheet("background-color:" + self.parallels_color)
            self.height_color_combobox.setCurrentText("None")
            for layer_name in self.layers_checkboxes:
                self.layers_checkboxes[layer_name].setChecked(False)
            self.layers_checkboxes["Paper Texture"].setChecked(True)
            self.layers_checkboxes["Mountains"].setChecked(True)
        elif self.templates_combobox.currentText() == "Old Equirectangular":
            self.name_input.setText("Old Equirectangular")
            self.width_input.setValue(3000)
            self.height_input.setValue(1500)
            self.projection_combobox.setCurrentText("equirectangular")
            self.land_color = "#fffff3c7"
            self.land_color_button.setText(self.land_color)
            self.land_color_button.setStyleSheet("background-color:" + self.land_color)
            self.water_color = "#fff0ddb8"
            self.water_color_button.setText(self.water_color)
            self.water_color_button.setStyleSheet("background-color:" + self.water_color)
            self.contour_color = "#ff000000"
            self.contour_color_button.setText(self.contour_color)
            self.contour_color_button.setStyleSheet("background-color:" + self.contour_color)
            self.parallels_color = "#8f000000"
            self.parallels_color_button.setText(self.parallels_color)
            self.parallels_color_button.setStyleSheet("background-color:" + self.parallels_color)
            self.height_color_combobox.setCurrentText("None")
            for layer_name in self.layers_checkboxes:
                self.layers_checkboxes[layer_name].setChecked(False)
            self.layers_checkboxes["Paper Texture"].setChecked(True)
            self.layers_checkboxes["Rhumb Lines"].setChecked(True)
            self.layers_checkboxes["Mountains"].setChecked(True)

    def request_view(self):
        layers = []
        for layer_name in LAYERS:
            if layer_name.lower() == "contour":
                if self.contour_color != "#00000000":
                    layers.append("contour")
            elif layer_name.lower().replace(" ", "_") == "parallels_and_meridians":
                if self.parallels_color != "#00000000":
                    layers.append("parallels_and_meridians")
            elif self.layers_checkboxes[layer_name].isChecked():
                layer_name2 = layer_name.lower().replace(" ", "_")
                layers.append(layer_name2)

        if "mountains" in layers:
            if not os.path.exists("img2/"): 
                os.makedirs("img2/")
            for filename in ["hill.png", "mountain1.png", "mountain2.png"]:
                old_color = (160, 140, 100, 255)
                new_color = [0, 0, 0, 0]
                new_color[3] = int(self.land_color[1:3], 16)
                new_color[0] = int(self.land_color[3:5], 16)
                new_color[1] = int(self.land_color[5:7], 16)
                new_color[2] = int(self.land_color[7:9], 16)
                img = Image.open('img/' + filename).convert("RGBA")
                new_img = change_color(img, old_color, new_color)
                new_img.save('img2/' + filename)

        files_to_remove = glob.glob("out/" + self.name_input.text() + "/*.png")
        for filePath in files_to_remove:
            try:
                os.remove(filePath)
            except OSError:
                pass
        view_config = {
            "resolution": [self.width_input.value(), self.height_input.value()],
            "center": [self.center_latitude_input.value(), 0.0],
            "rotation": self.rotation_input.value(),
            "land_color": str(self.land_color),
            "water_color": str(self.water_color),
            "contour_color": str(self.contour_color),
            "layers": layers,
            "projection": self.projection_combobox.currentText(),
            "output_path": "out/" + self.name_input.text(),
            "rotation_frames": 1,
            "parallels_interval": self.parallels_interval_input.value(),
            "parallels_color": str(self.parallels_color),
            "height_colors": COLOR_SCHEMES[self.height_color_combobox.currentText()]
        }
        req_data = {
            "world_name":self.main_window.selected_world(),
            "params":view_config
        }
        
        view_name = self.name_input.text()
        req_view_thread = Thread(target=self.main_window.tabs.currentWidget().request_view, args=[view_name, req_data])
        req_view_thread.start()
        
        images = self.main_window.tabs.currentWidget().map_viewer.images
        while view_name not in images or len(images[view_name].data) == 0:
            sleep(0.2)
        self.main_window.tabs.currentWidget().map_viewer.view_side_menu.add_view_option(self.name_input.text())
        self.main_window.tabs.currentWidget().map_viewer.view_side_menu.select_view(self.name_input.text())
        self.accept()

