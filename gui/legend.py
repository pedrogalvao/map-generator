import matplotlib.pyplot as plt
import numpy as np
import matplotlib.colors as mcolors
import matplotlib.patches as mpatches

def generate_heightmap_legend(height_color_pairs):
    heights, colors = zip(*height_color_pairs)
    
    # Create a figure and axis
    fig, ax = plt.subplots(figsize=(6, 1))
    fig.subplots_adjust(bottom=0.5)
    
    # Create a colormap and normalizer
    cmap = mcolors.LinearSegmentedColormap.from_list("custom_cmap", colors)
    norm = mcolors.Normalize(vmin=min(heights), vmax=max(heights))
    
    # Create colorbar
    cbar = plt.colorbar(plt.cm.ScalarMappable(norm=norm, cmap=cmap), cax=ax, orientation='horizontal')
    cbar.set_label("Height")
    
    # Show the legend
    plt.show()

# Example usage
height_color_pairs = [
    (0, "#0000FF"),   # Blue at sea level
    (0.1, "#00FF00"),  # Green for lowlands
    (50, "#00FF00"),  # Green for lowlands
    (51, "#FFFF00"),  # Yellow for midlands
    (100, "#FFFF00"), # Yellow for midlands
    (101, "#FFA500"), # Orange for highlands
    (200, "#FFA500"), # Orange for highlands
    (201, "#FF0000"), # Red for peaks
    (300, "#FF0000")  # Red for peaks
]

def generate_legend(pairs):
    fig, ax = plt.subplots(figsize=(6, 2))
    
    # Create legend patches
    patches = [mpatches.Patch(color=color, label=f"{name}") for name, color in pairs]
    
    # Add legend to plot
    ax.legend(handles=patches, loc='center', ncol=len(pairs))
    ax.axis('off')
    
    # Show the legend
    plt.show()

# Example usage
height_color_pairs = [
    (0, "#0000FF"),
    (50, "#00FF00"),
    (100, "#FFFF00"),
    (200, "#FFA500"),
    (300, "#FF0000")
]

generate_heightmap_legend(height_color_pairs)

generate_legend([
	("A", "#0000FF"),
    ("B", "#00FF00"),
    ("C", "#FFFF00"),
    ("D", "#FFA500"),
    ("E", "#FF0000")
])
