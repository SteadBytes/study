import pytest


def get_max_profit(stock_prices):
    if len(stock_prices) < 2:
        return 0
    min_price = stock_prices[0]
    max_profit = 0
    for price in stock_prices:
        min_price = min(min_price, price)
        profit = price - min_price
        max_profit = max(max_profit, profit)
    return max_profit


def test_given_example():
    stock_prices = [10, 7, 5, 8, 11, 9]
    assert get_max_profit(stock_prices) == 6


def test_all_descending_prices_returns_0():
    stock_prices = [10, 8, 6, 4, 2, 1]
    assert get_max_profit(stock_prices) == 0


@pytest.mark.parametrize("stock_prices", ([], [1]))
def test_stock_prices_less_than_2_elements_returns_0(stock_prices):
    assert get_max_profit(stock_prices) == 0
