from dash import Dash, html, dcc, callback, Output, Input
import plotly.graph_objects as go
import plotly.express as px

from datetime import date, datetime, time
from dateutil.relativedelta import relativedelta
import pytz

from pricer import Call, ShockLimits, generate_shock_grid

app = Dash()

app.layout = [
    html.H1(children='Title of Dash App', style={'textAlign':'center'}),
    dcc.Input(id='strike', type="number", value=50.),
    dcc.Input(id='cost', type="number", value=5.),
    dcc.DatePickerSingle(id="expiry", min_date_allowed=date.today(), date=date.today() + relativedelta(months=3)),
    dcc.Graph(id='graph-content')
]

def get_dt(date: str) -> str:
    non_local = datetime.strptime(date, "%Y-%m-%d")
    local = pytz.utc.localize(non_local)
    return local.isoformat()

def get_predefined_shock_grid():
    price = 40.0
    price_limits = ShockLimits(0.3, 0.3, 100)
    volatiltity = 0.4
    volatility_limits = ShockLimits(0.5, 0.5, 100)
    return generate_shock_grid(price, price_limits, volatiltity, volatility_limits)

@callback(
    Output('graph-content', 'figure'),
    Input('strike', 'value'),
    Input('cost', 'value'),
    Input('expiry', 'date')
)
def update_off_strike(strike, cost, expiry):
    call = Call(strike, get_dt(expiry), cost)
    shock_grid = get_predefined_shock_grid()
    risk_free_rate = 0.04
    valuations = shock_grid.value_black_scholes(call, risk_free_rate)
    fig = go.Figure(data=[go.Surface(x=shock_grid.prices(), y=shock_grid.volatilities(), z=valuations, cmid=0., colorscale=["red", "white", "green"])])
    fig.update_layout(title="Option Plot", width=700, height=700)
    fig.update_scenes(xaxis_title_text='Price', yaxis_title_text='Volatility', zaxis_title_text='BSM Valuation')
    return fig

if __name__ == '__main__':
    app.run(debug=True)