# Apple Stocks

[Source](https://www.interviewcake.com/question/python/stock-price)

Given a list ,`stock_prices`, of Apple stock prices from yesterday indexed by time (in minutes) past
trade opening time (09:30am local time) write an efficient function that takes `stock_prices`
and returns the best profit that could be made from one purchase and one sale of
one share of Apple stock yesterday.

No "shorting"—you need to buy before you can sell. Also, you can't buy and sell in the same time step—at least 1 minute has to pass.

## Example

```python
>>> stock_prices = [10, 7, 5, 8, 11, 9]
>>> get_max_profit(stock_prices)
6 # buying for $5 and selling for $11
```

## Notes

Buy index = `b`, sell index = `s`

`b` < `s`

- Must buy before sell
- Cannot buy and sell in same time step

Brute force:

- For each price calculate difference between each remaining price in list
- Return highest pair
- _O(n^2)_

Improved:

- Iterate through `stock_prices` once
- Keep track of maximum possible profit so far
  - Same as previous iteration
  - Max profit by selling at the current iteration price
    - Track minimum price so far to calculate
- _O(n)_
