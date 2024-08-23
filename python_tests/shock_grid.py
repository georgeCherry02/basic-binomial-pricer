from pricer import ShockGrid, ShockLimits, generate_shock_grid

def test_shockgrid_generation():
    price = 5
    price_limits = ShockLimits(0.5, 0.5, 100)
    volatiltiy = 10
    volatility_limits = ShockLimits(0.5, 0.5, 100)
    shock_grid = generate_shock_grid(price, price_limits, volatiltiy, volatility_limits)
    assert True
