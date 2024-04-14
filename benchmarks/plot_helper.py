import matplotlib.pyplot as plt
import numpy as np

def safe_name(unsafe: str):
    unsafe = unsafe.replace(" ", "-")
    def repl(c: str):
        if c.isalnum() or c in ('.', '_', '!', '$'):
            return c
        else:
            return "-"
    return "".join(map(repl, list(unsafe)))

def plot(
    x_label: str,
    y_label: str,
    x_vals: np.array,
    y_vals: np.array,
):
    NAME = f"{x_label}/{y_label}"

    plt.figure(num=NAME)
    plt.plot(x_vals, y_vals)
    plt.xlabel(x_label)
    plt.ylabel(y_label)
    plt.title(NAME)
    plt.savefig(f"{safe_name(NAME)}.svg", bbox_inches='tight')
    plt.show()

def plot_3d(
    x_label: str,
    y_label: str,
    z_label: str,
    x_vals: np.array,
    y_vals: np.array,
    z_vals: np.array,
):
    NAME = f"({x_label}, {y_label}) :-> {z_label}"

    fig = plt.figure(num=NAME)
    fig.suptitle(NAME)
    ax = fig.add_subplot(projection='3d')
    ax.scatter(x_vals, y_vals, z_vals)
    ax.set_xlabel(x_label)
    ax.set_ylabel(y_label)
    ax.set_zlabel(z_label)
    plt.savefig(f"{safe_name(NAME)}.svg", bbox_inches='tight')
    plt.show()

