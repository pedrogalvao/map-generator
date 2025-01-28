
PROJECTIONS = ["mollweide","equirectangular", "azimuthal", "double azimuthal", "orthographic", "double orthographic", "mercator", "pseudocylindrical"]
# LAYERS = ["Climate", "Countries", "Ice Caps", "Plates", "Demographics", "Annual Temperature"]
LAYERS = ["Satellite", "Climate", "Relief Shadow", "Plates", "Annual Precipitation", "Rivers", "Temperature", "Continentality", "Contour", "Mountains", "Trees", "Rhumb Lines", "Parallels and Meridians", "Paper Texture"]

COLOR_SCHEMES = {
    "None": None,
    "Atlas": {
        "points": [
            (-1000, "#ff5a96ff"),
            (-300, "#ff6eb4ff"),
            (0, "#ff6eb4ff"),
            (1, "#ff32c832"),
            (200, "#ff32c832"),
            (201, "#ff4be64b"),
            (500, "#ff4be64b"),
            (501, "#ff96ff64"),
            (1500, "#ff96ff64"),
            (1501, "#ffc8c864"),
            (3000, "#ffc8c864"),
            (3001, "#ff99805a"),
            (5000, "#ff99805a"),
            (5001, "#ffffffff"),
        ]
    },
    "Atlas2": {
        "points": [
            (-1000, "#ff5a96ff"),
            (-300, "#ff6eb4ff"),
            (0, "#ff6eb4ff"),
            (1, "#ff32c832"),
            (100, "#ff32c832"),
            (101, "#ff32d832"),
            (200, "#ff32d832"),
            (201, "#ff4be64b"),
            (400, "#ff4be64b"),
            (401, "#ff7bef5b"),
            (700, "#ff7bef5b"),
            (701, "#ff96ff64"),
            (1300, "#ff96ff64"),
            (1301, "#ffc8c864"),
            (2000, "#ffc8c864"),
            (2001, "#ffb8a860"),
            (3000, "#ffb8a860"),
            (3001, "#ff99805a"),
            (5000, "#ff99805a"),
            (5001, "#ffffffff"),
        ]
    },
    "Atlas Dark Water": {
        "points": [
            (-1000, "#ff1a449a"),
            (-300, "#ff2a54aa"),
            (0, "#ff4e64bb"),
            (1, "#ff32c832"),
            (100, "#ff32c832"),
            (101, "#ff32d832"),
            (200, "#ff32d832"),
            (201, "#ff4be64b"),
            (400, "#ff4be64b"),
            (401, "#ff7bef5b"),
            (700, "#ff7bef5b"),
            (701, "#ff96ff64"),
            (1300, "#ff96ff64"),
            (1301, "#ffc8c864"),
            (2000, "#ffc8c864"),
            (2001, "#ffb8a860"),
            (3000, "#ffb8a860"),
            (3001, "#ff99805a"),
            (5000, "#ff99805a"),
            (5001, "#ffffffff"),
        ]
    },
    "Bright": {
        "points": [
            (-1000, "#ff5a96ff"),
            (-300, "#ff6eb4ff"),
            (0, "#ff6eb4ff"),
            (1, "#ff32c832"),
            (100, "#ff32c832"),
            (200, "#ff32d832"),
            (400, "#ff4be64b"),
            (700, "#ff7bef5b"),
            (1300, "#ff96ff64"),
            (2000, "#ffc8c864"),
            (3000, "#ffb8a860"),
            (5000, "#ff99805a"),
            (6000, "#ffffffff"),
        ]
    },
    "Colorful": {
        "points": [
            (-5000, "#ff000032"),
            (0, "#ff0000ff"),
            (1, "#ff006400"),
            (1000, "#ff00ff00"),
            (2000, "#ffffff00"),
            (5000, "#ffff0000"),
            (8000, "#ffffffff"),
        ]
    },
    "Green Blue Black": {
        "points": [
            (-5000, "#ff000001"),
            (0, "#ff0000ff"),
            (1, "#ff000100"),
            (6400, "#ff00ff00"),
        ]
    },
    "Grayscale": {
        "points": [
            (-5000, "#ff000000"),
            (6400, "#ffffffff"),
        ]
    }
}