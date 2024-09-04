# Calculation

This document explains how to calculate the geographic position using a sextant. The calculations for latitude (i.e. north-south position) and a longitude (i.e. east-west position) used in the geographic coordinate system (GCS) are explained separately in the sections [latitude](#latitude) and [longitude](#longitude), respectively.

The computations and formulas in this section are based on the original source from 1768:

> Harrison, John. The principles of Mr. Harrison's time-keeper: with plates of the same. W. Richardson and S. Clark and sold by J. Nourse, 1768.
> Retrieved from <https://books.google.fi/books?id=MwZZAAAAcAAJ&pg=IA2>


## Latitude

The general idea for calculating the latitude ,i.e. the _north-south_ position, is the observation that the closer you get to the equator, the higher the sun will be, i.e. the steeper the sun's rays will hit the earth. This is also why it's usually warmer around the equator than at the poles. Therefore, you should be able to make an educated guess whether you are close to the equator or the poles based on the angle of the sun. With a sextant, you can actually measure the angle between the sun compared and the horizon (**elevation**) precisely.

However, you also need to consider **seasons**: During the winter months, the sun's rays hit the earth more flatly than during summer. So to actually make a calculation instead of an educated guess, you would need to know the date of the measurement and compensate for the season somehow. This season-dependent sun angle is called **sun declination** ($d_s$) and, as you might have guessed it, can be calculated using the day of the year ($d_y$) and a trigonometric function (i.e. sin/cos).

Formula:

$$ \text{lat} = 90° - (e_{s,c} - d_s) $$

with
- $e_{s,c}$: elevation, i.e. measured angle between the sun and the horizon at culmination
- $d_s$: as the sun declination

### Sun Declination 


$$ d_s = -23.45° \cdot cos(\frac{360}{365}) \cdot (d_y + 10) $$

with

- $d_s$ being the declination of the sun (i.e. the result), which can be obtained using the [NOAA Solar Calculator](https://gml.noaa.gov/grad/solcalc/), and
- $d_y$ the day of the year (i.e. $1 \geq d_y \geq 365$)



## Longitude

The idea for the longitude, i.e. the east-west position, is that the earth rotates under the sun, so the sun will raise at eastern countries before it raises at western countries, which is why we have time zones. For example, Helsinki/Finland uses UTC+2 while London/UK uses UTC (both in winter, i.e. without daylight saving time). This way, both locations see the sun in its highest point at around 12 o'clock local time. So if you note the time at which the sun was at its highest point, you can calculate the difference between when you experienced the culmination and when a certain reference location experienced the culmination (via a lookup for the day of the year). This difference can the be used to map it to a longitude difference. Preferably, you use the prime meridian as reference, because it has the longitude 0°, so your longitude difference will be the actual longitude.

**Formula**:

$$ \text{lng} = (t_{c,r} - t_{c,p}) \cdot \frac{180°}{12 h} $$

with:
- $t_{c,r}$: time of culmination at the measurement location
- $t_{c,p}$: time of culmination at the prime meridian. This value can be obtained using the [NOAA Solar Calculator](https://gml.noaa.gov/grad/solcalc/). (TODO: There should also be a way to calculate it yourself).

Note that in order to calculate with times, they are encoded as 24-hour UTC _decimal hours_~(see section [decimal hours](#decimal-hours)).


### Decimal hours

A decimal hour is a mapping of hh:mm to a decimal number, so for example 1:30 pm = 13:30 = 13.5 hours.

**Formula**:

$$ t_h + \frac{t_m}{60 \frac{min}{h}} $$

with

- $t_h$: The hour part of the time value
- $t_m$: The minute part of the time value

**Example**: Consider the time 1:04 pm = 13:04, i.e. $t_h = 13$ and $t_m = 4$:

$$
t_h + \frac{t_m}{60 \frac{min}{h}}
= 13 \; h + \frac{4 \; min}{60 \frac{min}{h}}
= \frac{13 \; h \cdot 60 \frac{min}{h}}{60 \frac{min}{h}} + \frac{4 \; min}{60 \frac{min}{h}}
= \frac{784 \; min}{60 \frac{min}{h}} = \frac{196}{15} h = 13.0 \bar{6} h
$$