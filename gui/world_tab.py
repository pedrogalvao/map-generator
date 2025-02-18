from copy import deepcopy
import json
from threading import Thread
from time import sleep
from PyQt5.QtWidgets import QFrame, QVBoxLayout, QWidget
from PyQt5.QtGui import QPixmap
from PyQt5.QtCore import QBuffer, QIODevice
import requests
from sortedcontainers import SortedDict

from viewer import MapViewer


class ViewImages:

    def __init__(self):
        self.data = SortedDict()  # Automatically keeps keys sorted
    
    def add(self, rotation, pixmap):
        self.data[rotation] = pixmap
    
    def get(self, number):
        keys = self.data.keys()
        if len(keys) == 0:
            return QPixmap()
        idx = self.data.bisect_left(number)

        # Handle edge cases
        if idx == 0:
            return self.data[keys[0]]
        if idx == len(keys):
            return self.data[keys[-1]]

        # Find closest key
        before = keys[idx - 1]
        after = keys[idx]

        if abs(before - number) <= abs(after - number):
            return self.data[before]
        else:
            return self.data[after]


class WorldTab(QFrame):
    def __init__(self, name, main_window):
        super().__init__()

        self.name = name
        self.map_viewer = MapViewer(main_window)

        main_layout = QVBoxLayout()
        main_layout.addWidget(self.map_viewer)

        container = QWidget()
        container.setLayout(main_layout)
    
        self.setLayout(main_layout)
        self.map_viewer.display_image()

    def select_view(self, view_name:str):
        self.map_viewer.select_view(view_name)

    def request_view(self, view_name: str, req_data):
        print('request_view')
        self.map_viewer.images[view_name] = ViewImages()
        def request_image(req_data, req_number):
            print(f'sending', req_number)
            headers = {'Content-Type': 'application/json'}
            json_data = json.dumps(req_data)
            response = requests.get("http://127.0.0.1:8000/get_image", data=json_data, headers=headers)
            print(f'View Response status code: {response.status_code}')

            image_data = response.content
            pixmap = QPixmap()
            buffer = QBuffer()
            buffer.setData(image_data)
            buffer.open(QIODevice.ReadOnly)
            pixmap.loadFromData(buffer.data())

            print("before", len(self.map_viewer.images[view_name].data))
            self.map_viewer.images[view_name].add(req_number, pixmap)
            print("after", len(self.map_viewer.images[view_name].data))

        print('request_view - after def')
        n_sent_requests = 0
        for i in [0, 15, 25, 5, 20, 10]:
            rotation = (i * 360 / 30) % 360
            if rotation > 180:
                rotation -= 360
            print(f'loop', rotation)
            req_data["params"]["center"][1] = rotation
            t = Thread(target=request_image, args=[deepcopy(req_data), rotation])
            t.start()
            n_sent_requests += 1
            while len(self.map_viewer.images[view_name].data) < n_sent_requests - 2:
                print(len(self.map_viewer.images[view_name].data), i)
                sleep(1)
        for i in range(30):
            if i % 5 == 0:
                continue
            if i % 2 == 1:
                continue
            rotation = (i * 360 / 30) % 360
            if rotation > 180:
                rotation -= 360
            req_data["params"]["center"][1] = rotation
            t = Thread(target=request_image, args=[deepcopy(req_data), rotation])
            t.start()
            n_sent_requests += 1
            while len(self.map_viewer.images[view_name].data) < n_sent_requests - 3:
                sleep(1)
        for i in range(30):
            if i % 5 == 0:
                continue
            if i % 2 == 0:
                continue
            rotation = (i * 360 / 30) % 360
            if rotation > 180:
                rotation -= 360
            req_data["params"]["center"][1] = rotation
            t = Thread(target=request_image, args=[deepcopy(req_data), rotation])
            t.start()
            n_sent_requests += 1
            while len(self.map_viewer.images[view_name].data) < n_sent_requests - 3:
                sleep(1)
        for i in range(30):
            rotation = (i * 360 / 30 + 360 / 60) % 360
            if rotation > 180:
                rotation -= 360
            req_data["params"]["center"][1] = rotation
            t = Thread(target=request_image, args=[deepcopy(req_data), rotation])
            t.start()
            n_sent_requests += 1
            while len(self.map_viewer.images[view_name].data) < n_sent_requests - 3:
                sleep(1)

    def load_images(self, view_name: str):
        self.map_viewer.load_images(view_name)

    def display_image(self, view_name: str):
        self.map_viewer.display_image(view_name)
