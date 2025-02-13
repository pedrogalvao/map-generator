from PyQt5.QtWidgets import QPushButton, QVBoxLayout, QWidget, QFileDialog, QSizePolicy
from PyQt5.QtCore import Qt
from PyQt5.QtGui import QCursor


class ViewMenu(QWidget):
    def __init__(self, main_window, world_tab):
        super().__init__(world_tab)
        self.main_window = main_window
        self.world_tab = world_tab
        self.menu_layout = QVBoxLayout()
        self.menu_layout.setSizeConstraint(QVBoxLayout.SetMinAndMaxSize)
        style = """
            QPushButton {
                background-color: #6644597e;
                color: white;
                border: none;
                font-weight: bold;
                padding: 5px;
                text-align: left;
                height: 15px;
            }
            QPushButton:hover {
                background-color: #4b69a8bb;
            }
            QPushButton:pressed {
                background-color: #2c3e50;
            }
        """
        self.setWindowFlags(Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint)

        self.setStyleSheet(style)

        self.view_options = {}

        self.minimize_button = QPushButton("-", self)
        self.minimize_button.setFixedSize(20, 20)
        self.minimize_button.clicked.connect(self.toggle_menu)
        self.minimize_button.setCursor(QCursor(Qt.PointingHandCursor))

        self.restore_button = QPushButton("+", self)
        self.restore_button.clicked.connect(self.show_menu)
        self.restore_button.setCursor(QCursor(Qt.PointingHandCursor))
        self.restore_button.hide()
        
        self.menu_layout.addWidget(self.minimize_button)
        self.menu_layout.addWidget(self.restore_button)
        self.menu_layout.setSpacing(0)
        self.menu_layout.setContentsMargins(6, 4, 2, 0)

        self.setLayout(self.menu_layout)

    def toggle_menu(self):
        for view_name in self.view_options:
            self.view_options[view_name].setVisible(False)
        self.restore_button.setFixedSize(20, 20)
        self.restore_button.show()
        self.minimize_button.hide()

    def show_menu(self):
        self.minimize_button.setFixedSize(20, 20)
        self.minimize_button.show()
        for view_name in self.view_options:
            self.view_options[view_name].setVisible(True)
        self.restore_button.hide()
        self.adjustSize()

    def open_view_dir(self):
        directory = QFileDialog.getExistingDirectory(self, "Select Directory", ".")
        if directory:
            self.world_tab.directory = ""
            self.main_window.tabs.currentWidget().load_images(directory)
            view_name = directory.split("/")[-1]
            self.view_options[view_name] = QPushButton(view_name, self)
            self.view_options[view_name].clicked.connect(lambda _, view_name=view_name: self.select_view(view_name))
            self.view_options[view_name].setCursor(QCursor(Qt.PointingHandCursor))
            self.menu_layout.addWidget(self.view_options[view_name])
    
    def add_view_option(self, view_name):
        if view_name in self.view_options:
            self.menu_layout.removeWidget(self.view_options[view_name])
        self.view_options[view_name] = QPushButton(view_name, self)
        self.view_options[view_name].clicked.connect(lambda _, view_name=view_name: self.select_view(view_name))
        self.view_options[view_name].setCursor(QCursor(Qt.PointingHandCursor))
        self.view_options[view_name].setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Preferred)
        self.menu_layout.addWidget(self.view_options[view_name])
        self.adjustSize()
        self.show_menu()

    def add_layer_checkbox(self, name):
        self.layers_buttons[name] = QPushButton(name)
        self.layers_buttons[name].setCheckable(True)
        self.layers_buttons[name].setCursor(QCursor(Qt.PointingHandCursor))
        def changeColor():
            if self.layers_buttons[name].isChecked():
                self.layers_buttons[name].setStyleSheet("background-color: #5b88aaa")
            else:
                self.layers_buttons[name].setStyleSheet("background-color: #44597e")
        self.layers_buttons[name].clicked.connect(changeColor)
        self.layers_layout.addWidget(self.layers_buttons[name])
        self.menu_layout.addWidget(self.layers_buttons[name])

    def select_view(self, view_name:str):
        style_not_selected = """
            QPushButton:hover {
                background-color: #994b69a8;
            }
            QPushButton:pressed {
                background-color: #992c3e50;
            }
        """
        style_selected = """
            QPushButton {
                background-color: #dd5b88aa;
            }
        """
        for button in self.view_options.values():
            button.setStyleSheet(style_not_selected)
        self.view_options[view_name].setStyleSheet(style_selected)
        self.world_tab.select_view(view_name)
