from PyQt5.QtWidgets import QWidget, QLabel, QVBoxLayout, QHBoxLayout, QSizePolicy
from PyQt5.QtGui import QPixmap
from PyQt5.QtCore import Qt

class InitialTab(QWidget):
    def __init__(self):
        super().__init__()
        self.initUI()

    def initUI(self):
        layout = QHBoxLayout(self)

        # Left side (Text)
        text_layout = QVBoxLayout()

        self.description = QLabel()
        self.description.setText("<h1>Welcome</h1><p>Get started by creating a new world or opening a file.</p>")
        self.description.setWordWrap(True)

        text_layout.addWidget(self.description)
        text_layout.addStretch()

        # Right side (Image)
        self.image_label = QLabel()
        self.image_label.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Expanding)
        self.image_label.setAlignment(Qt.AlignmentFlag.AlignHCenter | Qt.AlignmentFlag.AlignVCenter)
        
        # self.pixmap = QPixmap("img/img.png")
        # self.pixmap = self.pixmap.scaled(500, 500, Qt.AspectRatioMode.KeepAspectRatio, Qt.TransformationMode.SmoothTransformation)
        # self.image_label.setPixmap(self.pixmap)

        layout.addLayout(text_layout)
        layout.addWidget(self.image_label)
        
        self.setLayout(layout)
