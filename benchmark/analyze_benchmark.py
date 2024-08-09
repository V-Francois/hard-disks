import numpy as np
import pandas as pd
import glob
import seaborn as sns
import matplotlib.pyplot as plt
import pymser
import yaml


DISK_DIAMETER = a0 = 1
DISK_VOLUME = (DISK_DIAMETER / 2) ** 2 * np.pi


def pressure_from_yaml(file_path: str, plot=False) -> dict:
    packing_fraction = float(file_path.split("_")[2].strip(".yaml"))
    disk_surface_area = np.pi * (0.5) ** 2
    number_density = packing_fraction / disk_surface_area

    with open(file_path) as f:
        results = yaml.safe_load(f)

    df = pd.DataFrame(
        {"g": results["g_of_r"]["normalized_g"], "r": results["g_of_r"]["r"]}
    )

    coefs = np.polyfit(df["r"], df["g"], deg=4)
    xs = np.linspace(min(df["r"]), max(df["r"]))
    ys = (
        coefs[0] * xs**4
        + coefs[1] * xs**3
        + coefs[2] * xs**2
        + coefs[3] * xs
        + coefs[4]
    )

    value_at_0 = np.sum(coefs)

    p_over_kt = number_density * (1 + 2 * packing_fraction * value_at_0)

    if plot:
        sns.scatterplot(data=df, x="r", y="g")
        plt.plot(xs, ys)
        plt.show()

    return {"pressure": p_over_kt, "density": packing_fraction}


def average_volume_from_packing_fraction(packing_fraction: float) -> float:
    return DISK_VOLUME / packing_fraction


def density_for_yaml(file_path: str, plot=False) -> dict:
    pressure = float(file_path.split("_")[2].strip(".yaml"))

    with open(file_path) as f:
        results = yaml.safe_load(f)

    df = pd.DataFrame({"density": results["density"], "step": results["step"]})

    # Run pyMSER to get equilibration data
    results = pymser.equilibrate(
        df["density"],
        LLM=True,
        batch_size=1,
        ADF_test=True,
        uncertainty="uSD",
        print_results=False,
    )

    if plot:
        df["step"] = [int(i) for i in df.index]
        ax = sns.scatterplot(data=df, x="step", y="density")
        ax.plot(
            range(len(df["density"]))[results["t0"] :],
            results["equilibrated"],
            label="Equilibrated data",
            color="red",
        )
        ax.plot(
            [0, len(df["density"])],
            [results["average"], results["average"]],
            color="green",
            zorder=4,
            label="Equilibrated average",
        )
        ax.fill_between(
            range(len(df["density"])),
            results["average"] - results["uncertainty"],
            results["average"] + results["uncertainty"],
            color="lightgreen",
            alpha=0.3,
            zorder=4,
        )
        ax.legend()
        plt.show()

    return {
        "pressure": pressure,
        "density": results["average"],
        "density_err": results["uncertainty"],
    }


if __name__ == "__main__":

    npt_files = glob.glob("./results/results_npt*")
    df_npt = pd.DataFrame.from_records(
        [density_for_yaml(f, plot=False) for f in npt_files]
    )
    df_npt["origin"] = "NPT"

    nvt_files = glob.glob("./results/results_nvt*")
    df_nvt = pd.DataFrame.from_records(
        [pressure_from_yaml(f, plot=False) for f in nvt_files]
    )
    df_nvt["origin"] = "NVT"
    df_nvt["pressure"] = df_nvt["pressure"].apply(lambda x: x)

    ax = sns.scatterplot(
        data=pd.concat([df_npt, df_nvt]),
        y="pressure",
        x="density",
        hue="origin",
    )
    plt.show()
