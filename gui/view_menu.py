from PyQt5.QtWidgets import QPushButton, QVBoxLayout, QFrame, QFileDialog
from PyQt5.QtCore import Qt
from PyQt5.QtGui import QCursor


class ViewMenu(QFrame):
    def __init__(self, main_window):
        super().__init__()
        self.main_window = main_window
        self.setStyleSheet("background-color: #cccccc;")
        self.menu_layout = QVBoxLayout()
        style = """
            QFrame {
                background-color: #2c3e50;
                border-radius: 10px;
            }
            QPushButton {
                background-color: #44597e;
                color: white;
                border: none;
                font-weight: bold;
                border-radius: 5px;
                padding: 10px;
                text-align: left;
                min-width: 60px;
                max-width: 150px;
            }
            QPushButton:hover {
                background-color: #4b69a8;
            }
            QPushButton:pressed {
                background-color: #2c3e50;
            }
        """
        self.setStyleSheet(style)

        self.options_layout = QVBoxLayout()
        self.menu_layout.addLayout(self.options_layout)

        self.projection_buttons = {}

        self.view_options_layout = QVBoxLayout()
        self.view_options = {}
        self.menu_layout.addLayout(self.view_options_layout)

        self.menu_layout.addStretch()

        self.setLayout(self.menu_layout)
    
    def open_view_dir(self):
        directory = QFileDialog.getExistingDirectory(self, "Select Directory", ".")
        if directory:
            self.main_window.directory = ""
            self.main_window.load_images(directory)
            view_name = directory.split("/")[-1]
            self.view_options[view_name] = QPushButton(view_name, self)
            self.view_options[view_name].clicked.connect(lambda _, view_name=view_name: self.select_view(view_name))
            self.view_options[view_name].setCursor(QCursor(Qt.PointingHandCursor))
            self.options_layout.addWidget(self.view_options[view_name])
    
    def add_view_option(self, view_name):
        if view_name in self.view_options:
            self.options_layout.removeWidget(self.view_options[view_name])
        self.view_options[view_name] = QPushButton(view_name, self)
        self.view_options[view_name].clicked.connect(lambda _, view_name=view_name: self.select_view(view_name))
        self.view_options[view_name].setCursor(QCursor(Qt.PointingHandCursor))
        self.options_layout.addWidget(self.view_options[view_name])
        # self.projections_submenu.addWidget(self.view_options[view_name])

    def add_layer_checkbox(self, name):
        self.layers_buttons[name] = QPushButton(name)
        self.layers_buttons[name].setCheckable(True)
        self.layers_buttons[name].setCursor(QCursor(Qt.PointingHandCursor))
        def changeColor():
            if self.layers_buttons[name].isChecked():
                self.layers_buttons[name].setStyleSheet("background-color: #5b88aa")
            else:
                self.layers_buttons[name].setStyleSheet("background-color: #44597e")
        self.layers_buttons[name].clicked.connect(changeColor)
        self.layers_layout.addWidget(self.layers_buttons[name])
        self.menu_layout.addWidget(self.layers_buttons[name])
        # self.layers_submenu.addWidget(self.layers_buttons[name])

    def select_view(self, view_name:str):
        style_not_selected = """
            QPushButton:hover {
                background-color: #4b69a8;
            }
            QPushButton:pressed {
                background-color: #2c3e50;
            }
        """
        style_selected = """
            QPushButton {
                background-color: #5b88aa;
            }
        """
        for button in self.view_options.values():
            button.setStyleSheet(style_not_selected)
        self.view_options[view_name].setStyleSheet(style_selected)
        self.main_window.select_view(view_name)
