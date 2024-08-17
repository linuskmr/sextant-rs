# Calculation

## Latitude

$$ \text{lat} = 90° - (e_{s,c} - d_s) $$


with

- $e_(s,c)$ being the elevation of the sun at culmination, and
- $d_s$ the declination of the sun based on the current day.


### Sun Declination

$$ d_s = -23.45° \cdot cos(\frac{360}{365}) \cdot (d_y + 10) $$

with

- $d_s$ being the declination of the sun (i.e. the result), which can be obtained using the [NOAA Solar Calculator](https://gml.noaa.gov/grad/solcalc/), and
- $d_y$ the day of the year (i.e. $1 eq.gt d_y eq.gt 365$)



## Longitude

$$ \text{lng} = (t_{c,r} - t_{c,p}) \cdot \frac{180°}{12 h} $$

with

- $t_{c,r}$ being the time of culmination at the prime meridian, and
- $t_{c,p}$ the time of culmination at the measurement position

whereby all time values are 24-hour UTC [decimal hours](#decimal-hours).

### Decimal hours

$$ t_h + \frac{t_m}{60 \frac{min}{h}} $$

with

- $t_h$ being the hour part of the time, and
- $t_m$ the minute part of the time.

*Example*: Consider the time $"1:04" "pm" = "13:04"$:

$$
t_h = 13 \; , \; \; t_m = 4 \\
t_h + \frac{t_m}{60 \frac{min}{h}}
= 13 h + \frac{4 min}{60 \frac{min}{h}}
= \frac{13 h \cdot 60 \frac{min}{h}}{60 \frac{min}{h}} + \frac{4 min}{60 \frac{min}{h}}
= \frac{784 min}{60 \frac{min}{h}} = \frac{196}{15} h = 13.0 \bar{6} h
$$
