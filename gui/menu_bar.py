from PyQt5.QtWidgets import QAction, QMenuBar 

from edit_requests import ClimatePopup, ResizePopup, WaterLevelPopup, add_noise_request, erosion_request, smooth_request, translation_noise_request
from generation_menu import GenerationMenu
from new_view_menu import NewViewMenu


class TopMenuBar(QMenuBar):
    def __init__(self, main_window):
        super().__init__()
        self.main_window = main_window

        file_menu = self.addMenu("File")
        view_menu = self.addMenu("View")
        edit_menu = self.addMenu("Edit")

        new_world_action = QAction("New World", self.main_window)
        new_world_action.triggered.connect(lambda : GenerationMenu(self.main_window).exec())
        file_menu.addAction(new_world_action)
        
        save_action = QAction("Save", self.main_window)
        save_action.triggered.connect(self.main_window.save_map)
        file_menu.addAction(save_action)
        load_action = QAction("Open", self.main_window)
        load_action.triggered.connect(self.main_window.load_map)
        file_menu.addAction(load_action)
        file_menu.addAction("Export as...")


        add_erosion_action = QAction("Add Erosion", self.main_window)
        add_erosion_action.triggered.connect(lambda : erosion_request(self.main_window.selected_world()))
        edit_menu.addAction(add_erosion_action)

        add_noise_action = QAction("Add Noise", self.main_window)
        add_noise_action.triggered.connect(lambda : add_noise_request(self.main_window.selected_world()))
        edit_menu.addAction(add_noise_action)
        
        add_translation_noise_action = QAction("Add Translation Noise", self.main_window)
        add_translation_noise_action.triggered.connect(lambda : translation_noise_request(self.main_window.selected_world()))
        edit_menu.addAction(add_translation_noise_action)

        sea_level_action = QAction("Change Sea Level", self.main_window)
        sea_level_action.triggered.connect(lambda : WaterLevelPopup(self.main_window).exec())
        edit_menu.addAction(sea_level_action)

        smooth_action = QAction("Smooth", self.main_window)
        smooth_action.triggered.connect(lambda : smooth_request(self.main_window.selected_world()))
        edit_menu.addAction(smooth_action)

        resize_action = QAction("Resize", self.main_window)
        resize_action.triggered.connect(lambda : ResizePopup(self.main_window).exec())
        edit_menu.addAction(resize_action)

        climate_action = QAction("Define Climate", self.main_window)
        climate_action.triggered.connect(lambda : ClimatePopup(self.main_window).exec())
        edit_menu.addAction(climate_action)

        add_view_action = QAction("Add View", self.main_window)
        add_view_action.triggered.connect(lambda : NewViewMenu(self.main_window).exec())
        view_menu.addAction(add_view_action)

        open_view_action = QAction("Open View Directory", self)
        open_view_action.triggered.connect(lambda : self.main_window.tabs.currentWidget().view_side_menu.open_view_dir())
        view_menu.addAction(open_view_action)
