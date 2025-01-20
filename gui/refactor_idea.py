


self.current_longitude = 0

def display_image(self):
    if self.curr_projection not in self.images:
        request.post("http://127.0.0.1:8000/image", self.view_config)
    for layer in self.view_config.layers:
        if layer not in self.images[self.curr_projection]:
            req_config = copy(self.view_config)
            req_config["layers"] = [layer]
            request.post("http://127.0.0.1:8000/image", self.view_config)
        self.display_layer(self.images[self.curr_projection][layer][self.curr_longitude])

