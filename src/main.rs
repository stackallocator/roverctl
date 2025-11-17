use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal,
};
use std::io::{self, Write};
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    println!("roverctl: Press W, A, S, D, or Space.");
    println!("Press Ctrl+C to exit...");

    terminal::enable_raw_mode()?;
    io::stdout().flush()?;
    let result = run_key_listener();

    terminal::disable_raw_mode()?;

    if let Err(e) = &result {
        eprintln!("Error: {}", e);
    }

    result
}

fn run_key_listener() -> io::Result<()> {
    terminal::disable_raw_mode()?;

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let dest_addr = "192.168.1.112:44144";
    println!(
        "Socket bound to {}, sending to {}",
        socket.local_addr()?,
        dest_addr
    );

    terminal::enable_raw_mode()?;

    loop {
        let event = event::read()?;

        let Event::Key(event::KeyEvent {
            code,
            modifiers,
            kind,
            #[allow(unused_variables)]
            state,
        }) = event
        else {
            continue;
        };
        if kind == KeyEventKind::Press {
            if code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
                println!("Goodbye!!!");
                break;
            }

            let data_to_send: Option<&[u8]> = match code {
                KeyCode::Char('w') | KeyCode::Char('W') => Some("f".as_bytes()),
                KeyCode::Char('a') | KeyCode::Char('A') => Some("l".as_bytes()),
                KeyCode::Char('s') | KeyCode::Char('S') => Some("b".as_bytes()),
                KeyCode::Char('d') | KeyCode::Char('D') => Some("r".as_bytes()),
                KeyCode::Char(' ') => Some("s".as_bytes()),
                _ => None,
            };

            if let Some(data) = data_to_send {
                socket.send_to(data, dest_addr)?;

                let char_sent = std::str::from_utf8(data).unwrap_or("?");
                print!("Sent: '{}'    \r", char_sent);
                io::stdout().flush()?;
            }
        }
    }

    Ok(())
}
