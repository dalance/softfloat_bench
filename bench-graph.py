#!/usr/bin/env python3
import re
import numpy as np
import matplotlib.pyplot as plt
import argparse
from matplotlib.ticker import ScalarFormatter
import os

# Library colors
COLORS = {
    'rug': '#1f77b4',
    'rustc_apfloat': '#ff7f0e',
    'simple_soft_float': '#2ca02c',
    'softfloat_sys': '#d62728',
    'softfloat_pure': '#9467bd',
    'const_soft_float': '#8c564b'
}

def parse_benchmark_data(raw_data):
    """Parse the benchmark output format into structured data."""
    lines = raw_data.strip().split('\n')
    benchmarks = []

    pattern = r'test ([^:]+)::([^_]+)_([^ ]+)\s+\.\.\. bench:\s+([\d,\.]+) ns/iter'

    for line in lines:
        match = re.match(pattern, line)
        if match:
            precision, operation, library, time_str = match.groups()
            time = float(time_str.replace(',', ''))
            benchmarks.append({
                'precision': precision,
                'operation': operation,
                'library': library,
                'time': time
            })

    return benchmarks

def create_visualization(benchmarks, output_dir='benchmark_graphs'):
    """Create visualizations for the benchmark data."""
    # Ensure output directory exists
    os.makedirs(output_dir, exist_ok=True)

    # Extract unique values
    precisions = sorted(list(set(b['precision'] for b in benchmarks)))
    operations = sorted(list(set(b['operation'] for b in benchmarks)))
    libraries = sorted(list(set(b['library'] for b in benchmarks)))

    # Set the style
    plt.style.use('ggplot')

    # Create plots for each operation
    for operation in operations:
        # Filter data for this operation
        op_data = [b for b in benchmarks if b['operation'] == operation]

        # Prepare data for plotting
        plot_data = {}
        for precision in precisions:
            plot_data[precision] = {}
            for library in libraries:
                matching = [b for b in op_data if b['precision'] == precision and b['library'] == library]
                if matching:
                    plot_data[precision][library] = matching[0]['time']
                else:
                    plot_data[precision][library] = 0

        # Create the figure
        plt.figure(figsize=(12, 8))

        # Set up the bar positions
        bar_width = 0.15
        index = np.arange(len(precisions))

        # Plot bars for each library
        for i, library in enumerate(libraries):
            values = [plot_data[p][library] for p in precisions]
            plt.bar(
                index + i * bar_width,
                values,
                bar_width,
                label=f"{library.replace('_', ' ')}",
                color=COLORS.get(library, f'C{i}')
            )

        # Set up the axes
        plt.xlabel('Precision', fontsize=14)
        plt.ylabel('Time (ns/iter)', fontsize=14)
        plt.title(f'{operation.capitalize()} Operation Performance by Library and Precision', fontsize=16)
        plt.xticks(index + bar_width * (len(libraries) - 1) / 2, precisions)
        plt.legend()

        # Set log scale for y-axis with proper formatting
        plt.yscale('log')
        plt.grid(True, which="both", ls="-", alpha=0.2)
        plt.gca().yaxis.set_major_formatter(ScalarFormatter())

        # Add value labels on top of each bar
        for i, library in enumerate(libraries):
            for j, precision in enumerate(precisions):
                value = plot_data[precision][library]
                if value > 0:  # Only label non-zero bars
                    # Format large numbers with commas
                    if value < 1000:
                        label_text = str(value)
                    else:
                        label_text = f"{value:,}"

                    plt.text(
                        j + i * bar_width,
                        value * 1.1,  # Position slightly above the bar
                        label_text,
                        ha='center',
                        va='bottom',
                        rotation=90,
                        fontsize=8
                    )

        # Adjust layout and save
        plt.tight_layout()
        plt.savefig(f"{output_dir}/{operation}_benchmark.png", dpi=300)
        plt.close()

    # Create a combined visualization with subplots
    fig, axes = plt.subplots(nrows=len(operations), figsize=(14, 5 * len(operations)))

    for i, operation in enumerate(operations):
        ax = axes[i] if len(operations) > 1 else axes

        # Filter data for this operation
        op_data = [b for b in benchmarks if b['operation'] == operation]

        # Prepare data for plotting
        plot_data = {}
        for precision in precisions:
            plot_data[precision] = {}
            for library in libraries:
                matching = [b for b in op_data if b['precision'] == precision and b['library'] == library]
                if matching:
                    plot_data[precision][library] = matching[0]['time']
                else:
                    plot_data[precision][library] = 0

        # Set up the bar positions
        bar_width = 0.15
        index = np.arange(len(precisions))

        # Plot bars for each library
        for j, library in enumerate(libraries):
            values = [plot_data[p][library] for p in precisions]
            ax.bar(
                index + j * bar_width,
                values,
                bar_width,
                label=f"{library.replace('_', ' ')}",
                color=COLORS.get(library, f'C{j}')
            )

        # Set up the axes
        ax.set_xlabel('Precision', fontsize=12)
        ax.set_ylabel('Time (ns/iter)', fontsize=12)
        ax.set_title(f'{operation.capitalize()} Operation Performance', fontsize=14)
        ax.set_xticks(index + bar_width * (len(libraries) - 1) / 2)
        ax.set_xticklabels(precisions)

        # Set log scale for y-axis with proper formatting
        ax.set_yscale('log')
        ax.grid(True, which="both", ls="-", alpha=0.2)
        ax.yaxis.set_major_formatter(ScalarFormatter())

        # Only add legend to the first subplot to avoid redundancy
        if i == 0 and len(operations) > 1:
            ax.legend(loc='upper left', bbox_to_anchor=(1, 1))

    # Add overall legend for the combined plot
    if len(operations) == 1:
        axes.legend(loc='upper left', bbox_to_anchor=(1, 1))

    # Adjust layout and save
    plt.tight_layout()
    plt.savefig(f"{output_dir}/combined_benchmark.png", dpi=300)
    plt.close()

    # Create a "zoomed-in" version excluding the slowest library
    create_zoomed_visualization(benchmarks, output_dir)

def create_zoomed_visualization(benchmarks, output_dir):
    """Create visualizations excluding the slowest library for better comparison."""
    # Extract unique values
    precisions = sorted(list(set(b['precision'] for b in benchmarks)))
    operations = sorted(list(set(b['operation'] for b in benchmarks)))
    # Exclude the simple_soft_float library which is much slower
    libraries = [lib for lib in sorted(list(set(b['library'] for b in benchmarks))) if lib != 'simple_soft_float']

    # Create a combined visualization with subplots
    fig, axes = plt.subplots(nrows=len(operations), figsize=(14, 5 * len(operations)))

    for i, operation in enumerate(operations):
        ax = axes[i] if len(operations) > 1 else axes

        # Filter data for this operation
        op_data = [b for b in benchmarks if b['operation'] == operation and b['library'] in libraries]

        # Prepare data for plotting
        plot_data = {}
        for precision in precisions:
            plot_data[precision] = {}
            for library in libraries:
                matching = [b for b in op_data if b['precision'] == precision and b['library'] == library]
                if matching:
                    plot_data[precision][library] = matching[0]['time']
                else:
                    plot_data[precision][library] = 0

        # Set up the bar positions
        bar_width = 0.2  # Wider bars with fewer libraries
        index = np.arange(len(precisions))

        # Plot bars for each library
        for j, library in enumerate(libraries):
            values = [plot_data[p][library] for p in precisions]
            ax.bar(
                index + j * bar_width,
                values,
                bar_width,
                label=f"{library.replace('_', ' ')}",
                color=COLORS.get(library, f'C{j}')
            )

            # Add value labels
            for k, value in enumerate(values):
                if value > 0:
                    ax.text(
                        k + j * bar_width,
                        value * 1.1,
                        str(value),
                        ha='center',
                        va='bottom',
                        rotation=90,
                        fontsize=9
                    )

        # Set up the axes
        ax.set_xlabel('Precision', fontsize=12)
        ax.set_ylabel('Time (ns/iter)', fontsize=12)
        ax.set_title(f'{operation.capitalize()} Operation Performance (Excluding simple_soft_float)', fontsize=14)
        ax.set_xticks(index + bar_width * (len(libraries) - 1) / 2)
        ax.set_xticklabels(precisions)

        # Use linear scale for better comparison without the extreme outliers
        ax.grid(True, which="both", ls="-", alpha=0.2)

        # Only add legend to the first subplot to avoid redundancy
        if i == 0 and len(operations) > 1:
            ax.legend(loc='upper left', bbox_to_anchor=(1, 1))

    # Add overall legend for the combined plot
    if len(operations) == 1:
        axes.legend(loc='upper left', bbox_to_anchor=(1, 1))

    # Adjust layout and save
    plt.tight_layout()
    plt.savefig(f"{output_dir}/zoomed_benchmark.png", dpi=300)
    plt.close()

def main():
    parser = argparse.ArgumentParser(description='Generate visualizations from Rust benchmark data')
    parser.add_argument('--input', type=str, help='Input file with benchmark data')
    parser.add_argument('--output', type=str, default='benchmark_graphs', help='Output directory for graphs')
    args = parser.parse_args()

    # Read from file or stdin
    if args.input:
        with open(args.input, 'r') as f:
            raw_data = f.read()
    else:
        print("Please paste your benchmark data (press Ctrl+D when done):")
        raw_data = ''
        try:
            while True:
                line = input()
                raw_data += line + '\n'
        except EOFError:
            pass

    # Parse and visualize
    benchmarks = parse_benchmark_data(raw_data)
    if benchmarks:
        create_visualization(benchmarks, args.output)
        print(f"Visualizations saved to {args.output}/ directory")
    else:
        print("No valid benchmark data found")

if __name__ == '__main__':
    main()
