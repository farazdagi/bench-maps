# Benchmarks of maps

- The test suite was adapted from https://github.com/xacrimon/conc-map-bench.
- Tests were done using [bustle](https://crates.io/crates/bustle) test harness.
- When testing three hashing algorithms were used:
    - standard hasher
    - [FxHash](https://crates.io/crates/fxhash) hasher from rustc compiler. This is **not** cryptographically secure
      hasher.
    - [AHash](https://crates.io/crates/ahash) fast DOS resistant hasher. Output is of high quality, but **not**
      cryptographically secure.
- Three different workloads were tested:
    - Read-heavy: (read: 98%; insert: 1%; remove: 1%)
    - Write-heavy: (read: 5%; insert: 80%; remove: 5%; update: 10%)
    - Mixed: (read: 10%; insert: 40%; remove: 40%; update: 10%)
- In each workload, total number of operations was capped at ~34M.
- All hashers were used in both single-threaded and multi-threaded mode (with 2^2..2^5 threads).
- Machine was Core i9, 8 kernels, 16 cpu.
- Crates tested:
    - [DashMap](https://crates.io/crates/dashmap)
    - [HashBrown](https://crates.io/crates/hashbrown)
    - [Flurry](https://crates.io/crates/flurry)
    - [CHashMap](https://crates.io/crates/chashmap)
    - `std::collections::HashMap`
    - `std::collections::BTreeMap`
- sled's [concurrent-map](https://github.com/komora-io/concurrent-map) was ruled out because:
    - It doesn't try to implement Rust's `HashMap` API (no `with_capacity()`, no `with_hasher()`).
    - It doesn't support hasher injection at all (and we might need custom hashing).
    - While it is claimed that it is a lock-free B+ tree, it is `Send`, but
      not `Sync` ([see details](https://github.com/komora-io/concurrent-map/blob/main/src/lib.rs#L300)), meaning that it
      is not possible to use in multithreaded environment (without cloning). The problem
      is [erb](https://github.com/komora-io/concurrent-map/blob/main/src/lib.rs#L343) which relies on RefCell
      internally.
    - Overall, as the whole sled project, this thing seems to be still of a beta-version quality.
- To run: `cargo run bench --workload ReadHeavy|WriteHeavy|Mixed` (tests below were run on optimized/release binary).

## Read-heavy workload

### DashMap

[DashMap](https://crates.io/crates/dashmap) is a blazingly fast concurrent map in Rust.
DashMap tries to implement an easy-to-use API similar to `std::collections::HashMap` with some slight changes to handle
concurrency.

```
-- DashMap - StdHasher
threads=1	total_ops=33,554,432	spent=5.0s      latency=147.00ns    throughput=6,760,596 op/s
threads=2	total_ops=33,554,432	spent=2.6s      latency=154.00ns    throughput=12,965,943 op/s
threads=4	total_ops=33,554,432	spent=1.4s      latency=166.00ns    throughput=24,012,293 op/s
threads=8	total_ops=33,554,432	spent=830.3ms   latency=197.00ns    throughput=40,410,898 op/s
threads=16	total_ops=33,554,432	spent=537.9ms	latency=256.00ns    throughput=62,378,414 op/s
threads=32	total_ops=33,554,432	spent=563.8ms	latency=537.00ns    throughput=59,510,906 op/s

-- DashMap - FxHasher
threads=1	total_ops=33,554,432	spent=2.6s      latency=78.00ns     throughput=12,666,648 op/s
threads=2	total_ops=33,554,432	spent=1.5s      latency=88.00ns     throughput=22,662,270 op/s
threads=4	total_ops=33,554,432	spent=882.3ms	latency=105.00ns    throughput=38,028,548 op/s
threads=8	total_ops=33,554,432	spent=586.4ms	latency=139.00ns    throughput=57,216,450 op/s
threads=16	total_ops=33,554,432	spent=449.9ms	latency=214.00ns    throughput=74,588,061 op/s
threads=32	total_ops=33,554,432	spent=444.0ms	latency=423.00ns    throughput=75,576,047 op/s

-- DashMap - AhashHasher
threads=1	total_ops=33,554,432	spent=2.8s      latency=83.00ns     throughput=11,941,357 op/s
threads=2	total_ops=33,554,432	spent=1.6s      latency=92.00ns     throughput=21,521,166 op/s
threads=4	total_ops=33,554,432	spent=882.5ms	latency=105.00ns    throughput=38,020,443 op/s
threads=8	total_ops=33,554,432	spent=564.1ms	latency=134.00ns    throughput=59,480,602 op/s
threads=16	total_ops=33,554,432	spent=478.0ms	latency=227.00ns    throughput=70,192,475 op/s
threads=32	total_ops=33,554,432	spent=457.0ms	latency=435.00ns    throughput=73,426,841 op/s

```

### HashBrown

[HashBrown](https://crates.io/crates/hashbrown) is a Rust port of Google's high-performance SwissTable hash map, adapted
to make it a drop-in replacement for Rust's standard `HashMap` and `HashSet` types.

```
-- HashbrownHashMap - StdHasher
threads=1	total_ops=33,554,432	spent=4.1s	latency=123.00ns  throughput=8,097,989 op/s
threads=2	total_ops=33,554,432	spent=3.0s	latency=180.00ns  throughput=11,050,000 op/s
threads=4	total_ops=33,554,432	spent=2.8s	latency=337.00ns  throughput=11,836,585 op/s
threads=8	total_ops=33,554,432	spent=3.3s	latency=786.00ns  throughput=10,168,501 op/s
threads=16	total_ops=33,554,432	spent=3.9s	latency=1.85µs    throughput=8,662,875 op/s
threads=32	total_ops=33,554,432	spent=8.3s	latency=7.91µs    throughput=4,046,191 op/s

-- HashbrownHashMap - FxHasher
threads=1	total_ops=33,554,432	spent=2.7s	latency=80.00ns   throughput=12,499,268 op/s
threads=2	total_ops=33,554,432	spent=2.1s	latency=123.00ns  throughput=16,216,954 op/s
threads=4	total_ops=33,554,432	spent=2.3s	latency=273.00ns  throughput=14,635,683 op/s
threads=8	total_ops=33,554,432	spent=2.5s	latency=599.00ns  throughput=13,353,110 op/s
threads=16	total_ops=33,554,432	spent=3.3s	latency=1.57µs    throughput=10,211,156 op/s
threads=32	total_ops=33,554,432	spent=5.8s	latency=5.57µs    throughput=5,748,582 op/s

-- HashbrownHashMap - AhashHasher
threads=1	total_ops=33,554,432	spent=2.7s	latency=79.00ns   throughput=12,657,635 op/s
threads=2	total_ops=33,554,432	spent=2.3s	latency=134.00ns  throughput=14,883,942 op/s
threads=4	total_ops=33,554,432	spent=2.5s	latency=295.00ns  throughput=13,553,126 op/s
threads=8	total_ops=33,554,432	spent=2.8s	latency=665.00ns  throughput=12,020,944 op/s
threads=16	total_ops=33,554,432	spent=3.9s	latency=1.86µs    throughput=8,616,012 op/s
threads=32	total_ops=33,554,432	spent=5.6s	latency=5.33µs    throughput=6,004,247 op/s
```

### Flurry

[Flurry](https://crates.io/crates/flurry) is a port of Java's `java.util.concurrent.ConcurrentHashMap` to Rust.

```
-- Flurry - StdHasher
threads=1	total_ops=33,554,432	spent=6.3s	latency=187.00ns	throughput=5,326,869 op/s
threads=2	total_ops=33,554,432	spent=3.1s	latency=186.00ns	throughput=10,736,922 op/s
threads=4	total_ops=33,554,432	spent=1.6s	latency=188.00ns	throughput=21,164,300 op/s
threads=8	total_ops=33,554,432	spent=839.6ms	latency=200.00ns	throughput=39,966,944 op/s
threads=16	total_ops=33,554,432	spent=557.6ms	latency=265.00ns	throughput=60,181,188 op/s
threads=32	total_ops=33,554,432	spent=516.3ms	latency=492.00ns	throughput=64,986,992 op/s

-- Flurry - FxHasher
threads=1	total_ops=33,554,432	spent=5.8s	latency=172.00ns	throughput=5,808,279 op/s
threads=2	total_ops=33,554,432	spent=3.0s	latency=178.00ns	throughput=11,222,547 op/s
threads=4	total_ops=33,554,432	spent=1.5s	latency=176.00ns	throughput=22,694,596 op/s
threads=8	total_ops=33,554,432	spent=815.9ms	latency=194.00ns	throughput=41,127,695 op/s
threads=16	total_ops=33,554,432	spent=496.6ms	latency=236.00ns	throughput=67,563,590 op/s
threads=32	total_ops=33,554,432	spent=472.9ms	latency=450.00ns	throughput=70,954,322 op/s

-- Flurry - AhashHasher
threads=1	total_ops=33,554,432	spent=6.1s      latency=181.00ns    throughput=5,501,711 op/s
threads=2	total_ops=33,554,432	spent=3.0s      latency=176.00ns    throughput=11,337,997 op/s
threads=4	total_ops=33,554,432	spent=1.5s      latency=181.00ns    throughput=22,010,138 op/s
threads=8	total_ops=33,554,432	spent=812.7ms   latency=193.00ns    throughput=41,288,222 op/s
threads=16	total_ops=33,554,432	spent=478.7ms   latency=228.00ns    throughput=70,093,964 op/s
threads=32	total_ops=33,554,432	spent=492.0ms   latency=469.00ns    throughput=68,204,950 op/s

```

### CHashMap

[CHashMap](https://crates.io/crates/chashmap) is a fast, concurrent hash maps with extensive API. Doesn't support hasher
injection.

```
-- CHashMapTable
threads=1	total_ops=33,554,432	spent=5.3s	latency=156.00ns  throughput=6,388,604 op/s
threads=2	total_ops=33,554,432	spent=3.5s	latency=210.00ns  throughput=9,494,840 op/s
threads=4	total_ops=33,554,432	spent=3.7s	latency=446.00ns  throughput=8,950,519 op/s
threads=8	total_ops=33,554,432	spent=3.9s	latency=928.00ns  throughput=8,614,507 op/s
threads=16	total_ops=33,554,432	spent=4.7s	latency=2.22µs    throughput=7,212,732 op/s
threads=32	total_ops=33,554,432	spent=4.3s	latency=4.08µs    throughput=7,839,372 op/s
```

### Std::HashMap

Standard collection with [parking_lot's RWLock](https://crates.io/crates/parking_lot) around it.

```
-- RWLock<HashMap> - StdHasher
	threads=1	total_ops=33,554,432	spent=4.2s	latency=124.00ns  throughput=8,063,699 op/s
	threads=2	total_ops=33,554,432	spent=3.1s	latency=182.00ns  throughput=10,941,067 op/s
	threads=4	total_ops=33,554,432	spent=2.8s	latency=333.00ns  throughput=11,986,456 op/s
	threads=8	total_ops=33,554,432	spent=3.3s	latency=775.00ns  throughput=10,311,764 op/s
	threads=16	total_ops=33,554,432	spent=3.9s	latency=1.86µs    throughput=8,595,398 op/s
	threads=32	total_ops=33,554,432	spent=8.4s	latency=7.97µs    throughput=4,013,602 op/s

-- RWLock<HashMap> - FxHasher
	threads=1	total_ops=33,554,432	spent=2.6s	latency=76.00ns   throughput=13,083,593 op/s
	threads=2	total_ops=33,554,432	spent=2.2s	latency=131.00ns  throughput=15,177,634 op/s
	threads=4	total_ops=33,554,432	spent=2.2s	latency=261.00ns  throughput=15,313,425 op/s
	threads=8	total_ops=33,554,432	spent=2.8s	latency=666.00ns  throughput=12,003,108 op/s
	threads=16	total_ops=33,554,432	spent=3.4s	latency=1.62µs    throughput=9,891,256 op/s
	threads=32	total_ops=33,554,432	spent=5.9s	latency=5.67µs    throughput=5,647,334 op/s

-- RWLock<HashMap> - AhashHasher
	threads=1	total_ops=33,554,432	spent=2.7s	latency=79.00ns   throughput=12,609,698 op/s
	threads=2	total_ops=33,554,432	spent=2.4s	latency=140.00ns  throughput=14,210,944 op/s
	threads=4	total_ops=33,554,432	spent=2.4s	latency=283.00ns  throughput=14,105,936 op/s
	threads=8	total_ops=33,554,432	spent=2.8s	latency=673.00ns  throughput=11,879,163 op/s
	threads=16	total_ops=33,554,432	spent=3.6s	latency=1.70µs    throughput=9,388,587 op/s
	threads=32	total_ops=33,554,432	spent=5.1s	latency=4.89µs    throughput=6,543,613 op/s
```

### Std::BTreeMap

Standard collection with [parking_lot's RWLock](https://crates.io/crates/parking_lot) around it.

```
-- RWLock<BTreeMap>
	threads=1	total_ops=33,554,432	spent=14.7s	latency=438.00ns  throughput=2,281,567 op/s
	threads=2	total_ops=33,554,432	spent=8.1s	latency=482.00ns  throughput=4,149,124 op/s
	threads=4	total_ops=33,554,432	spent=5.4s	latency=638.00ns  throughput=6,265,544 op/s
	threads=8	total_ops=33,554,432	spent=5.1s	latency=1.22µs    throughput=6,536,579 op/s
	threads=16	total_ops=33,554,432	spent=6.7s	latency=3.20µs    throughput=4,996,845 op/s
	threads=32	total_ops=33,554,432	spent=10.4s	latency=9.87µs    throughput=3,240,777 op/s
```

## Write-heavy workload

### DashMap

```
-- DashMap - StdHasher
	threads=1	total_ops=33,554,432	spent=5.4s	latency=160.00ns	throughput=6,223,283 op/s
	threads=2	total_ops=33,554,432	spent=2.7s	latency=162.00ns	throughput=12,274,627 op/s
	threads=4	total_ops=33,554,432	spent=1.5s	latency=181.00ns	throughput=22,015,261 op/s
	threads=8	total_ops=33,554,432	spent=944.0ms	latency=225.00ns	throughput=35,545,997 op/s
	threads=16	total_ops=33,554,432	spent=765.3ms	latency=364.00ns	throughput=43,842,590 op/s
	threads=32	total_ops=33,554,432	spent=791.9ms	latency=755.00ns	throughput=42,371,854 op/s

-- DashMap - FxHasher
	threads=1	total_ops=33,554,432	spent=3.9s	latency=116.00ns	throughput=8,610,050 op/s
	threads=2	total_ops=33,554,432	spent=2.2s	latency=129.00ns	throughput=15,478,914 op/s
	threads=4	total_ops=33,554,432	spent=1.2s	latency=145.00ns	throughput=27,541,478 op/s
	threads=8	total_ops=33,554,432	spent=780.3ms	latency=186.00ns	throughput=43,003,757 op/s
	threads=16	total_ops=33,554,432	spent=621.7ms	latency=296.00ns	throughput=53,975,591 op/s
	threads=32	total_ops=33,554,432	spent=657.6ms	latency=627.00ns	throughput=51,022,288 op/s

-- DashMap - AhashHasher
	threads=1	total_ops=33,554,432	spent=3.3s	latency=97.00ns       throughput=10,271,776 op/s
	threads=2	total_ops=33,554,432	spent=1.8s	latency=108.00ns      throughput=18,373,320 op/s
	threads=4	total_ops=33,554,432	spent=1.1s	latency=129.00ns      throughput=30,924,955 op/s
	threads=8	total_ops=33,554,432	spent=769.4ms	latency=183.00ns      throughput=43,611,888 op/s
	threads=16	total_ops=33,554,432	spent=627.3ms	latency=299.00ns      throughput=53,489,340 op/s
	threads=32	total_ops=33,554,432	spent=660.2ms	latency=629.00ns      throughput=50,826,023 op/s
```

### HashBrown

```
-- HashbrownHashMap - StdHasher
	threads=1	total_ops=33,554,432	spent=4.8s	latency=144.00ns    throughput=6,938,407 op/s
	threads=2	total_ops=33,554,432	spent=6.5s	latency=387.00ns    throughput=5,166,344 op/s
	threads=4	total_ops=33,554,432	spent=10.4s	latency=1.24µs      throughput=3,231,832 op/s
	threads=8	total_ops=33,554,432	spent=29.1s	latency=6.93µs      throughput=1,154,733 op/s
	threads=16	total_ops=33,554,432	spent=33.2s	latency=15.83µs     throughput=1,010,713 op/s
	threads=32	total_ops=33,554,432	spent=38.5s	latency=36.76µs     throughput=870,604 op/s

-- HashbrownHashMap - FxHasher
	threads=1	total_ops=33,554,432	spent=4.2s	latency=123.00ns    throughput=8,083,490 op/s
	threads=2	total_ops=33,554,432	spent=5.5s	latency=326.00ns    throughput=6,131,892 op/s
	threads=4	total_ops=33,554,432	spent=8.6s	latency=1.03µs      throughput=3,884,150 op/s
	threads=8	total_ops=33,554,432	spent=21.3s	latency=5.07µs      throughput=1,577,349 op/s
	threads=16	total_ops=33,554,432	spent=39.8s	latency=18.96µs     throughput=843,898 op/s
	threads=32	total_ops=33,554,432	spent=17.1s	latency=16.26µs     throughput=1,967,680 op/s

-- HashbrownHashMap - AhashHasher
	threads=1	total_ops=33,554,432	spent=4.3s	latency=127.00ns    throughput=7,851,580 op/s
	threads=2	total_ops=33,554,432	spent=5.5s	latency=330.00ns    throughput=6,051,338 op/s
	threads=4	total_ops=33,554,432	spent=8.2s	latency=977.00ns    throughput=4,093,207 op/s
	threads=8	total_ops=33,554,432	spent=21.7s	latency=5.18µs      throughput=1,544,008 op/s
	threads=16	total_ops=33,554,432	spent=41.0s	latency=19.55µs     throughput=818,356 op/s
	threads=32	total_ops=33,554,432	spent=17.4s	latency=16.61µs     throughput=1,926,698 op/s
```

### Flurry

```
-- Flurry - StdHasher
	threads=1	total_ops=33,554,432	spent=10.5s	latency=312.00ns    throughput=3,195,553 op/s
	threads=2	total_ops=33,554,432	spent=4.8s	latency=289.00ns    throughput=6,919,031 op/s
	threads=4	total_ops=33,554,432	spent=3.2s	latency=376.00ns    throughput=10,622,794 op/s
	threads=8	total_ops=33,554,432	spent=2.2s	latency=522.00ns    throughput=15,306,958 op/s
	threads=16	total_ops=33,554,432	spent=1.6s	latency=763.00ns    throughput=20,958,353 op/s
	threads=32	total_ops=33,554,432	spent=1.5s	latency=1.39µs      throughput=23,095,150 op/s

-- Flurry - FxHasher
	threads=1	total_ops=33,554,432	spent=8.6s	latency=255.00ns    throughput=3,913,332 op/s
	threads=2	total_ops=33,554,432	spent=4.8s	latency=283.00ns    throughput=7,056,738 op/s
	threads=4	total_ops=33,554,432	spent=3.2s	latency=375.00ns    throughput=10,646,274 op/s
	threads=8	total_ops=33,554,432	spent=2.1s	latency=507.00ns    throughput=15,772,563 op/s
	threads=16	total_ops=33,554,432	spent=1.5s	latency=733.00ns    throughput=21,820,331 op/s
	threads=32	total_ops=33,554,432	spent=1.4s	latency=1.29µs      throughput=24,731,995 op/s

-- Flurry - AhashHasher
	threads=1	total_ops=33,554,432	spent=8.2s	latency=243.00ns    throughput=4,108,334 op/s
	threads=2	total_ops=33,554,432	spent=4.6s	latency=273.00ns    throughput=7,313,282 op/s
	threads=4	total_ops=33,554,432	spent=3.2s	latency=384.00ns    throughput=10,391,391 op/s
	threads=8	total_ops=33,554,432	spent=2.1s	latency=507.00ns    throughput=15,769,147 op/s
	threads=16	total_ops=33,554,432	spent=1.5s	latency=718.00ns    throughput=22,270,980 op/s
	threads=32	total_ops=33,554,432	spent=1.4s	latency=1.36µs      throughput=23,549,486 op/s
```

### CHashMap

```
-- CHashMapTable
	threads=1	total_ops=33,554,432	spent=5.1s	latency=151.00ns    throughput=6,588,760 op/s
	threads=2	total_ops=33,554,432	spent=3.6s	latency=216.00ns    throughput=9,220,696 op/s
	threads=4	total_ops=33,554,432	spent=3.9s	latency=465.00ns    throughput=8,597,276 op/s
	threads=8	total_ops=33,554,432	spent=4.2s	latency=990.00ns    throughput=8,080,326 op/s
	threads=16	total_ops=33,554,432	spent=4.1s	latency=1.97µs      throughput=8,111,109 op/s
	threads=32	total_ops=33,554,432	spent=4.2s	latency=4.00µs      throughput=7,992,182 op/s
```

### Std::HashMap

```
-- RWLock<HashMap> - StdHasher
	threads=1	total_ops=33,554,432	spent=4.9s	latency=145.00ns    throughput=6,869,440 op/s
	threads=2	total_ops=33,554,432	spent=6.5s	latency=389.00ns    throughput=5,133,215 op/s
	threads=4	total_ops=33,554,432	spent=10.3s	latency=1.23µs      throughput=3,249,879 op/s
	threads=8	total_ops=33,554,432	spent=29.5s	latency=7.04µs      throughput=1,136,060 op/s
	threads=16	total_ops=33,554,432	spent=44.7s	latency=21.30µs     throughput=751,318 op/s
	threads=32	total_ops=33,554,432	spent=43.0s	latency=40.97µs     throughput=781,046 op/s

-- RWLock<HashMap> - FxHasher
	threads=1	total_ops=33,554,432	spent=4.2s	latency=126.00ns    throughput=7,927,871 op/s
	threads=2	total_ops=33,554,432	spent=5.7s	latency=339.00ns    throughput=5,894,992 op/s
	threads=4	total_ops=33,554,432	spent=8.4s	latency=998.00ns    throughput=4,006,728 op/s
	threads=8	total_ops=33,554,432	spent=21.8s	latency=5.20µs      throughput=1,539,288 op/s
	threads=16	total_ops=33,554,432	spent=39.2s	latency=18.71µs     throughput=855,288 op/s
	threads=32	total_ops=33,554,432	spent=27.0s	latency=25.79µs     throughput=1,240,715 op/s

-- RWLock<HashMap> - AhashHasher
	threads=1	total_ops=33,554,432	spent=3.8s	latency=112.00ns    throughput=8,885,549 op/s
	threads=2	total_ops=33,554,432	spent=5.1s	latency=305.00ns    throughput=6,556,817 op/s
	threads=4	total_ops=33,554,432	spent=7.7s	latency=923.00ns    throughput=4,332,484 op/s
	threads=8	total_ops=33,554,432	spent=21.8s	latency=5.20µs      throughput=1,537,612 op/s
	threads=16	total_ops=33,554,432	spent=42.5s	latency=20.25µs     throughput=790,036 op/s
	threads=32	total_ops=33,554,432	spent=32.2s	latency=30.67µs     throughput=1,043,274 op/s
```

### Std::BTreeMap

```
-- RWLock<BTreeMap>
	threads=1	total_ops=33,554,432	spent=13.7s	latency=407.00ns    throughput=2,456,580 op/s
	threads=2	total_ops=33,554,432	spent=18.5s	latency=1.10µs      throughput=1,811,115 op/s
	threads=4	total_ops=33,554,432	spent=28.5s	latency=3.40µs      throughput=1,175,717 op/s
	threads=8	total_ops=33,554,432	spent=43.4s	latency=10.35µs     throughput=773,148 op/s
	threads=16	total_ops=33,554,432	spent=48.8s	latency=23.29µs     throughput=687,118 op/s
	threads=32	total_ops=33,554,432	spent=50.0s	latency=47.71µs     throughput=670,771 op/s
```

## Mixed workload

### DashMap

```
-- DashMap - StdHasher
	threads=1	total_ops=33,554,432	spent=5.9s	latency=176.00ns	throughput=5,673,658 op/s
	threads=2	total_ops=33,554,432	spent=3.2s	latency=190.00ns	throughput=10,478,106 op/s
	threads=4	total_ops=33,554,432	spent=1.8s	latency=209.00ns	throughput=19,055,066 op/s
	threads=8	total_ops=33,554,432	spent=1.1s	latency=258.00ns	throughput=30,922,509 op/s
	threads=16	total_ops=33,554,432	spent=743.2ms	latency=354.00ns	throughput=45,145,698 op/s
	threads=32	total_ops=33,554,432	spent=832.6ms	latency=794.00ns	throughput=40,301,878 op/s

-- DashMap - FxHasher
	threads=1	total_ops=33,554,432	spent=4.2s	latency=126.00ns	throughput=7,934,636 op/s
	threads=2	total_ops=33,554,432	spent=2.3s	latency=136.00ns	throughput=14,670,167 op/s
	threads=4	total_ops=33,554,432	spent=1.3s	latency=154.00ns	throughput=25,886,843 op/s
	threads=8	total_ops=33,554,432	spent=824.3ms	latency=196.00ns	throughput=40,707,329 op/s
	threads=16	total_ops=33,554,432	spent=719.7ms	latency=343.00ns	throughput=46,623,259 op/s
	threads=32	total_ops=33,554,432	spent=716.4ms	latency=683.00ns	throughput=46,836,583 op/s

-- DashMap - AhashHasher
	threads=1	total_ops=33,554,432	spent=3.9s	latency=116.00ns	throughput=8,565,802 op/s
	threads=2	total_ops=33,554,432	spent=2.1s	latency=125.00ns	throughput=15,891,670 op/s
	threads=4	total_ops=33,554,432	spent=1.3s	latency=149.00ns	throughput=26,788,954 op/s
	threads=8	total_ops=33,554,432	spent=802.9ms	latency=191.00ns	throughput=41,790,759 op/s
	threads=16	total_ops=33,554,432	spent=671.2ms	latency=320.00ns	throughput=49,989,372 op/s
	threads=32	total_ops=33,554,432	spent=729.3ms	latency=695.00ns	throughput=46,010,116 op/s
```

### HashBrown

```
-- HashbrownHashMap - StdHasher
	threads=1	total_ops=33,554,432	spent=5.5s	latency=164.00ns    throughput=6,062,268 op/s
	threads=2	total_ops=33,554,432	spent=7.2s	latency=430.00ns    throughput=4,643,449 op/s
	threads=4	total_ops=33,554,432	spent=12.3s	latency=1.46µs      throughput=2,733,832 op/s
	threads=8	total_ops=33,554,432	spent=30.0s	latency=7.15µs      throughput=1,119,477 op/s
	threads=16	total_ops=33,554,432	spent=45.1s	latency=21.52µs     throughput=743,414 op/s
	threads=32	total_ops=33,554,432	spent=40.3s	latency=38.39µs     throughput=833,529 op/s

-- HashbrownHashMap - FxHasher
	threads=1	total_ops=33,554,432	spent=4.1s	latency=122.00ns    throughput=8,145,592 op/s
	threads=2	total_ops=33,554,432	spent=5.6s	latency=330.00ns    throughput=6,044,676 op/s
	threads=4	total_ops=33,554,432	spent=8.4s	latency=999.00ns    throughput=4,002,199 op/s
	threads=8	total_ops=33,554,432	spent=21.5s	latency=5.12µs      throughput=1,562,567 op/s
	threads=16	total_ops=33,554,432	spent=41.9s	latency=20.00µs     throughput=800,186 op/s
	threads=32	total_ops=33,554,432	spent=32.5s	latency=30.98µs     throughput=1,032,954 op/s

-- HashbrownHashMap - AhashHasher
	threads=1	total_ops=33,554,432	spent=4.3s	latency=129.00ns    throughput=7,731,301 op/s
	threads=2	total_ops=33,554,432	spent=5.5s	latency=327.00ns    throughput=6,109,628 op/s
	threads=4	total_ops=33,554,432	spent=8.3s	latency=990.00ns    throughput=4,036,860 op/s
	threads=8	total_ops=33,554,432	spent=22.0s	latency=5.25µs      throughput=1,523,613 op/s
	threads=16	total_ops=33,554,432	spent=41.8s	latency=19.92µs     throughput=803,300 op/s
	threads=32	total_ops=33,554,432	spent=40.7s	latency=38.78µs     throughput=825,243 op/s
```

### Flurry

```
-- Flurry - StdHasher
	threads=1	total_ops=33,554,432	spent=9.5s	latency=284.00ns    throughput=3,518,594 op/s
	threads=2	total_ops=33,554,432	spent=5.3s	latency=315.00ns    throughput=6,329,168 op/s
	threads=4	total_ops=33,554,432	spent=3.4s	latency=406.00ns    throughput=9,842,157 op/s
	threads=8	total_ops=33,554,432	spent=2.5s	latency=595.00ns    throughput=13,425,534 op/s
	threads=16	total_ops=33,554,432	spent=1.9s	latency=908.00ns    throughput=17,609,395 op/s
	threads=32	total_ops=33,554,432	spent=2.0s	latency=1.87µs      throughput=17,119,716 op/s

-- Flurry - FxHasher
	threads=1	total_ops=33,554,432	spent=8.7s	latency=259.00ns    throughput=3,851,492 op/s
	threads=2	total_ops=33,554,432	spent=4.9s	latency=293.00ns    throughput=6,824,122 op/s
	threads=4	total_ops=33,554,432	spent=3.1s	latency=366.00ns    throughput=10,900,970 op/s
	threads=8	total_ops=33,554,432	spent=2.2s	latency=513.00ns    throughput=15,567,971 op/s
	threads=16	total_ops=33,554,432	spent=2.1s	latency=1.02µs      throughput=15,755,152 op/s
	threads=32	total_ops=33,554,432	spent=1.9s	latency=1.80µs      throughput=17,773,115 op/s

-- Flurry - AhashHasher
	threads=1	total_ops=33,554,432	spent=9.2s	latency=274.00ns    throughput=3,647,041 op/s
	threads=2	total_ops=33,554,432	spent=5.1s	latency=302.00ns    throughput=6,601,390 op/s
	threads=4	total_ops=33,554,432	spent=3.5s	latency=411.00ns    throughput=9,711,046 op/s
	threads=8	total_ops=33,554,432	spent=2.5s	latency=597.00ns    throughput=13,380,680 op/s
	threads=16	total_ops=33,554,432	spent=1.8s	latency=848.00ns    throughput=18,855,749 op/s
	threads=32	total_ops=33,554,432	spent=1.6s	latency=1.57µs      throughput=20,434,399 op/s
```

### CHashMap

```
-- CHashMapTable
	threads=1	total_ops=33,554,432	spent=5.1s	latency=153.00ns    throughput=6,529,219 op/s
	threads=2	total_ops=33,554,432	spent=3.7s	latency=220.00ns    throughput=9,064,433 op/s
	threads=4	total_ops=33,554,432	spent=4.0s	latency=477.00ns    throughput=8,378,358 op/s
	threads=8	total_ops=33,554,432	spent=4.2s	latency=1.00µs      throughput=7,996,181 op/s
	threads=16	total_ops=33,554,432	spent=4.3s	latency=2.03µs      throughput=7,884,022 op/s
	threads=32	total_ops=33,554,432	spent=4.2s	latency=4.02µs      throughput=7,962,576 op/s
```

### Std::HashMap

```
-- RWLock<HashMap> - StdHasher
	threads=1	total_ops=33,554,432	spent=5.4s	latency=162.00ns    throughput=6,163,341 op/s
	threads=2	total_ops=33,554,432	spent=7.3s	latency=436.00ns    throughput=4,582,142 op/s
	threads=4	total_ops=33,554,432	spent=12.2s	latency=1.46µs      throughput=2,739,574 op/s
	threads=8	total_ops=33,554,432	spent=28.6s	latency=6.82µs      throughput=1,172,753 op/s
	threads=16	total_ops=33,554,432	spent=45.9s	latency=21.87µs     throughput=731,606 op/s
	threads=32	total_ops=33,554,432	spent=45.1s	latency=42.99µs     throughput=744,399 op/s

-- RWLock<HashMap> - FxHasher
	threads=1	total_ops=33,554,432	spent=4.2s	latency=126.00ns    throughput=7,913,733 op/s
	threads=2	total_ops=33,554,432	spent=5.4s	latency=319.00ns    throughput=6,257,984 op/s
	threads=4	total_ops=33,554,432	spent=8.6s	latency=1.02µs      throughput=3,914,671 op/s
	threads=8	total_ops=33,554,432	spent=21.9s	latency=5.22µs      throughput=1,532,322 op/s
	threads=16	total_ops=33,554,432	spent=42.2s	latency=20.12µs     throughput=795,399 op/s
	threads=32	total_ops=33,554,432	spent=42.9s	latency=40.91µs     throughput=782,200 op/s

-- RWLock<HashMap> - AhashHasher
	threads=1	total_ops=33,554,432	spent=4.1s	latency=121.00ns    throughput=8,210,276 op/s
	threads=2	total_ops=33,554,432	spent=5.6s	latency=334.00ns    throughput=5,980,151 op/s
	threads=4	total_ops=33,554,432	spent=8.7s	latency=1.04µs      throughput=3,853,543 op/s
	threads=8	total_ops=33,554,432	spent=23.0s	latency=5.47µs      throughput=1,461,841 op/s
	threads=16	total_ops=33,554,432	spent=41.5s	latency=19.80µs     throughput=807,956 op/s
	threads=32	total_ops=33,554,432	spent=33.8s	latency=32.22µs     throughput=993,019 op/s
```

### Std::BTreeMap

```
-- RWLock<BTreeMap>
	threads=1	total_ops=33,554,432	spent=17.2s	latency=513.00ns    throughput=1,946,113 op/s
	threads=2	total_ops=33,554,432	spent=24.1s	latency=1.44µs      throughput=1,392,402 op/s
	threads=4	total_ops=33,554,432	spent=36.9s	latency=4.40µs      throughput=908,718 op/s
	threads=8	total_ops=33,554,432	spent=51.7s	latency=12.33µs     throughput=649,058 op/s
	threads=16	total_ops=33,554,432	spent=55.8s	latency=26.60µs     throughput=601,585 op/s
	threads=32	total_ops=33,554,432	spent=56.3s	latency=53.66µs     throughput=596,383 op/s
```
