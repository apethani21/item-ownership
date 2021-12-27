using Dates
using JSON
using DataStructures
using PrettyTables

function relativedelta(start_date, end_date)
    if end_date < start_date
        throw(DomainError((start_date, end_date), "Require start_date <= end_date"))
    end

    months = (12 * Dates.year(end_date) + Dates.month(end_date)) - (12 * Dates.year(start_date) + Dates.month(start_date))

    if abs(months) > 11
        years, months = divrem(months, 12)
    else
        years = 0
    end

    updated_start_date = start_date + Dates.Year(years) + Dates.Month(months)

    if updated_start_date > end_date
        updated_start_date = updated_start_date - Dates.Month(1)
        months -= 1
    end

    delta_year = Dates.year(end_date) - Dates.year(updated_start_date)
    delta_month = Dates.month(end_date) - Dates.month(updated_start_date)

    if (delta_year == 0) & (delta_month == 0)
        days = Dates.day(end_date) - Dates.day(updated_start_date)
    elseif (delta_year == 0) & (delta_month == 1)
        days = Dates.day(end_date) + (Dates.day(lastdayofmonth(updated_start_date)) - Dates.day(updated_start_date))
    elseif delta_year == 1
        @assert delta_month == -11
        days = Dates.day(end_date) + (31 - Dates.day(updated_start_date))
    else
        throw(DomainError((delta_year, delta_month), "unexpected value for (delta_year, delta_month)"))
    end

    return (years, months, days)
end

function print_ownership_times()
    item_dates = JSON.parsefile(string(homedir(), "/", "purchase-dates.json"))
    format = DateFormat("d/m/y")

    item_dates = sort([
            (item, relativedelta(Date(purchase_date, format), Dates.today()))
            for (item, purchase_date) in item_dates
        ], by = x -> x[2], rev = true)

    item_dates = OrderedDict(
        (item => "$years years, $months months, and $days days"
         for (item, (years, months, days)) in item_dates)
    )

    pretty_table(reshape([item_dates.keys; item_dates.vals], (:, 2)); header = ["Item", "Age"])
end

print_ownership_times()