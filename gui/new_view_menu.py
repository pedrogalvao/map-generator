import glob
import json
import os
from threading import Thread
from time import sleep
from PIL import Image
from PyQt5.QtWidgets import QLabel, QWidget, QVBoxLayout, QHBoxLayout, QDialog, QColorDialog, QPushButton, QDoubleSpinBox, QDialogButtonBox, QComboBox, QFormLayout, QSpinBox, QCheckBox, QLineEdit
from PyQt5.QtGui import QIcon, QColor

import requests
from constants import COLOR_SCHEMES, LAYERS, PROJECTIONS
from change_color import change_color


class NewViewMenu(QDialog):
    def __init__(self, main_window):
        super().__init__()
        self.main_window = main_window

        self.setWindowIcon(QIcon('img/planet.png'))
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
            response = requests.get(self.main_window.backend_address + "get_size", data=json_data, headers=headers)
            return json.loads(response.text)
        
        size = req_size()
        width = size["width"]
        height = size["height"]

        self.templates_combobox = QComboBox()
        
        self.templates = {}
        template_files = glob.glob("templates/*.json")
        for filepath in template_files:
            with open(filepath, "r") as f:
                template = json.loads(f.read())
                self.templates[template["name"]] = template
        self.templates_combobox.addItems(["Custom"])
        self.templates_combobox.addItems(list(self.templates.keys()))
        self.templates_combobox.currentTextChanged.connect(self.choose_template)
        self.left_layout.addRow("Template", self.templates_combobox)

        self.name_input = QLineEdit()
        self.left_layout.addRow("Name", self.name_input)

        self.projection_combobox = QComboBox()
        self.projection_combobox.addItems(PROJECTIONS)
        self.left_layout.addRow("Projection", self.projection_combobox)

        self.width_input = QSpinBox()
        self.width_input.setRange(10, 40000)
        self.width_input.setValue(width)
        self.left_layout.addRow("Width", self.width_input)

        self.height_input = QSpinBox()
        self.height_input.setRange(10, 20000)
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

        def req_layers():
            headers = {'Content-Type': 'application/json'}
            json_data = json.dumps({"world_name": self.main_window.selected_world()})
            response = requests.get(self.main_window.backend_address + "get_layers", data=json_data, headers=headers)
            return json.loads(response.text)

        self.custom_layers = req_layers()

        for layer_name in self.custom_layers + LAYERS:
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
        self.center_latitude_input.setValue(0.0)
        self.rotation_input.setValue(0.0)
        template_name = self.templates_combobox.currentText()
        if template_name in self.templates:
            selected_template = self.templates[template_name]
            view_names = list(self.main_window.tabs.currentWidget().map_viewer.images)
            if template_name in view_names:
                new_view_name = template_name + " (2)"
                i = 2
                while new_view_name in view_names:
                    new_view_name = template_name + " (" + str(i) + ")"
                    i += 1
                self.name_input.setText(new_view_name)
            else:
                self.name_input.setText(template_name)
            if "land_color" in selected_template:
                self.land_color = selected_template["land_color"]
                self.land_color_button.setText(self.land_color)
                self.land_color_button.setStyleSheet("background-color:" + self.land_color)
            if "water_color" in selected_template:
                self.water_color = selected_template["water_color"]
                self.water_color_button.setText(self.water_color)
                self.water_color_button.setStyleSheet("background-color:" + self.water_color)
            if "contour_color" in selected_template:
                self.contour_color = selected_template["contour_color"]
                self.contour_color_button.setText(self.contour_color)
                self.contour_color_button.setStyleSheet("background-color:" + self.contour_color)
            if "parallels_color" in selected_template:
                self.parallels_color = selected_template["parallels_color"]
                self.parallels_color_button.setText(self.parallels_color)
                self.parallels_color_button.setStyleSheet("background-color:" + self.parallels_color)
            if "parallels_interval" in selected_template:
                self.parallels_interval_input.setValue(int(selected_template["parallels_interval"]))
            if "height_colors" in selected_template:
                self.height_color_combobox.setCurrentText(selected_template["height_colors"])
            if "projection" in selected_template:
                self.projection_combobox.setCurrentText(selected_template["projection"])
            if "center_latitude" in selected_template:
                self.center_latitude_input.setValue(selected_template["center_latitude"])
            if "rotation" in selected_template:
                self.rotation_input.setValue(selected_template["rotation"])
            for layer_name in self.layers_checkboxes:
                if "layers" in selected_template and layer_name in selected_template["layers"]:
                    self.layers_checkboxes[layer_name].setChecked(True)
                else:
                    self.layers_checkboxes[layer_name].setChecked(False)
            if "width" in selected_template:
                self.width_input.setValue(selected_template["width"])
            if "height" in selected_template:
                self.height_input.setValue(selected_template["height"])

    def request_view(self):
        layers = []
        for layer_name in self.custom_layers + LAYERS:
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
            "request_priority": 1,
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

