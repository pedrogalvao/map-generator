import json
from PyQt5.QtWidgets import QDialog, QVBoxLayout, QDialogButtonBox, QLabel, QDoubleSpinBox, QSlider, QHBoxLayout
from PyQt5.QtCore import Qt

import requests


BACKEND_ADDRESS = "http://127.0.0.1:8000/"


def translation_noise_request():
    headers = {'Content-Type': 'application/json'}
    response = requests.post(BACKEND_ADDRESS + "translation_noise", headers=headers)
    print(f'Translation Noise Response status code: {response.status_code}')

def erosion_request():
    headers = {'Content-Type': 'application/json'}
    response = requests.post(BACKEND_ADDRESS + "erosion", headers=headers)
    print(f'Erosion Response status code: {response.status_code}')

def smooth_request():
    headers = {'Content-Type': 'application/json'}
    response = requests.post(BACKEND_ADDRESS + "smooth", headers=headers)
    print(f'Smooth Response status code: {response.status_code}')

def add_noise_request():
    headers = {'Content-Type': 'application/json'}
    response = requests.post(BACKEND_ADDRESS + "add_noise", headers=headers)
    print(f'Add Noise Response status code: {response.status_code}')


class OperationDialog(QDialog):
    def __init__(self, main_window):
        super().__init__()
        self.main_window = main_window
        QBtn = QDialogButtonBox.Ok | QDialogButtonBox.Cancel
        self.buttonBox = QDialogButtonBox(QBtn)
        self.buttonBox.accepted.connect(self.send_request)
        self.buttonBox.rejected.connect(self.reject)
        self.layout = QVBoxLayout(self)
        self.options = {}

    def setup_float_input_slider(self, spinbox, slider, min_value, max_value, default_value):
        spinbox.setRange(min_value, max_value)
        slider.setRange(min_value * 100, max_value * 100)
        
        # Synchronize the spinbox and slider
        spinbox.valueChanged.connect(lambda value: slider.setValue(int(round(value, 1) * 100)))
        slider.valueChanged.connect(lambda value: spinbox.setValue(round(value, 1) / 100.0))
        spinbox.setValue(default_value)

    def add_float_option(self, name, min_val, max_val, default_val):
        label = QLabel(name)
        self.layout.addWidget(label)
        option_layout = QHBoxLayout()
    
        self.options[name] = QDoubleSpinBox()
        slider = QSlider(Qt.Horizontal)
        
        self.setup_float_input_slider(self.options[name], slider, min_val, max_val, default_val)
        option_layout.addWidget(self.options[name])
        option_layout.addWidget(slider)
        
        self.layout.addLayout(option_layout)

    def send_request(self):
        headers = {'Content-Type': 'application/json'}
        options_values = {}
        for k in self.options:
            options_values[k.lower().replace(" ", "_")] = self.options[k].value()
        json_data = json.dumps(options_values)
        print(json_data)

        response = requests.post(self.main_window.backend_address + self.route, data=json_data, headers=headers)
        print(f'Operation Response status code: {response.status_code}')

        self.accept()


class NoisePopup(OperationDialog):
    def __init__(self, main_window):
        super().__init__(main_window)
        # TODO
        # noise type
        # self.add_float_option("Frequency", 0, 100, 50)
        # self.add_float_option("Intensity", 0, 100, 50)
        self.layout.addWidget(self.buttonBox)
        self.route = "add_noise"


class ResizePopup(OperationDialog):
    def __init__(self, main_window):
        super().__init__(main_window)
        self.add_float_option("Factor", 0, 10, 2)
        self.layout.addWidget(self.buttonBox)
        self.route = "resize"


class WaterLevelPopup(OperationDialog):
    def __init__(self, main_window):
        super().__init__(main_window)
        self.add_float_option("Percentage", 0, 100, 71)
        self.layout.addWidget(self.buttonBox)
        self.route = "adjust_water_percentage"


class ClimatePopup(OperationDialog):
    def __init__(self, main_window):
        super().__init__(main_window)
        self.add_float_option("Pole Temperature", -100, 100, -35)
        self.add_float_option("Equator Temperature", -100, 100, 25)
        self.layout.addWidget(self.buttonBox)
        self.route = "calculate_climate"

