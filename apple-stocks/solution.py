import pytest


def get_max_profit(stock_prices):
    best = 0
    for i, buy_price in enumerate(stock_prices):
        for sell_price in stock_prices[i + 1 :]:
            best = max(best, sell_price - buy_price)
    return best


def test_given_example():
    stock_prices = [10, 7, 5, 8, 11, 9]
    assert get_max_profit(stock_prices) == 6


def test_all_descending_prices():
    stock_prices = [10, 8, 6, 4, 2, 1]
    assert get_max_profit(stock_prices) == 0
