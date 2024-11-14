# Benchmarks 

## BASELINE (V1)
objective function 1 person
time:   [213.16 ns 213.70 ns 214.26 ns]

objective function 2 people
time:   [535.07 ns 540.79 ns 547.81 ns]

objective function 4 people
time:   [928.76 ns 935.68 ns 943.17 ns]

objective function 6 people
time:   [1.3090 µs 1.3188 µs 1.3297 µs]

## V2 - BTreeMap -> HashMap

objective function 1 person
time:   [204.63 ns 205.38 ns 206.57 ns]
change: [-4.6456% -4.0301% -3.4489%] (p = 0.00 < 0.05)

objective function 2 people
time:   [469.66 ns 472.44 ns 475.36 ns]
change: [-19.349% -17.620% -15.935%] (p = 0.00 < 0.05)

objective function 4 people
time:   [842.74 ns 856.59 ns 869.65 ns]
change: [-10.489% -9.2086% -8.0499%] (p = 0.00 < 0.05)

objective function 6 people
time:   [1.0786 µs 1.0884 µs 1.0983 µs]
change: [-19.369% -18.140% -16.997%] (p = 0.00 < 0.05)

## V3 HashMap flattened

objective function 1 person
time:   [185.40 ns 185.63 ns 185.86 ns]
change: [-9.9313% -9.4972% -9.1032%] (p = 0.00 < 0.05)

objective function 2 people
time:   [441.82 ns 443.70 ns 445.83 ns]
change: [-6.5465% -5.9125% -5.2900%] (p = 0.00 < 0.05)


objective function 4 people
time:   [821.07 ns 832.62 ns 847.07 ns]
change: [-0.1935% +2.0329% +4.3734%] (p = 0.09 > 0.05)

objective function 6 people
time:   [1.0937 µs 1.1001 µs 1.1069 µs]
change: [+0.3302% +1.4489% +2.5565%] (p = 0.01 < 0.05)

Performance has not improved so back to v2


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


## V5 Inner structs

objective function 1 person
time:   [140.66 ns 141.30 ns 142.03 ns]
change: [-13.953% -13.381% -12.868%] (p = 0.00 < 0.05)
Performance has improved.

objective function 2 people
time:   [246.34 ns 250.52 ns 255.51 ns]
change: [-17.951% -16.040% -14.119%] (p = 0.00 < 0.05)

objective function 4 people
time:   [580.83 ns 604.51 ns 634.44 ns]
change: [-16.655% -13.799% -10.523%] (p = 0.00 < 0.05)


objective function 6 people
time:   [772.28 ns 785.71 ns 799.99 ns]
change: [-11.331% -8.6524% -6.0633%] (p = 0.00 < 0.05)


## V6 No allocations





### Algorithm improvement
# V1:
medium: Avg iterations 204, avg elapsed 1002ms

# V2
medium: 