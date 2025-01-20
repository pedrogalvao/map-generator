import json
from PyQt5.QtWidgets import QDialog, QPushButton, QVBoxLayout, QFrame, QDialogButtonBox, QLabel, QDoubleSpinBox, QFormLayout, QSlider, QHBoxLayout
from PyQt5.QtGui import QPixmap, QCursor
from PyQt5.QtCore import pyqtSignal, Qt

from PyQt5 import QtCore, QtGui, QtWidgets
from PyQt5.QtWidgets import QMessageBox
import requests


class EditMenu(QFrame):
    image_loaded = pyqtSignal(QPixmap)
    def __init__(self, main_window):
        super().__init__()
        self.main_window = main_window
        self.setStyleSheet("background-color: #cccccc;")
        self.menu_layout = QVBoxLayout()
        style = """
            QFrame {
                background-color: #2c3e50;
                border-radius: 10px;
                padding: 10px;
            }
            QPushButton {
                background-color: #44597e;
                color: white;
                border: none;
                border-radius: 5px;
                padding: 10px;
                text-align: left;
                font-size: 12px;
                font-weight: bold;
            }
            QPushButton:hover {
                background-color: #4b69a8;
            }
            QPushButton:pressed {
                background-color: #2c3e50;
            }
            QLabel {
                color: #ffffff;
                font-size: 14px;
            }
            QCheckBox {
                color: #ffffff;
                font-size: 14px;
            }
        """

        self.operation_buttons = {}
        for op in ["Change Sea Level", "Smooth", "Add Noise", "Translation Noise", "Adjust Height", "Erosion", "Resize"]:
            op_button = QPushButton(op, self)
            self.operation_buttons[op] = op_button
            self.menu_layout.addWidget(op_button)
            op_button.setCursor(QCursor(Qt.PointingHandCursor))
            # op_button.clicked.connect(lambda op=op : self.button_clicked(op))

        self.operation_buttons["Add Noise"].clicked.connect(self.add_noise_request)
        self.operation_buttons["Change Sea Level"].clicked.connect(self.water_level_popup)
        self.operation_buttons["Smooth"].clicked.connect(self.smooth_request)
        self.operation_buttons["Erosion"].clicked.connect(self.erosion_request)
        self.operation_buttons["Translation Noise"].clicked.connect(self.translation_noise_request)

        self.setStyleSheet(style)
        self.menu_layout.addStretch()
        self.setLayout(self.menu_layout)
        self.show_popup()

    def translation_noise_request(self):
        headers = {'Content-Type': 'application/json'}
        response = requests.post("http://127.0.0.1:8000/translation_noise", headers=headers)
        print(f'Translation Noise Response status code: {response.status_code}')

    def erosion_request(self):
        headers = {'Content-Type': 'application/json'}
        response = requests.post("http://127.0.0.1:8000/erosion", headers=headers)
        print(f'Erosion Response status code: {response.status_code}')

    def smooth_request(self):
        headers = {'Content-Type': 'application/json'}
        response = requests.post("http://127.0.0.1:8000/smooth", headers=headers)
        print(f'Smooth Response status code: {response.status_code}')

    def add_noise_request(self):
        headers = {'Content-Type': 'application/json'}
        response = requests.post("http://127.0.0.1:8000/add_noise", headers=headers)
        print(f'Add Noise Response status code: {response.status_code}')

    def water_level_popup(self):
        print("WaterLevelPopup")
        dlg = WaterLevelPopup(self.main_window)
        if dlg.exec():
            print("Success!")
        else:
            print("Cancel!")

    def show_popup(self):
        print("POPUP")
        msg = QMessageBox()
        msg.setWindowTitle("Tutorial on PyQt5")
        msg.setText("This is the main text!")
        msg.setIcon(QMessageBox.Question)
        msg.setStandardButtons(QMessageBox.Cancel|QMessageBox.Retry|QMessageBox.Ignore)
        msg.setDefaultButton(QMessageBox.Retry)
        msg.setInformativeText("informative text")

        msg.setDetailedText("details")

        msg.buttonClicked.connect(self.popup_button)

    def popup_button(self, i):
        print(i.text())

    def button_clicked(self, s):
        print("click", s)
        dlg = CustomDialog()
        if dlg.exec():
            print("Success!")
        else:
            print("Cancel!")


class CustomDialog(QDialog):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("HELLO!")

        QBtn = QDialogButtonBox.Ok | QDialogButtonBox.Cancel

        self.buttonBox = QDialogButtonBox(QBtn)
        self.buttonBox.accepted.connect(self.accept)
        self.buttonBox.rejected.connect(self.reject)

        self.layout = QVBoxLayout()
        message = QLabel("Something happened, is that OK?")
        self.layout.addWidget(message)
        self.layout.addWidget(self.buttonBox)
        self.setLayout(self.layout)

class WaterLevelPopup(QDialog):
    def __init__(self, main_window):
        super().__init__()
        self.main_window = main_window
        QBtn = QDialogButtonBox.Ok | QDialogButtonBox.Cancel

        self.buttonBox = QDialogButtonBox(QBtn)
        self.buttonBox.accepted.connect(self.send_request)
        self.buttonBox.rejected.connect(self.reject)
        self.layout = QVBoxLayout(self)
        self.options = {}
        self.add_float_option("Water Percentage", 0, 100, 71)
        self.layout.addWidget(self.buttonBox)

    def setup_float_input_slider(self, spinbox, slider, min_value, max_value, default_value):
        spinbox.setRange(min_value, max_value)
        slider.setRange(min_value * 100, max_value * 100)
        
        # Synchronize the spinbox and slider
        spinbox.valueChanged.connect(lambda value: slider.setValue(int(round(value, 1) * 100)))
        slider.valueChanged.connect(lambda value: spinbox.setValue(round(value, 1) / 100.0))
        spinbox.setValue(default_value)

    def add_float_option(self, name, min_val, max_val, default_val):
        label = QLabel(name)
        label.setStyleSheet("""
                color: #ffffff; font-size: 14px""")
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
        print("percentage", self.options["Water Percentage"].value())
        json_data = json.dumps({"percentage": self.options["Water Percentage"].value()})
        print(json_data)

        response = requests.post("http://127.0.0.1:8000/adjust_water_percentage", data=json_data, headers=headers)
        print(f'Water Level Response status code: {response.status_code}')
