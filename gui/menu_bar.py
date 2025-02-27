from PyQt5.QtWidgets import QAction, QMenuBar, QFileDialog

from edit_requests import ClimatePopup, ResizePopup, WaterLevelPopup, add_noise_request, erosion_request, smooth_request, translation_noise_request
from generation_menu import GenerationMenu
from new_view_menu import NewViewMenu


class TopMenuBar(QMenuBar):
    def __init__(self, main_window):
        super().__init__()
        self.main_window = main_window

        self.file_menu = self.addMenu("File")
        self.view_menu = self.addMenu("View")
        self.edit_menu = self.addMenu("Edit")

        new_world_action = QAction("New World", self.main_window)
        new_world_action.triggered.connect(lambda : GenerationMenu(self.main_window).exec())
        self.file_menu.addAction(new_world_action)
        
        save_action = QAction("Save", self.main_window)
        save_action.triggered.connect(self.main_window.save_map)
        self.file_menu.addAction(save_action)
        load_action = QAction("Open", self.main_window)
        load_action.triggered.connect(self.main_window.load_map)
        self.file_menu.addAction(load_action)
    
        export_action = QAction("Export as...", self.main_window)
        export_action.triggered.connect(self.export_image)
        self.file_menu.addAction(export_action)

        add_erosion_action = QAction("Add Erosion", self.main_window)
        add_erosion_action.triggered.connect(lambda : erosion_request(self.main_window.selected_world()))
        self.edit_menu.addAction(add_erosion_action)

        add_noise_action = QAction("Add Noise", self.main_window)
        add_noise_action.triggered.connect(lambda : add_noise_request(self.main_window.selected_world()))
        self.edit_menu.addAction(add_noise_action)
        
        add_translation_noise_action = QAction("Add Translation Noise", self.main_window)
        add_translation_noise_action.triggered.connect(lambda : translation_noise_request(self.main_window.selected_world()))
        self.edit_menu.addAction(add_translation_noise_action)

        sea_level_action = QAction("Change Sea Level", self.main_window)
        sea_level_action.triggered.connect(lambda : WaterLevelPopup(self.main_window).exec())
        self.edit_menu.addAction(sea_level_action)

        smooth_action = QAction("Smooth", self.main_window)
        smooth_action.triggered.connect(lambda : smooth_request(self.main_window.selected_world()))
        self.edit_menu.addAction(smooth_action)

        resize_action = QAction("Resize", self.main_window)
        resize_action.triggered.connect(lambda : ResizePopup(self.main_window).exec())
        self.edit_menu.addAction(resize_action)

        climate_action = QAction("Define Climate", self.main_window)
        climate_action.triggered.connect(lambda : ClimatePopup(self.main_window).exec())
        self.edit_menu.addAction(climate_action)

        add_view_action = QAction("Add View", self.main_window)
        add_view_action.triggered.connect(lambda : NewViewMenu(self.main_window).exec())
        self.view_menu.addAction(add_view_action)

        # open_view_action = QAction("Open View Directory", self)
        # open_view_action.triggered.connect(lambda : self.main_window.tabs.currentWidget().map_viewer.view_side_menu.open_view_dir())
        # view_menu.addAction(open_view_action)

        self.view_menu.aboutToShow.connect(self.enable_or_disable)
        self.edit_menu.aboutToShow.connect(self.enable_or_disable)

    def enable_or_disable(self):
        if self.main_window.selected_world() != None:
            self.enable()
        else:
            self.disable()
    
    def disable(self):
        for action in self.view_menu.actions():
            action.setDisabled(True)
        for action in self.edit_menu.actions():
            action.setDisabled(True)

    def enable(self):
        for action in self.view_menu.actions():
            action.setDisabled(False)
        for action in self.edit_menu.actions():
            action.setDisabled(False)
        for action in self.file_menu.actions():
            print(action)

    def export_image(self):
        options = QFileDialog.Options()
        options |= QFileDialog.DontUseNativeDialog
        filepath, _ = QFileDialog.getSaveFileName(self, 
            "Save File", "", "PNG (*.png);; JPEG (*.jpg);; GIF (*.gif)", options = options)
        if filepath:
            file_extension = filepath.split(".")[-1]
            print(file_extension)
            if file_extension == "png":
                curr_img_pixmap = self.main_window.tabs.currentWidget().map_viewer.current_image()
                curr_img_pixmap.save(filepath, "PNG")
            elif file_extension == "jpg":
                curr_img_pixmap = self.main_window.tabs.currentWidget().map_viewer.current_image()
                curr_img_pixmap.save(filepath, "JPEG")
            elif file_extension == "gif":
                pass