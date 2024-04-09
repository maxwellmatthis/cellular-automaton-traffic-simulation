import matplotlib.pyplot as plt
import numpy as np

def plot(
    x_label: str,
    y_label: str,
    x_vals: np.array,
    y_vals: np.array,
    rounds
):
    NAME = f"{x_label}/{y_label} ({rounds} rounds each)"

    plt.figure(num=NAME)
    plt.plot(x_vals, y_vals)
    plt.xlabel(x_label)
    plt.ylabel(y_label)
    plt.title(NAME)
    plt.show()

