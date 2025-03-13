import matplotlib.pyplot as plt
import matplotlib as mpl
import matplotlib.ticker as ticker

from PyQt5.QtGui import QPixmap

from constants import COLOR_SCHEMES


def generate_heightmap_legend(height_color_pairs, filename):
    colors = []
    for c in height_color_pairs:
        if len(colors) == 0 or c[0] != colors[-1][0]:
            color = "#" + c[1][-6:] # remove alpha
            if len(colors) == 0 or color != colors[-1][1]:
                colors.append((round(c[0], -1), color))
    bounds = [p[0] for p in colors]
    colors = [p[1] for p in colors]
    
    fig, ax = plt.subplots(figsize=(1, 6), layout='constrained')

    cmap = mpl.colors.ListedColormap(colors)
    # cmap = mpl.colors.LinearSegmentedColormap.from_list("", colors)
    norm = mpl.colors.BoundaryNorm(bounds, cmap.N)

    cbar = fig.colorbar(
        mpl.cm.ScalarMappable(cmap=cmap, norm=norm),
        cax=ax, orientation='vertical',
        spacing='uniform',
    )
    cbar.set_ticks(bounds)
    cbar.ax.yaxis.set_major_formatter(ticker.FuncFormatter(lambda x, _: f'{x}m'))
    fig.savefig(filename, transparent=True)

def generate_heightmap_legend_pixmap(height_color_pairs):
    colors = []
    for c in height_color_pairs:
        if len(colors) == 0 or c[0] != colors[-1][0]:
            color = "#" + c[1][-6:] # remove alpha
            if len(colors) == 0 or color != colors[-1][1]:
                colors.append((round(c[0], -1), color))
    bounds = [p[0] for p in colors]
    colors = [p[1] for p in colors]
    
    fig, ax = plt.subplots(figsize=(1, 6), layout='constrained')

    cmap = mpl.colors.ListedColormap(colors)
    norm = mpl.colors.BoundaryNorm(bounds, cmap.N)

    cbar = fig.colorbar(
        mpl.cm.ScalarMappable(cmap=cmap, norm=norm),
        cax=ax, orientation='vertical',
        spacing='uniform',
    )
    cbar.set_ticks(bounds)
    cbar.ax.yaxis.set_major_formatter(ticker.FuncFormatter(lambda x, _: f'{x}m'))

    fig.savefig("legend.png")
    pixmap = QPixmap("legend.png")
    return pixmap


generate_heightmap_legend(COLOR_SCHEMES["Wikipedia"]["points"],"a.png")
generate_heightmap_legend(COLOR_SCHEMES["Atlas2"]["points"],"b.png")
