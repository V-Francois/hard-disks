import numpy as np
import pandas as pd
import glob
import seaborn as sns
import matplotlib.pyplot as plt
import os
import time
import pymser


DISK_DIAMETER = a0 = 1
DISK_VOLUME = (DISK_DIAMETER / 2) ** 2 * np.pi


def average_volume_from_packing_fraction(packing_fraction: float) -> float:
    return DISK_VOLUME / packing_fraction


def data_from_result_file(file_path: str, plot=False) -> dict:
    pressure = float(os.path.basename(file_path).split("_")[-1].strip(".csv"))

    df = pd.read_csv(file_path)
    df["volume"] = df["density"].apply(average_volume_from_packing_fraction)

    # Run pyMSER to get equilibration data
    start_time = time.time()
    results = pymser.equilibrate(
        df["volume"],
        LLM=True,
        batch_size=1,
        ADF_test=True,
        uncertainty="uSD",
        print_results=False,
    )

    end_time = time.time() - start_time
    # Print the results
    # print(f"pyMSER Execution time:               {end_time:.3f} s")

    if plot:
        df["step"] = [int(i) for i in df.index]
        ax = sns.scatterplot(data=df, x="step", y="volume")
        ax.plot(
            range(len(df["volume"]))[results["t0"] :],
            results["equilibrated"],
            label="Equilibrated data",
            color="red",
        )
        ax.plot(
            [0, len(df["volume"])],
            [results["average"], results["average"]],
            color="green",
            zorder=4,
            label="Equilibrated average",
        )
        ax.fill_between(
            range(len(df["volume"])),
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
        "volume": results["average"],
        "volume_err": results["uncertainty"],
    }


if __name__ == "__main__":
    # Data from 10.1103/physreve.55.750
    df_ref = pd.DataFrame(
        {
            "pressure": [9, 8.5, 8, 7.5, 7, 6.5, 6],
            "volume": [1.23, 1.245, 1.255, 1.32, 1.35, 1.37, 1.4],
        }
    )
    df_ref["origin"] = "Ref"

    result_files = glob.glob("./results-256/*.csv")
    df_res_256 = pd.DataFrame.from_records(
        [data_from_result_file(f, plot=False) for f in result_files]
    )
    df_res_256["origin"] = "Me - 256"

    result_files = glob.glob("./results/*.csv")
    df_res_100 = pd.DataFrame.from_records(
        [data_from_result_file(f, plot=False) for f in result_files]
    )
    df_res_100["origin"] = "Me - 100"

    # df_res["pressure_corrected"] = df_res["pressure"]

    sns.scatterplot(
        data=pd.concat([df_res_256, df_res_100, df_ref]),
        x="volume",
        y="pressure",
        hue="origin",
    )
    plt.show()
