# /// script
# requires-python = ">=3.14"
# dependencies = [
#     "matplotlib",
# ]
# ///
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon


def plot_filled_polygon(points_file, output_image="polygon2.png"):
    """
    Reads points from a file in format "x,y" (one per line),
    connects them with 1-pixel black lines, marks exact points as 1-pixel red squares,
    fills the area, no axes - just the PNG.
    """
    # Read points from file
    points = []
    with open(points_file, "r") as f:
        for line in f:
            line = line.strip()
            if line:
                x, y = map(int, line.split(","))
                points.append((x, y))

    if len(points) < 3:
        print("Need at least 3 points to form a polygon")
        return

    # Create figure without axes
    fig, ax = plt.subplots(figsize=(10, 8))
    fig.patch.set_facecolor("white")

    # Filled polygon with 1-pixel black lines
    polygon = Polygon(
        points,
        closed=True,
        fill=True,
        edgecolor="black",
        facecolor="lightblue",
        linewidth=72.0 / fig.dpi,
    )  # Exactly 1 pixel wide
    ax.add_patch(polygon)

    # Exactly 1-pixel red squares at points
    x_coords, y_coords = zip(*points)
    pixel_size = (72.0 / fig.dpi) ** 2  # Exactly 1 pixel in points^2
    ax.scatter(
        x_coords,
        y_coords,
        color="red",
        s=pixel_size,
        marker="s",
        edgecolors=None,
        linewidths=0,
        zorder=5,
    )

    # Auto-limits with padding, equal aspect
    x_min, x_max = min(x_coords), max(x_coords)
    y_min, y_max = min(y_coords), max(y_coords)
    padding = 1.5
    ax.set_xlim(x_min - padding, x_max + padding)
    ax.set_ylim(y_min - padding, y_max + padding)
    ax.set_aspect("equal")

    # Remove all axes, titles, grid, labels
    ax.set_xticks([])
    ax.set_yticks([])
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.spines["bottom"].set_visible(False)
    ax.spines["left"].set_visible(False)
    ax.axis("off")

    # Save clean PNG without any plot elements
    plt.savefig(
        output_image, dpi=300, bbox_inches="tight", pad_inches=0, facecolor="white"
    )
    plt.close()  # Don't show, just save
    print(f"Clean PNG saved as {output_image}")


# Example usage with your points
if __name__ == "__main__":
    plot_filled_polygon("./inputs/2025/day9.txt")
