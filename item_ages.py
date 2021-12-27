#!/Users/ajaypethani/anaconda3/bin/python
import os
import json
from dateutil import relativedelta
from dateutil.parser import parse
from dateutil.utils import today
from tabulate import tabulate


def get_purchase_dates() -> dict:
    home = os.path.expanduser("~")
    with open(f"{home}/purchase-dates.json", "r") as f:
        tech_purchase_dates = json.loads(f.read())
    return tech_purchase_dates


def print_ownership_times():
    purchase_dates = get_purchase_dates()
    purchase_dates = {item: parse(purchase_date, dayfirst=True)
                           for item, purchase_date in purchase_dates.items()}
    ages = {item: relativedelta.relativedelta(today(), purchase_date)
            for item, purchase_date in purchase_dates.items()}
    ages = {item: f"{age.years} years, {age.months} months, and {age.days} days"
            for item, age in ages.items()}
    print(tabulate(ages.items(),
                   headers=["Item", "Age"],
                   tablefmt="psql"))
    return


if __name__ == "__main__":
    print_ownership_times()
