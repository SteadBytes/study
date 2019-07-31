import pytest


def get_max_profit(stock_prices):
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


def test_all_descending_prices():
    stock_prices = [10, 8, 6, 4, 2, 1]
    assert get_max_profit(stock_prices) == 0
