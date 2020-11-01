# Proxies Test

Test the proxies by sending request to some website for n times and measure the time it took.

I faced a issue with Luminati proxies where some proxies were too slow. So I decided to make this tool.


## Usage

In proxies.txt enter proxies like this

```
zproxy.lum-superproxy.io:22225:lum-customer-hl_123123123123-zone-zone1-ip-192.240.19.168:aaaaaaaaaa
```


Adjust the the config u want and run it via cargo like this.

```bash
cargo run --release
```


It should print the timing etc in terminal and finally in result you will have proxy sorted by minimum time it took to perform 10 request.


### Future Plans
- Dedicated Config File
- Upgrade Tokio / Reqwest Crate.


### Related Projects
- [Rotating Proxies](https://github.com:shirshak55/rotating_proxies.git)

## Author
Shirshak Bajgain
shirshak55