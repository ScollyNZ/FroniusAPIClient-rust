# FroniusAPIClient-rust
Optimising power usage around the home requires the ability to see when excess power is available from the solar array

## Fronuis Gen24
The Fronuis Gen24 Inverter has an unsecured API on the local interface. My inverter's IP address is 192.168.1.85

The API endpoint /status/powerflow returns a JSON payload describing the current power flows on the inverter.

```
{
    "common": {
        "datestamp": "19.10.2024",
        "timestamp": "08:25:49"
    },
    "inverters": [
        {
            "CID": 0,
            "DT": 0,
            "E_Total": 7588551.1283333329,
            "ID": 1,
            "P": 0.0
        }
    ],
    "site": {
        "BatteryStandby": false,
        "E_Day": null,
        "E_Total": 7588551.1283333329,
        "E_Year": null,
        "MLoc": 0,
        "Mode": "meter",
        "P_Akku": null,
        "P_Grid": 1484.3199999999999,
        "P_Load": -1484.3199999999999,
        "P_PV": 0.0,
        "rel_Autonomy": 0.0,
        "rel_SelfConsumption": null
    },
    "version": "12"
}
```
The key fields are `P_Grid`, `P_Load` and `P_PV`. These refer to current wattage being draw from the grid, supplied to the monitored phase and drawn from the photovotalic array, respectively.

The above sample was taken at night so the load, 1.48kw, is being drawn entirely from the grid.

```
{
    "common": {
        "datestamp": "20.10.2024",
        "timestamp": "02: 49: 19"
    },
    "inverters": [
        {
            "CID": 0,
            "DT": 0,
            "E_Total": 7613610.0483333329,
            "ID": 1,
            "P": 2924.706298828125
        }
    ],
    "site": {
        "BatteryStandby": false,
        "E_Day": null,
        "E_Total": 7613610.0483333329,
        "E_Year": null,
        "MLoc": 0,
        "Mode": "meter",
        "P_Akku": null,
        "P_Grid": -1232.78,
        "P_Load": -1675.31765625,
        "P_PV": 3017.677978515625,
        "rel_Autonomy": 100.0,
        "rel_SelfConsumption": 57.608713815007398
    },
    "version": "12"
}
```

This sample was taken during the day. During the day, any PV power generated, that is not consumed by the household load, is sold back to the grid. Negative numbers on the `Grid` indicate exporting power, negative numbrs on the `Load` indicate consuming power.

I have two power loads that could consume excess power ad-hoc. The first is all storm water on my property is collected in a 10Kl tank and pumped out to the council managed storm water system. The second is my EV. 

The intent of this implementation is to consume power for household activites rather than sell to the grid.

The pump implementation is fairly simple. The pump draws 1.5kw, so, if the tank has water in it and > 1.5kw is being sold to the grid, turn on the pump. The power loads will be checked every 10 minutes, not continously, to avoid turning the pump on and off on cloudy days or because of other, uncontrolled loads drawing power. The tank will have a float switch installed that will be monitored every second to ensure the pump doesn't run dry.

The car currently has a charge timer that restricts charging to between 2100 and 0700. The power retailer I use charges 12.95c / kwh for night power, 29.95c / kwh for day power and pays 12c / kwh for generated powered so charging the car at night is mch cheaper. The price difference between charing at night and charing directly off the solar is negligable, so this project won't enable me to retire, but I might learning some Rust in the process.

The car implementation will supply power to the charger between the hours of 2100 and 0700 and during the day if > 1.8kw is being sold to the grid.

## The complexities
The above describes the naive implementation but this would cause the pump to be supplied power before the car. There are a few solutions to this.

1. Include an addition float switch in the tank, near the top, so that the pump only operates when the tank is almost full. This would reduce the likelihood of the pump overriding the car but would also increase the chances of the tank overflowing due to lack of sunny days. The high level float switch would be used as an override, consuming grid power when activated and running for 30 mins to reduce the level in the tank. The pump moves ~67l / minute, emptying the tank in ~2.5 hours. Because the pump consumes less power than the car, it will activate earlier, reduce the power available to the car.
2. The car may not be plugged in, this could be detected by switching the charger on and monitoring power consumption. If power consumption does not increase, the car is not plugged in.

## An advanced solution
The extend the functionality and independance of these smart-grid units, a broadcast protocol could be considered. Each unit would have a priority and a consumption level.

The pump, if the tank has water in it, could be considered priority 3. If the tank is almost full, priority 1. The car would be priority 2. This priority could be changed depending on schedule or weather forecast. 

When a unit detects that PV power is sufficient to power it and has been for a period (5 mins?) it broadcast "I am priority 2 and I would like 1.5kw"