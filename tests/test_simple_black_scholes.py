from datetime import datetime
from dateutil.relativedelta import relativedelta

from pricer import Call, price_black_scholes
from .test_utils import get_dt_str, is_close


def test_washington_uni():
    symbol = "AAPL"
    four_months_more = datetime.now() + relativedelta(days=121)
    expiry = get_dt_str(four_months_more)
    strike = 45.0
    cost = 0.0

    volatility = 0.4
    underlying_price = 40.0
    apr = 0.04
    dividend_rate = 0.

    call = Call(symbol, strike, expiry, cost)
    value = price_black_scholes(call, volatility, underlying_price, apr, dividend_rate)
    assert is_close(
        value, 2.0557, 0.0001
    ), f"Valued 4 month call correctly, value={value}, exp=2.055"


def __main__():
    test_washington_uni()
