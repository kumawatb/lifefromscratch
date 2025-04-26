# This script converts all .dot molecule files in the current directory to .png images.
# It uses networkx to read the .dot files and matplotlib to save them as .png images.
import os
import networkx as nx
import networkx.drawing.nx_pydot as dot
import matplotlib.pyplot as plt
from pathlib import Path
import argparse

def render_molecule(dot_file_path):
    """
    Renders a molecule from a .dot file and saves it as a .png image.
    
    Args:
        dot_file_path (str): Path to the .dot file.
    """
    # Read the .dot file
    G = dot.read_dot(dot_file_path)

    labeldict = {}
    for node in G.nodes(data=True):
        labeldict[node[0]] = int(node[1]['label'].strip('"'))

    # Create a layout for the nodes
    pos = nx.nx_agraph.graphviz_layout(G, prog='fdp', args='-Elen=1.0 -GK=1.0')

    # Draw the graph
    plt.figure(figsize=(5, 5))
    nx.draw(G, pos, labels=labeldict, with_labels=True, node_size=1500, node_color='lightblue', font_size=10, font_weight='bold', width=2, edgecolors='black')

    # Save the figure
    png_file_path = dot_file_path.with_suffix('.png')
    plt.savefig(png_file_path)
    plt.close()

def main():
    """
    Main function to render all .dot files in the current directory.
    """

    parser = argparse.ArgumentParser(description="Render all .dot files in a directory to .png images.")
    parser.add_argument('-d','--dir', type=str, help='Directory containing .dot files')

    args = parser.parse_args()
    dirpath = Path(args.dir if args.dir else '.')


    # Find all .dot files in the current directory
    dot_files = list(dirpath.glob('*.dot'))

    # Render each .dot file
    for dot_file in dot_files:
        print(f"Rendering {dot_file}...")
        render_molecule(dot_file)
        print(f"Saved as {dot_file.with_suffix('.png')}")
    
    print("All molecules rendered.")

if __name__ == "__main__":
    main()