# Sextant-Calculator

sextant-calculator calculates the geographic position (latitude, longitude) on the Earth’s surface using a sextant measurement.

## Motivation

GPS jamming,

GPS jamming is commonly used [during military exercises](https://www.bbc.com/news/uk-scotland-highlands-islands-43878152) and can be observed in conflict zones, as shown by the GPS jamming maps by [gpsjam.org](https://gpsjam.org) and [flightradar24.com](https://www.flightradar24.com/data/gps-jamming).

Examples include [GPS blocking in Israel](https://www.bbc.com/news/world-middle-east-68734689), [in North Korea](https://www.koreatimes.co.kr/www/nation/2024/08/103_375779.html) and [in the Baltic Sea](https://www.dlr.de/en/kn/latest/news/mysterious-gps-interference-hinders-shipping-and-air-traffic). Especially in the Baltic Sea, GPS disruptions are a [problem for airlines](https://www.bbc.com/news/articles/cne900k4wvjo), causing them to [stop flying to certain destinations](https://company.finnair.com/en/media-centre/all-releases/news?id=2568789F7B492403) as well as for [navigation on (sailing) ships](https://www.yacht.de/en/sailing-knowledge/navigation/navigation-new-gps-faults-in-the-baltic-sea/).



## Solution

Since GPS jamming is threat to safe operation of many widely-used services, a robust backup solution is needed. One approach is [celestial navigation](https://en.wikipedia.org/wiki/Celestial_navigation), i.e. using a **sextant**.
The idea was proposed by John Hadley, Thomas Godfrey and Isaac Newton around the year 1731, so it can be called a battle-tested technology.
With the help of a sextant, the angle of the sun relative to the horizon (or to a artificial horizon, a fluid-filled tube with bubble) is measured at culmination, i.e. at the time of the highest point of the sun, which is around noon. Together with the time of culmination from a sufficiently precise clock (a challenge back in the time), the position (latitude, longitude) can be calculated (see [Calculation](./doc/calculation.md) for details). This library aims to implement these tedious and error-prone calculations.


### Limitations & Other Approaches

The accuracy of a position calculated by a sextant measurement can vary from a few kilometers/miles to a several dozen kilometers/miles, depending on the accuracy of the measurement. Therefore, sextant navigation is not suitable for close navigation (e.g. inside a city), but rather for long-distance navigation (e.g. on the open sea).

[R-Mode Baltic project](https://www.r-mode-baltic.eu/about-r-mode-baltic/) terrestrial positioning system, which "allows positioning even in times when the Global Navigation Satellite Systems (GNSS) fail."