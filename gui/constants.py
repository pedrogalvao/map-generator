
import glob
import json


PROJECTIONS = ["mollweide","equirectangular", "azimuthal", "double azimuthal", "orthographic", "double orthographic", "mercator", "pseudocylindrical"]
# LAYERS = ["Climate", "Countries", "Ice Caps", "Plates", "Demographics", "Annual Temperature"]
LAYERS = ["Satellite", "Climate", "Relief Shadow", "Plates", "Annual Precipitation", "Rivers", "Temperature", "Continentality", "Contour", "Mountains", "Trees", "Rhumb Lines", "Parallels and Meridians", "Paper Texture"]

COLOR_SCHEMES = {"None":{"points": [[0, "#00000000"]]}}
for filepath in glob.glob("colors/*.json"):
    with open(filepath, "r") as f:
        # print(f.read())
        COLOR_SCHEMES[filepath.split("/")[-1].split("\\")[-1].split(".")[0]] = json.loads(f.read())