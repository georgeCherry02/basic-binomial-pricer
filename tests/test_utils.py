from datetime import datetime
import pytz


def get_dt_str(dt: datetime) -> str:
    return pytz.utc.localize(dt).isoformat()


def is_close(value: float, expected: float, precision: float) -> bool:
    return abs(expected - value) < precision
