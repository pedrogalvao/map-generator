import glob
import shutil
import sys
from threading import Thread
from time import sleep
import requests
import json
import re
from PyQt5.QtWidgets import QFormLayout, QCheckBox, QDialog, QDialogButtonBox, QComboBox, QApplication, QPushButton, QWidget, QLineEdit, QHBoxLayout, QSlider, QDoubleSpinBox, QSpinBox, QFileDialog
from PyQt5.QtGui import QPixmap, QCursor
from PyQt5.QtCore import pyqtSignal, Qt


def is_empty_image(qpixmap):
    if qpixmap.isNull():
        return True
    image = qpixmap.toImage()
    try:
        if image.pixel(0, 0) == (0,0,0,1) or image.pixel(0, 0) == (0,0,0,0):
            return True
        else:
            return False
    except:
        return True

def natural_sort(file_paths):
    def atoi(text):
        return int(text) if text.isdigit() else text

    def natural_keys(text):
        return [atoi(c) for c in re.split(r'(\d+)', text)]

    return sorted(file_paths, key=natural_keys)


class GenerationMenu(QDialog):
    image_loaded = pyqtSignal(QPixmap)
    def __init__(self, main_window):
        super().__init__()
        self.main_window = main_window

        self.setWindowFlags(self.windowFlags() & ~Qt.WindowContextHelpButtonHint)
        self.setWindowTitle("Create New World")

        self.menu_layout = QFormLayout()
        self.seed_input = QLineEdit()
        self.menu_layout.addRow("Seed", self.seed_input)

        self.shape_combobox = QComboBox()
        self.shape_combobox.addItems(["Globe", "Cylinder", "Flat"])
        self.menu_layout.addRow("Shape", self.shape_combobox)
        self.options = {}

        self.add_float_option("Water Percentage", 0, 100, 71)
        self.add_int_option("No. Plates", 1, 50, 25)
        self.add_float_option("Islands", 0, 10, 1)

        self.add_resolution_option()

        self.erosion_input = QSpinBox()
        self.erosion_input.setValue(0)
        self.menu_layout.addRow("Erosion Iterations", self.erosion_input)

        self.supercontinent_checkbox = QCheckBox("Supercontinent")
        self.menu_layout.addWidget(self.supercontinent_checkbox)
        self.climate_checkbox = QCheckBox("Climate")
        self.menu_layout.addWidget(self.climate_checkbox)
        # self.erosion_checkbox = QCheckBox("Erosion")
        # self.menu_layout.addWidget(self.erosion_checkbox)
        self.hotspots_checkbox = QCheckBox("Hotspots")
        self.menu_layout.addWidget(self.hotspots_checkbox)
        self.generation_button = QPushButton("Generate Map", self)
        self.generation_button.setCursor(QCursor(Qt.PointingHandCursor))
        self.generation_button.clicked.connect(self.send_request)
        self.menu_layout.addWidget(self.generation_button)

        self.setLayout(self.menu_layout)

    def load_map(self):
        file_dialog = QFileDialog(self)
        file_dialog.setNameFilter("binary (*.bin)");
        if file_dialog.exec():
            file = file_dialog.selectedFiles()[0]
            headers = {'Content-Type': 'application/json'}
            json_data = json.dumps({"file": file, "shape": self.shape_combobox.currentText()})
            print(json_data)
            response = requests.post(self.main_window.backend_address + "load", data=json_data, headers=headers)
            self.main_window.directory = "out"
            print(f'View Response status code: {response.status_code}')
            print(f'View Response JSON: {response.json()}')

    def load_map_from_height_image(self):
        file_dialog = QFileDialog(self)
        file_dialog.setNameFilter("PNG (*.png)");
        if file_dialog.exec():
            file = file_dialog.selectedFiles()[0]
            headers = {'Content-Type': 'application/json'}
            json_data = json.dumps({"file": file, "shape": self.shape_combobox.currentText()})
            print(json_data)
            response = requests.post(self.main_window.backend_address + "generate_from_image", data=json_data, headers=headers)
            self.main_window.directory = "out"
            print(f'View Response status code: {response.status_code}')
            print(f'View Response JSON: {response.json()}')

    def setup_float_input_slider(self, spinbox, slider, min_value, max_value, default_value):
        spinbox.setRange(min_value, max_value)
        slider.setRange(min_value * 100, max_value * 100)
        
        # Synchronize the spinbox and slider
        spinbox.valueChanged.connect(lambda value: slider.setValue(int(round(value, 1) * 100)))
        slider.valueChanged.connect(lambda value: spinbox.setValue(round(value, 1) / 100.0))
        spinbox.setValue(default_value)

    def setup_int_input_slider(self, spinbox, slider, min_value, max_value, default_value):
        spinbox.setRange(min_value, max_value)
        slider.setRange(min_value, max_value)
        
        # Synchronize the spinbox and slider
        spinbox.valueChanged.connect(lambda value: slider.setValue(int(round(value))))
        slider.valueChanged.connect(lambda value: spinbox.setValue(int(value)))
        spinbox.setValue(default_value)
    
    def add_float_option(self, name, min_val, max_val, default_val):
        widget = QWidget()
        layout_h = QHBoxLayout(widget)

        self.options[name] = QDoubleSpinBox()
        slider = QSlider(Qt.Horizontal)
        layout_h.addWidget(slider)
        layout_h.addWidget(self.options[name])

        self.setup_float_input_slider(self.options[name], slider, min_val, max_val, default_val)
        
        # self.menu_layout.addLayout(option_layout)
        self.menu_layout.addRow(name, widget)

    def add_int_option(self, name, min_val, max_val, default_val):
        widget = QWidget()
        layout_h = QHBoxLayout(widget)

        self.options[name] = QSpinBox()
        slider = QSlider(Qt.Horizontal)
        layout_h.addWidget(slider)
        layout_h.addWidget(self.options[name])

        self.setup_int_input_slider(self.options[name], slider, min_val, max_val, default_val)
        
        # self.menu_layout.addLayout(option_layout)
        self.menu_layout.addRow(name, widget)

    def add_resolution_option(self):
        self.resolution_widget = QWidget()
        layout_h = QHBoxLayout(self.resolution_widget)
        self.width_input = QSpinBox()
        self.width_input.setRange(10, 5000)
        self.width_input.setValue(1000)
        self.height_input = QSpinBox()
        self.height_input.setRange(10, 2500)
        self.height_input.setValue(500)
        layout_h.addWidget(self.width_input)
        layout_h.addWidget(self.height_input)
        self.menu_layout.addRow("Resolution", self.resolution_widget)

    def prepare_config(self):
        config = {
            "shape": "Globe",
            "seed": 1,
            "height_pixels": self.height_input.value(),
            "width_pixels": self.width_input.value(),
            "number_of_plates": 15,
            "water_percentage": 71.0,
            "land_height_percentiles": [[0.0, 0], [50.0, 250], [75.0, 500], [89.0, 1000], [93.0, 1500], [98.0, 2000], [99.0, 3500], [100.0, 6500]],
            "ocean_depth_percentiles": [[0.0, -10000], [5.0, -4000], [60.0, -2000], [80.0, -200], [100.0, 0]],
            "precipitation_percentiles": [[5.0, 0], [15.0, 20], [25.0, 35], [35.0, 50], [65.0, 70], [88.0, 150], [100.0, 250]],
            "number_of_rivers": 80,
        }

        config["seed"] = int(hash(self.seed_input.text())) & (2**32 - 1)
        config["shape"] = self.shape_combobox.currentText()
        config["number_of_plates"] = self.options["No. Plates"].value()
        config["water_percentage"] = self.options["Water Percentage"].value()
        config["islands"] = self.options["Islands"].value()
        config["supercontinent"] = self.supercontinent_checkbox.isChecked()
        config["make_climate"] = self.climate_checkbox.isChecked()
        config["erosion_iterations"] = self.erosion_input.value()
        config["hotspots"] = 50 if self.hotspots_checkbox.isChecked() else 0

        print(config)
        json_data = json.dumps(config)
        return json_data

    def send_request_thread(self):
        json_data = self.prepare_config()
        headers = {'Content-Type': 'application/json'}
        try:
            response = requests.post(self.main_window.backend_address + "generate", data=json_data, headers=headers)
            print(f'Response status code: {response.status_code}')
            print(f'Response JSON: {response.json()}')
            self.main_window.directory = "out/"
            # for layer in ["height", "climate", "rivers", "contour"]:
        except requests.exceptions.ConnectionError as e:
            print(f'Could not connect to server')
            print(e)
        self.done = True

    def display_pipeline_images_thread(self):
        try:
            shutil.rmtree("out/pipeline/")
        except FileNotFoundError:
            pass
        prev_path = ""
        count = 0
        while not self.done:
            pipeline_imgs_paths = natural_sort(glob.glob("out/pipeline/*.png"))
            if len(pipeline_imgs_paths) > 0:
                if prev_path == pipeline_imgs_paths[-1]:
                    sleep(0.3)
                    continue
                last_img_path = pipeline_imgs_paths[-1]
                last_image = QPixmap(last_img_path)
                if is_empty_image(last_image):
                    sleep(0.3)
                    continue
                # for view in self.main_window.images:
                self.main_window.current_projection = "pipeline"
                self.main_window.images["pipeline"] = [last_image]
                self.image_loaded.emit(last_image)
                prev_path = pipeline_imgs_paths[-1]
            sleep(0.3)
            count += 1
        print("count:", count)
    
    def send_request(self):
        self.done = False
        self.image_loaded.connect(self.main_window.display_image)
        t1 = Thread(target=self.send_request_thread, args=[])
        # self.main_window.start_loading_images()
        t2 = Thread(target=self.display_pipeline_images_thread, args=[])
        t2.start()
        t1.start()
        t1.join()
        t2.join()
        self.accept()


if __name__ == "__main__":
    app = QApplication(sys.argv)
    
    # Create the main window
    window = GenerationMenu()
    window.show()
    
    # Run the application
    sys.exit(app.exec_())
