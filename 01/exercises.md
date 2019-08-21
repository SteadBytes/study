pg 68

# 1.1

Given:

- _card(integer) = c<sub>I</sub>_
- _card(real) = c<sub>R</sub>_
- _card(char) = c<sub>C</sub>_

_type(sex) = (male, female)_

- _card(sex) = 2_

_type(Boolean) = (false, true)_:

- _card(sex) = 2_

_type(weekday) = (Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday)_

- _card(weekday) = 7_

_type(letter) = 'A'..'Z'_

- _card(letter) = 26c<sub>C</sub>_

_type(digit) = '0'..'9'_

- _card(digit) = 10c<sub>C</sub>_

_type(officer) = lieutenant..general_

- _card(officer) = 9_
  - Assuming [UK Army Officer ranks](https://www.army.mod.uk/who-we-are/our-people/ranks/)
  - Lieutenant, Captain, Major, Lieutenant Colonel, Colonel, Brigadier, Major General, Lieutenant General, General

_type(Row) = array[1..5] of real_

- _card(Row) = card(real)<sup>card(int)</sup> = c<sub>R</sub><sup>C<sub>I</sub></sup>_

_type(alfa) = array[1..10] of char_

- _card(alfa) = card(char)<sup>card(int)</sup> = c<sub>C</sub><sup>C<sub>I</sub></sup>_

_type Complex = record re:real; im: real end_

- _card(Complex) = card(real) \* card(real) = c<sub>R</sub><sup>2</sup>_

_type(Date) = record day: 1..31; month: 1..12; year:1..2000 end_

- _card(Date) = card(day) \* card(month) \* card(year) = 31c<sub>I</sub> \* 13c<sub>I</sub> \* 2000c<sub>I</sub> = 806000c<sub>I</sub>_

_type(Person) = record name: alfa; firstname: alfa; birthdate: Date; sex: (male, female); marstatus(single, married widowed, divorced) end_

- _card(Person) = card(alfa) \* card(alfa) \* card(Date) \* card(sex) \* card(marstatus) = c<sub>C</sub><sup>2C<sub>I</sub></sup> \* 806000c<sub>I</sub>\* \* 2 \* 4 = 8 \* c<sub>C</sub><sup>2C<sub>I</sub></sup> \* 806000c<sub>I</sub>_

_type(Coordinate) = record case kind: (Cartesian, polar) of Cartesian: (x, y: real); polar: (r: real; φ: real) end_

- _card(Coordinate) = sum(card(Cartesian), card(polar)) = sum(2c<sub>R</sub>, 2c<sub>R</sub>) = 4c<sub>R</sub>_

_type charset = set of char_

- _card(charset) = 2<sup>C<sub>C</sub></sup>_

_type tapestatus = set of exception_, where _type(exception) = (unloaded, manual, parity ,skew)_

- _card(tapestatus) = 2<sup>card(exception)</sup> = 2<sup>4</sup> = 16_
