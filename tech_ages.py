#!/usr/bin/env python
import os
import json
from dateutil import relativedelta
from dateutil.parser import parse
from dateutil.utils import today
from tabulate import tabulate


def get_tech_purchase_dates() -> dict:
    home = os.path.expanduser("~")
    with open(f"{home}/tech-purchase-dates.json", "r") as f:
        tech_purchase_dates = json.loads(f.read())
    return tech_purchase_dates


def print_ownership_times():
    tech_purchase_dates = get_tech_purchase_dates()
    tech_purchase_dates = {tech: parse(purchase_date, dayfirst=True)
                           for tech, purchase_date in tech_purchase_dates.items()}
    tech_ages = {tech: relativedelta.relativedelta(today(), purchase_date)
                 for tech, purchase_date in tech_purchase_dates.items()}
    tech_ages = {tech: f"{age.years} years, {age.months} months, and {age.days} days"
                 for tech, age in tech_ages.items()}
    print(tabulate(tech_ages.items(),
                   headers=["Item", "Age"],
                   tablefmt="psql"))
    return


if __name__ == "__main__":
    print_ownership_times()
