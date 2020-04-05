# Time and Time Keeping

Length of each day *fluctuates* due to the imperfect rotation of the earth.

- Effectively the length of a second changes slightly over time
- Computers do not tolerate this

**International Atomic Time** (*TAI*) = High-precision atomic coordinate time
standard with *fixed-length* seconds used by the world's atomic clocks.

- "Good" for computers
- Over time, TAI noon will drift towards sunset or sunrise

**Coordinated Universal Time** (*UTC*) = Primary time standard used across the
world to regulate clocks/time. It is *periodically adjusted* to account.

- "Good" for humans
- Sun's position relative to UTC noon stays constant
- Adds leap seconds to TAI every ~18 months

**Absolute time** = Describes the time that one would use in general day to day
life.

- Wall clock time
- Calendar time

**Realtime clock** = Physical clock *embedded* into a computer. Keeps time when
the power is off.

- CMOS clock

**System clock** = Time according to the *operating system*. Used once a system
has booted (taking over from the realtime clock).

- Used by applications
- Can "jump" e.g. manually set to a different position

**Monotonically increasing clock** = Clock that *never* provides the same time
twice.

**Steady clock** = Monotonically increasing clock with seconds guaranteed to be
of **equal length**.

- Do not tell the real time
- Useful for calculating relative durations between points in time

**High accuracy** = Clock with low *skew* from atomic clocks e.g. the length of
it's seconds are *regular* over time.

**High resolution** = Clock able to give accuracy to **10 nanoseconds** or
below.

- Typically found within CPU chips

**Fast clock** = Clock that takes very little realtime to read the time

- Sacrifice accuracy and precision for speed
