from datetime import datetime
from dateutil.relativedelta import relativedelta

from pricer import ShockLimits, generate_shock_grid, Call
from .test_utils import get_dt_str, is_close


def example_call():
    strike = 45.0
    cost = 0.0
    return Call(strike, get_dt_str(datetime.now() + relativedelta(months=4)), cost)


def generate_shock_grid_py():
    price = 40.0
    price_limits = ShockLimits(0.3, 0.3, 100)
    volatiltity = 0.4
    volatility_limits = ShockLimits(0.5, 0.5, 100)
    return generate_shock_grid(price, price_limits, volatiltity, volatility_limits)


def test_shockgrid_generation():
    shock_grid = generate_shock_grid_py()
    assert True


def test_shockgrid_prices():
    shock_grid = generate_shock_grid_py()
    prices = shock_grid.prices()
    assert len(prices) == 100
    assert is_close(prices[0], (40 * 0.7), 1)
    assert is_close(prices[99], (40 * 1.3), 1)


def test_shockgrid_volatilities():
    shock_grid = generate_shock_grid_py()
    vols = shock_grid.volatilities()
    assert len(vols) == 100
    assert is_close(vols[0], (0.4 * 0.5), 0.01)
    assert is_close(vols[99], (0.4 * 1.5), 0.01)


def test_shockgrid_pricing():
    call = example_call()
    risk_free_rate = 0.04
    shock_grid = generate_shock_grid_py()
    test = shock_grid.value_black_scholes(call, risk_free_rate)
    assert is_close(test[50][50], 2.0714, 0.0001), "Valued shocked 4 month correctly"
