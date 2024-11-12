# Benchmarks 

## BASELINE (V1)
objective function 1 person
time:   [204.05 ns 204.49 ns 204.98 ns]

objective function 2 people
time:   [429.81 ns 437.50 ns 447.39 ns]

objective function 4 people
time:   [2.1401 µs 2.1673 µs 2.1948 µs]

objective function 6 people
time:   [1.3546 µs 1.3912 µs 1.4289 µs]


## V2 - BTreeMap -> HashMap

objective function 1 person
time:   [188.01 ns 188.42 ns 188.87 ns]
change: [-8.5500% -8.1454% -7.8001%] (p = 0.00 < 0.05)

objective function 2 people
time:   [329.35 ns 332.14 ns 335.14 ns]
change: [-25.583% -24.389% -23.136%] (p = 0.00 < 0.05)

objective function 4 people
time:   [750.28 ns 762.37 ns 774.53 ns]
change: [-65.481% -64.618% -63.689%] (p = 0.00 < 0.05)

objective function 6 people
time:   [1.0561 µs 1.0953 µs 1.1315 µs]
change: [-27.130% -24.484% -21.768%] (p = 0.00 < 0.05)

## V3 HashMap flattened

objective function 1 person
time:   [187.74 ns 188.12 ns 188.57 ns]
change: [-0.1799% +0.1224% +0.4861%] (p = 0.47 > 0.05)

objective function 2 people
time:   [367.09 ns 369.93 ns 373.02 ns]
change: [+7.9259% +9.2831% +10.623%] (p = 0.00 < 0.05)

objective function 4 people
time:   [811.69 ns 817.23 ns 822.74 ns]
change: [+3.1214% +5.4686% +7.7936%] (p = 0.00 < 0.05)

objective function 6 people
time:   [1.1651 µs 1.1880 µs 1.2119 µs]
change: [+9.7665% +12.706% +15.646%] (p = 0.00 < 0.05)

Performance regressed so back to v2


## V4 HashMap -> AHashMap

objective function 1 person
time:   [162.19 ns 162.54 ns 162.96 ns]
change: [-23.733% -23.139% -22.569%] (p = 0.00 < 0.05)

objective function 2 people
time:   [281.72 ns 284.34 ns 287.04 ns]
change: [-30.581% -29.204% -27.938%] (p = 0.00 < 0.05)

objective function 4 people
time:   [581.76 ns 587.35 ns 593.31 ns]
change: [-30.005% -28.786% -27.569%] (p = 0.00 < 0.05)

objective function 6 people
time:   [836.95 ns 846.83 ns 856.61 ns]
change: [-24.033% -22.554% -21.124%] (p = 0.00 < 0.05)
