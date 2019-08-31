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

# 1.2

See [02.py](./q2.py) for full code examples.

## Computer Store

### _sex_, _Boolean_ and _weekday_

Simple enumerated types, each component represents an integer -> store as a single integer -> 32 bit block of memory.

### _letter_, _digit_ and _officer_

Subrange types can be represented in store by storing its minimum and maximum values - one section of memory for each.

- _*letter*_ = _char_ values -> 16 bit subrange -> two bytes, one for 'A' and one for 'Z'
- _digit_ = _integer_ values -> 32 bit subrange -> 4 bytes, 2 for `0` and 2 for `9`
- \_officer\_\_ = enumeration values, each level of officer can be represented as an integer enumeration -> 32 bit subrange -> 4 bytes -> 2 for integer representing `lieutenant`, 2 for integer representing `general`

### _row_, _alfa_

Arrays represented as a _contiguous_ block of memory in computer store:

- _row_ = 5 components of type _char_ -> 5 contiguous _bytes_ of memory
  - A _char_ is stored as 8 bits (1 byte)
- _alfa_ = 10 components of type _real_ -> 80 contiguous _bytes_ of memory
  - Assuming _real_ is stored using [IEEE 754 double precision floating point format](https://en.wikipedia.org/wiki/Double-precision_floating-point_format), each _real_ is 64 bits (8 bytes)

### _complex_, _Date_, _Person_, _Coordinate_

- _complex_ = 2 components of type _real_ -> 16 _bytes_ of contiguous memory (8 bytes per component)
- _Date_ = 3 _integer subrange_ components -> 12 _bytes_ of contiguous memory (4 bytes per component)
- _Person_ = 2 _alfa_, 1 _Date_, 2 enums = 180 _bytes_ of contiguous memory
  ```
  +----------------------+
  |name: alfa            | -> 80 bytes
  |----------------------|
  |firstname: alfa       | -> 80 bytes
  |----------------------|
  |birthdate: Date       | -> 12 bytes
  |----------------------|
  |sex: (male, female)   | -> 4 bytes
  |----------------------|
  |marstatus: (          |
  |  single, married,    | -> 4 bytes
  |  widowed, divorced)  |
  +----------------------+
  ```
- _Coordinate_ = 20 _bytes_ of contiguous memory -> 4 bytes for the `kind` field (enum `(Cartesian, polar)`), 8 _bytes_ for each coordinate of type _real_.
  - Variant part represented by a record with 1st element of tag field type (if given an identifier), followed by elements of the **biggest** variant [source](https://www.freepascal.org/docs-html/ref/refsu15.html)
  ```
  +------------------------+
  |kind: (Cartesian, Polar)|
  |------------------------|
  |real                    | -> x or r
  |------------------------|
  |real                    | -> y or phi
  +------------------------+
  ```

### _charset_, _tapestatus_

Sets represented as **bit vectors** -> one element representing each element of the set.

- _charset_ = 32 _byte_ array -> 1 _bit_ per element of set -> _char_ has range _0-255_ -> 256 _bits_ total
- _tapestatus_ = 4 bit array -> _exception_ has 4 possible values -> 1 bit each

## Fortran

Note: First time using Fortran therefore the solutions presented below are likely to be sub-optimal.

### _sex_, _Boolean_ and _weekday_

```fortran
enum, bind(C)
    enumerator sex :: 0
    enumerator :: MALE
    enumerator :: FEMALE
end enum

! Boolean as an enum
enum, bind(C)
    enumerator Boolean :: 0
    enumerator :: FALSE
    enumerator:: TRUE
end enum

! or use primitive boolean
LOGICAL :: a_bool = .TRUE.
LOGICAL :: another_bool = .FALSE.

! weekday
enum, bind(C)
    enumerator Weekday :: 0
    enumerator :: MONDAY
    enumerator :: TUESDAY
    enumerator :: WEDNESDAY
    enumerator :: THURSDAY
    enumerator :: FRIDAY
    enumerator :: SATURDAY
    enumerator :: SUNDAY
end enum
```

### _letter_, _digit_ and _officer_

Fortran does not have a subrange data type, as such the values are constrained within a user provided _constructor_ for each type.

```fortran
!types.f90
module m
  type letter
    character(len=1) :: value
  end type
  interface letter
    module procedure new_letter
  end interface

  type digit
      integer :: value
  end type
  interface digit
      module procedure new_digit
  end interface

  enum, bind(c)
    enumerator :: officer_ranks = 0
    enumerator:: lieutenant
    enumerator:: captain
    enumerator:: major
    enumerator:: lieutenant_colonel
    enumerator:: colonel
    enumerator:: brigadier
    enumerator:: major_general
    enumerator:: lieutenant_general
    enumerator:: general
  end enum

  type officer
        integer(kind(officer_ranks)) :: value
  end type
  interface officer
      module procedure new_officer
  end interface

contains

  function new_letter(ch)
          character, intent(in) :: ch
          type(letter) new_letter
    if (('A' .LE. ch) .AND. (ch .LE. 'Z')) then
        new_letter%value = ch
        return
    else
      print *, "letter value must be in range 'A'..'Z'"
      call EXIT(1)
    endif
  end function

  function new_digit(d)
          integer, intent(in) :: d
          type(digit) new_digit
    if ((0 .LE. d) .AND. (d .LE. 9)) then
        new_digit%value = d
        return
    else
      print *, "digit value must be in range 0..9"
      call EXIT(1)
    endif
  end function

  function new_officer(rank)
          integer(kind(officer_ranks)), intent(in) :: rank
          type(officer) new_officer
    if ((lieutenant .LE. rank) .AND. (rank .LE. general)) then
        new_officer%value = rank
        return
    else
      print *, "officer value must be in range lieutenant..general"
      call EXIT(1)
    endif
  end function

end module

program main
    use m
    type(letter) :: l
    type(digit) :: d
    type(officer) :: o

    l = letter('E')
    print *, l%value

    d = digit(5)
    print *, d%value

    o = officer(lieutenant)
    print *, o%value

    ! these would each cause the program to exit with error message
    ! l = letter('@')
    ! print *, l%value

    ! d = digit(12)
    ! print *, d%value

    ! o = officer(25) ! out of officer_ranks enum range
    ! print *, o%value

end program main
```

### _row_, _alfa_

```fortran
real, dimension(5) :: row
char, dimension(10) :: alfa
```

### _complex_, _Date_, _Person_, _Coordinate_

```fortran
module m
type Complex
  real :: re
  real :: im
end type

type Date
  integer :: day
  integer :: month
  integer :: year
end type
interface Date
  module procedure new_Date
end interface

enum, bind(c)
  enumerator marriage_status :: 0
  enumerator single
  enumerator married
  enumerator widowed
  enumerator divorced
end enum

type Person
  real, dimension(10) :: name
  real, dimension(10) :: firstname
  Date :: birthdate
  integer(kind(sex)) :: sex
  integer(kind(marriage_status)) :: marstatus
end type

! Coordinate (fortran doesn't have variant records)
type Cartesian
  real x
  real y
end type

type polar
  real r
  real phi
end type

contains
  function new_date(day, month, year)
    integer, intent(in) :: day
    integer, intent(in) :: month
    integer, intent(in) :: year
    type(Date) new_date
    if (.NOT. ((1 .LE. day) .AND. (day .LE. 31))) then
        print *, "day value must be in range 1..31"
        call EXIT(1)
    else if (.NOT. ((1 .LE. month) .AND. (month .LE. 12)))
        print *, "month value must be in range 1..12"
        call EXIT(1)
    else if (.NOT. ((1 .LE. year) .AND. (year .LE. 2000)))
        print *, "year value must be in range 1..2000"
        call EXIT(1)
    else
      new_date%day = year
      new_date%month = month
      new_date%year = year
    endif
  end function

end module
```

## Favourite Programming Language

I'm going to swap between languages a bit to better represent the type specifications.

- i.e. a fixed length array of a specific type such as _alfa_ would just be a general _list_ in Python.

### _sex_, _Boolean_ and _weekday_

```python
from enum import Enum

class Sex(Enum):
    MALE = 1
    FEMALE = 2

class Boolean(Enum):
    FALSE = 0
    TRUE = 1

# or just use actual bool types
a_bool = True
another_bool = False

class Weekday(Enum):
    MONDAY = 0
    TUESDAY = 1
    WEDNESDAY = 2
    THURSDAY = 3
    FRIDAY = 4
    SATURDAY = 5
    SUNDAY = 6
```

### _letter_, _digit_ and _officer_

```python
from enum import Enum

def subrange(base, low, high):
    if low > high:
        raise ValueError(f"low must be <= high")

    class SubRange(base):
        def __new__(cls, value):
            if low <= value and value <= high:
                x = base.__new__(cls, value)
                return x
            else:
                raise ValueError(f"value {value} not in range {low}..{high}")

    return SubRange


Letter = subrange(str, "A", "Z")

Digit = subrange(int, 0, 9)


class Officer(Enum):
    LIEUTENANT = 0
    CAPTAIN = 1
    MAJOR = 2
    LIEUTENANT_COLONEL = 3
    COLONEL = 4
    BRIGADIER = 5
    MAJOR_GENERAL = 6
    LIEUTENANT_GENERAL = 7
    GENERAL = 8
```

### _row_, _alfa_

```c
float Row[5];
char alfa[10];
```

### _complex_, _Date_, _Person_, _Coordinate_

```python
class Complex(NamedTuple):
    re: float
    im: float


Day = subrange(int, 1, 31)
Month = subrange(int, 1, 12)
Year = subrange(int, 1, 2000)


class Date:
    def __init__(self, day: Day, month: Month, year: Year):
        self.day = Day(day)
        self.month = Month(month)
        self.year = Year(year)


class MarriageStatus(Enum):
    SINGLE = 0
    MARRIED = 1
    WIDOWED = 2
    DIVORCED = 3


class Person(NamedTuple):
    name: str
    firstname: str
    birthdate: Date
    sex: Sex
    marstatus: MarriageStatus
```

```c
enum CoordinateKind {
  Cartesian,
  polar
};

struct Coordinate {
  enum CoordinateKind kind;
  union {
    /* Cartesian */
    struct {
      float x;
      float y;
    };
    /* polar */
    struct {
      float r
      float phi;
    }
  }
}
```

### _charset_ and _tapestatus_

```python
# Python has no built in char type -> use length 1 strings
charset: Set[str] = set()


class TapeException(Enum):  # renamed to not collide with python Exception class
    unloaded = 0
    manual = 1
    parity = 2
    skew = 3


tapestatus: Set[TapeException] = set()
```

# 1.4

Correct use of variant records can be checked at run time if the compiler generates appropriate code for runtime tests to verify that the type discriminator is consistent with any fields accessed on the record. For example:

```pascal
// variable c of type Coordinate
// insert runtime test here to ensure c.kind == Cartesian
c.x = 10.5
```

The above check is almost certainly superseded by compile time verification. Since the valid fields for each value of the type discriminator are specified the in type declaration, a compiler would be able to check for valid access.

# 1.5

Sequential files have _infinite_ cardinality - allowing for completely _dynamic_ data within a program. When using arrays, the data to be stored is known at compile time, whereas the data read from sequential files is known only at runtime. Sequential files also allow data to be _persisted_, allowing the results of a program to be stored after completion of program execution. This also allows the same data to be used in _multiple programs_ from the same sequential file.

