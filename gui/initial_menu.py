from PyQt5.QtWidgets import QWidget, QMainWindow, QVBoxLayout, QPushButton
from PyQt5.QtGui import QIcon, QCursor
from PyQt5.QtCore import Qt

class InitialMenu(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowIcon(QIcon('img/rose.png'))
        self.setWindowTitle("Map Maker")
        self.setGeometry(100, 100, 300, 400)

        central_widget = QWidget(self)
        self.setCentralWidget(central_widget)
        self.menu_layout = QVBoxLayout()
        central_widget.setLayout(self.menu_layout)
        
        self.buttons = {}
        for op in ["Load World", "Load Earth", "Create New World"]:
            op_button = QPushButton(op, self)
            self.buttons[op] = op_button
            self.menu_layout.addWidget(op_button)
            op_button.setCursor(QCursor(Qt.PointingHandCursor))

        style = """
            QPushButton {
                background-color: #44597e;
                color: white;
                border: none;
                border-radius: 5px;
                padding: 10px;
                text-align: center;
                font-size: 22px;
                font-weight: bold;
                height: 100%;
            }
            QPushButton:hover {
                background-color: #4b69a8;
            }
            QPushButton:pressed {
                background-color: #2c3e50;
            }
        """
        self.setStyleSheet(style)
