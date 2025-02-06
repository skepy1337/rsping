use colored::Colorize;
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

fn is_port_open(ip: &str, port: u16, timeout: u64) -> bool {
    let ip_addr: Result<IpAddr, _> = ip.parse();
    if let Ok(ip_addr) = ip_addr {
        let socket_addr = SocketAddr::new(ip_addr, port);
        TcpStream::connect_timeout(&socket_addr, Duration::from_millis(timeout)).is_ok()
    } else {
        eprintln!("Could not parse ip: {}", ip);
        std::process::exit(1);
    }
}

fn dns_resolve(hostname: &str) -> String {
    let socket_addrs = (hostname, 0).to_socket_addrs();

    if let Ok(mut addrs) = socket_addrs {
        if let Some(addr) = addrs.next() {
            return addr.ip().to_string();
        }
    } else {
        eprintln!("Hostname resolve failed");
        std::process::exit(1);
    }

    String::default()
}

fn set_terminal_title(title: &str) {
    #[cfg(target_os = "windows")]
    {
        use winapi::um::wincon::SetConsoleTitleW;
        use winapi::um::winnt::WCHAR;

        let wide_title: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let _ = unsafe { SetConsoleTitleW(wide_title.as_ptr() as *const WCHAR) };
    }

    #[cfg(not(target_os = "windows"))]
    {
        print!("\x1B]2;{}\x07", title);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <IP address / hostname> <port>\n", args[0]);
        std::process::exit(0);
    }

    let target = dns_resolve(&args[1]);
    let port = args[2].parse::<u16>().unwrap();
    let timeout = 2000;

    set_terminal_title(&format!("Pinging {}:{}", target, port));
    println!(
        "\nConnecting to {} on port {}:\r\n",
        &target.bright_yellow(),
        port.to_string().bright_yellow()
    );

    loop {
        let start_time = Instant::now();
        if is_port_open(&target, port, timeout) {
            let end_time = Instant::now();
            let duration = end_time.duration_since(start_time);

            let latency_ms = ((duration.as_secs() as f64 * 1000.0
                + duration.subsec_micros() as f64 / 1000.0)
                * 100.0)
                .round()
                / 100.0;

            println!(
                "Connected to {}: time={} port={}",
                &target.bright_green(),
                format!("{:.2}ms", &latency_ms).bright_green(),
                port.to_string().bright_green()
            );
        } else {
            println!("{}", format!("Connection timed out\r").bright_red());
        }
        std::thread::sleep(Duration::from_millis(1000));
    }
}
