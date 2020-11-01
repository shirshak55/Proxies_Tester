#[tokio::main]
async fn main() {
    let proxies = list_of_all_proxies();

    let mut vv = Vec::with_capacity(400);
    for proxy in proxies {
        let data = test_proxies(proxy).await;

        if let Some(data) = data{
            vv.push(data);
        }
    }

    vv.sort_by(|a, b| b.2.cmp(&a.2));

    for ss in vv.iter() {
        println!(
            "Username:{} - Time {} - Good Resp {} - Bad Resp {}",
            ss.3.username, ss.2, ss.0, ss.1
        )
    }

    println!("=============================================");
    let a = vv
        .into_iter()
        .map(|v| {
            let pp = v.3;
            let mut split = pp.hosts.split(":").into_iter();
            let hosts = split.next().unwrap();
            let port = split.next().unwrap();
            format!("{}:{}:{}:{}", hosts, port, pp.username, pp.password)
        })
        .collect::<Vec<_>>();

    use std::io::prelude::*;

    let mut handle = std::fs::File::create("result.txt").unwrap();
    let to_write = a.join("\n");
    handle.write_all(to_write.as_bytes()).unwrap();
}

struct Proxy {
    username: String,
    password: String,
    hosts: String,
}

fn list_of_all_proxies() -> Vec<Proxy> {
    let content = std::fs::read_to_string("proxies.txt").expect("Please ensure proxies file exits");
    content
        .lines()
        .into_iter()
        .map(|v| {
            let mut splits = v.split(":").into_iter();

            let hosts = format!("{}:{}", splits.next().unwrap(), splits.next().unwrap());
            let username = splits.next().expect("Invalid Username").to_string();
            let password = splits.next().expect("Invalid Password").to_string();

            Proxy {
                username,
                password,
                hosts,
            }
        })
        .collect::<Vec<_>>()
}

async fn test_proxies(proxy: Proxy) -> Option<(usize, usize, u128, Proxy)> {
    let pp = format!(
        "http://{}:{}@{}",
        proxy.username, proxy.password, proxy.hosts
    );
    let pxy = reqwest::Proxy::all(&pp).expect("tor proxy should be there");
    let client = reqwest::Client::builder()
        .proxy(pxy)
        .build()
        .expect("Unable to build reqwest client");

    let duration = std::time::Instant::now();

    let mut good_resp: usize = 0;
    let mut bad_resp: usize = 0;

    for ii in 0..10usize {
        let res = client.get("https://jsonip.com").send().await;

        if let Ok(resp) = res {
            let text = resp.text().await;

            if let Ok(tt) = text {
                if ii == 0 {
                    // println!("IP: {:?}", tt);
                }

                if tt.contains("API Help") == true {
                    good_resp += 1;
                } else {
                    println!("{:?}", tt);
                    bad_resp += 1;
                }
            } else {
                bad_resp += 1;
            }
        } else {
            bad_resp += 1;
        }
    }

    println!(
        "{:?}",
        (&proxy.username, duration.elapsed().as_micros(), bad_resp)
    );

    if(bad_resp>0){
        return None
    }

    Some((good_resp, bad_resp, duration.elapsed().as_micros(), proxy))
}
