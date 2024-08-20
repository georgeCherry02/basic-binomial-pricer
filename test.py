from datetime import datetime
from dateutil.relativedelta import relativedelta
from pricer import Call, price_black_scholes
import pytz

def get_dt_str(dt: datetime) -> str:
    return pytz.utc.localize(dt).isoformat()

def __main__():
    one_year_more = datetime.now() + relativedelta(days=90)
    expiry = get_dt_str(one_year_more)
    strike = 110.0
    volatility = 0.2

    underlying_price = 100.0
    apr = 0.05

    call = Call(strike, volatility, expiry)
    value = price_black_scholes(call, underlying_price, apr)
    print(f"Priced call at {value}")

if __name__ == "__main__":
    __main__()
