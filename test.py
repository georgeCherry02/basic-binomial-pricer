from datetime import datetime
from pricer import Call
import pytz

def get_now_str() -> str:
    return pytz.utc.localize(datetime.now()).isoformat()

def __main__():
    call = Call(100.0, 5.0, get_now_str())
    print(call)

if __name__ == "__main__":
    __main__()
