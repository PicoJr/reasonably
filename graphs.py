from enum import StrEnum
from dataclasses import dataclass
from typing import Tuple

import matplotlib.pyplot as plt
import numpy as np
import numpy.typing as npt
from matplotlib.widgets import Slider


class ProducerColor(StrEnum):
    InternsCost = "xkcd:light blue"
    JuniorDevsCost = "xkcd:light orange"
    SeniorDevsCost = "xkcd:light green"

    InternsProduction = "xkcd:blue"
    JuniorDevsProduction = "xkcd:orange"
    SeniorDevsProduction = "xkcd:green"


@dataclass
class GameConstants:
    interns_initial_cost: float = 20.0
    junior_devs_initial_cost: float = 1250.0
    senior_devs_initial_cost: float = 15_000.0

    interns_cost_growth: float = 1.015
    junior_devs_cost_growth: float = 1.02
    senior_devs_cost_growth: float = 1.0175

    interns_production: float = 1.0
    junior_devs_production: float = 40.0
    senior_devs_production: float = 1000.0


def producer_cost(
    initial_cost: float, growth: float, producers: npt.NDArray[np.float64]
) -> npt.NDArray[np.float64]:
    return initial_cost * growth**producers


def sum_producer_cost(
    initial_cost: float, growth: float, producers: npt.NDArray[np.float64]
) -> npt.NDArray[np.float64]:
    return initial_cost * (1.0 - growth**producers) / (1 - growth)


def slider_cost(
    fig,
    ax,
    x: npt.NDArray[np.float64],
    initial_cost: float,
    growth_cost: float,
    growth_slider_name: str,
    initial_slider_name: str,
    growth_valmin: float,
    growth_valmax: float,
    growth_valinit: float,
    initial_valmin: float,
    initial_valmax: float,
    initial_valinit: float,
    color: ProducerColor,
    label: str,
    slider_id: int,
) -> Tuple[Slider, Slider]:
    [data] = ax.plot(
        x,
        producer_cost(
            initial_cost=initial_cost,
            growth=growth_cost,
            producers=x,
        ),
        marker="o",
        color=color,
        label=label,
    )
    [sum_data] = ax.plot(
        x,
        sum_producer_cost(
            initial_cost=initial_cost,
            growth=growth_cost,
            producers=x,
        ),
        marker="+",
        color=color,
        label=f"sum({label})",
    )

    axgrowth = fig.add_axes((0.25, 0.05 * slider_id, 0.65, 0.03))
    growth_slider = Slider(
        ax=axgrowth,
        label=growth_slider_name,
        valmin=growth_valmin,
        valmax=growth_valmax,
        valinit=growth_valinit,
    )
    axinitial = fig.add_axes((0.25, 0.05 * (slider_id + 1), 0.65, 0.03))
    initial_slider = Slider(
        ax=axinitial,
        label=initial_slider_name,
        valmin=initial_valmin,
        valmax=initial_valmax,
        valinit=initial_valinit,
    )

    def sliders_on_change(_val):
        data.set_ydata(
            producer_cost(
                initial_cost=initial_slider.val, growth=growth_slider.val, producers=x
            )
        )
        sum_data.set_ydata(
            sum_producer_cost(
                initial_cost=initial_slider.val, growth=growth_slider.val, producers=x
            )
        )
        fig.canvas.draw_idle()

    growth_slider.on_changed(sliders_on_change)
    initial_slider.on_changed(sliders_on_change)
    return growth_slider, initial_slider


def slider_production(
    fig,
    ax,
    x: npt.NDArray[np.float64],
    multipliers: npt.NDArray[np.float64],
    production: float,
    slider_name: str,
    valmin: float,
    valmax: float,
    valinit: float,
    color: ProducerColor,
    label: str,
    slider_id: int = 0,
) -> Slider:

    [data] = ax.plot(
        x,
        x * multipliers * production,
        marker="o",
        color=color,
        label=label,
    )

    def update(updated_production):
        data.set_ydata(x * multipliers * updated_production)
        fig.canvas.draw_idle()

    axproduction = fig.add_axes((0.25, 0.05 * slider_id, 0.65, 0.03))
    growth_slider = Slider(
        ax=axproduction,
        label=slider_name,
        valmin=valmin,
        valmax=valmax,
        valinit=valinit,
    )
    growth_slider.on_changed(update)
    return growth_slider


def main():
    game_constants = GameConstants()
    points = 100
    counts = np.logspace(1, 2.8, points, base=10)
    interns_production_multipliers = np.interp(
        x=counts,
        xp=np.array([0.0, 99.0, 100.0, 199.0, 200.0, 399.0, 400.0]),
        fp=np.array([1.0, 1.0, 2.0, 2.0, 16.0, 16.0, 256.0]),
    )
    junior_devs_production_multipliers = np.interp(
        x=counts,
        xp=np.array([0.0, 99.0, 100.0, 199.0, 200.0, 399.0, 400.0]),
        fp=np.array([1.0, 1.0, 8.0, 8.0, 32.0, 32.0, 512.0]),
    )
    senior_devs_production_multipliers = np.interp(
        x=counts,
        xp=np.array([0.0, 99.0, 100.0, 199.0, 200.0, 399.0, 400.0]),
        fp=np.array([1.0, 1.0, 2.0, 2.0, 8.0, 8.0, 256.0]),
    )

    fig, ax = plt.subplots()
    ax.set_yscale("log")
    ax.vlines(
        x=np.array([100.0]),
        ymin=0,
        ymax=1e3,
    )
    fig.subplots_adjust(left=0.25, bottom=0.50)

    # ! important to keep ref to slider, otherwise not responsive !
    # cf https://github.com/matplotlib/matplotlib/pull/3132
    interns_cost_slider = slider_cost(
        fig,
        ax,
        x=counts,
        initial_cost=game_constants.interns_initial_cost,
        growth_cost=game_constants.interns_cost_growth,
        growth_slider_name="interns cost growth",
        initial_slider_name="interns initial cost",
        growth_valmin=1.0,
        growth_valmax=1.2,
        growth_valinit=game_constants.interns_cost_growth,
        initial_valmin=game_constants.interns_initial_cost / 10.0,
        initial_valmax=game_constants.interns_initial_cost * 10.0,
        initial_valinit=game_constants.interns_initial_cost,
        color=ProducerColor.InternsCost,
        label="interns cost",
        slider_id=1,
    )
    interns_production_slider = slider_production(
        fig,
        ax,
        x=counts,
        multipliers=interns_production_multipliers,
        production=game_constants.interns_production,
        slider_name="interns production",
        valmin=game_constants.interns_production / 10.0,
        valmax=game_constants.interns_production * 10.0,
        valinit=game_constants.interns_production,
        color=ProducerColor.InternsProduction,
        label="interns production",
        slider_id=3,
    )

    junior_devs_cost_slider = slider_cost(
        fig,
        ax,
        x=counts,
        initial_cost=game_constants.junior_devs_initial_cost,
        growth_cost=game_constants.junior_devs_cost_growth,
        growth_slider_name="junior devs cost growth",
        initial_slider_name="junior devs initial cost",
        growth_valmin=1.0,
        growth_valmax=1.2,
        growth_valinit=game_constants.junior_devs_cost_growth,
        initial_valmin=game_constants.junior_devs_initial_cost / 10.0,
        initial_valmax=game_constants.junior_devs_initial_cost * 10.0,
        initial_valinit=game_constants.junior_devs_initial_cost,
        color=ProducerColor.JuniorDevsCost,
        label="junior devs cost",
        slider_id=4,
    )
    junior_devs_production_slider = slider_production(
        fig,
        ax,
        x=counts,
        multipliers=junior_devs_production_multipliers,
        production=game_constants.junior_devs_production,
        slider_name="junior_devs production",
        valmin=game_constants.junior_devs_production / 10.0,
        valmax=game_constants.junior_devs_production * 10.0,
        valinit=game_constants.junior_devs_production,
        color=ProducerColor.JuniorDevsProduction,
        label="junior devs production",
        slider_id=6,
    )

    senior_devs_cost_slider = slider_cost(
        fig,
        ax,
        x=counts,
        initial_cost=game_constants.senior_devs_initial_cost,
        growth_cost=game_constants.senior_devs_cost_growth,
        growth_slider_name="senior devs cost growth",
        initial_slider_name="senior devs initial cost",
        growth_valmin=1.0,
        growth_valmax=1.2,
        growth_valinit=game_constants.senior_devs_cost_growth,
        initial_valmin=game_constants.senior_devs_initial_cost / 10.0,
        initial_valmax=game_constants.senior_devs_initial_cost * 10.0,
        initial_valinit=game_constants.senior_devs_initial_cost,
        color=ProducerColor.SeniorDevsCost,
        label="senior devs cost",
        slider_id=7,
    )
    senior_devs_production_slider = slider_production(
        fig,
        ax,
        x=counts,
        multipliers=senior_devs_production_multipliers,
        production=game_constants.senior_devs_production,
        slider_name="senior_devs production",
        valmin=game_constants.senior_devs_production / 10.0,
        valmax=game_constants.senior_devs_production * 10.0,
        valinit=game_constants.senior_devs_production,
        color=ProducerColor.SeniorDevsProduction,
        label="senior devs production",
        slider_id=9,
    )

    ax.legend(loc="upper left")
    plt.show()


if __name__ == "__main__":
    main()
