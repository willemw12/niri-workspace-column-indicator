use anyhow::{Result, bail};
use niri_ipc::{Event, Request, Response, socket::Socket};

fn main() -> Result<()> {
    let mut socket_events = Socket::connect()?;
    if let Err(e) = socket_events.send(Request::EventStream)? {
        bail!("Failed to start event stream: {e}");
    }

    let mut socket_requests = Socket::connect()?;

    print_columns_info(&mut socket_requests)?;

    let mut read_event = socket_events.read_events();
    while let Ok(event) = read_event() {
        // println!("DEBUG: {event:?}");

        #[allow(clippy::single_match)]
        match event {
            Event::WindowFocusChanged { .. }
            | Event::WindowOpenedOrChanged { .. }     // When a column could be created
            | Event::WindowClosed { .. }              // When a column on another workspace could be deleted
            | Event::WindowLayoutsChanged { .. } => { // When a window consume/expel could delete/create a column
                print_columns_info(&mut socket_requests)?;
            }
            _ => (),
        }
    }

    Ok(())
}

fn print_columns_info(socket: &mut Socket) -> Result<()> {
    #[allow(clippy::single_match)]
    match socket.send(Request::FocusedWindow)? {
        Ok(Response::FocusedWindow(Some(window))) => {
            let Some(workspace_id) = window.workspace_id else {
                return Ok(());
            };

            let Ok(Response::Windows(windows)) = socket.send(Request::Windows)? else {
                bail!("Failed to get windows");
            };

            let total_columns = windows
                .into_iter()
                .filter(|w| w.workspace_id == Some(workspace_id) && !w.is_floating)
                .filter_map(|w| w.layout.pos_in_scrolling_layout.map(|(c, _)| c))
                .max()
                .unwrap_or(0);

            let column_pos = window.layout.pos_in_scrolling_layout.map(|(c, _)| c);

            ////

            let bar: String = (1..=total_columns)
                .map(|i| {
                    if Some(i) == column_pos {
                        " " // ' '
                    } else {
                        " "
                    }
                })
                .collect();
            if let Some(column_pos) = column_pos {
                println!("{bar}  ({column_pos})");
            } else {
                println!("{bar}");
            }
        }
        _ => println!(),
    }

    Ok(())
}

// fn print_workspace_info() -> Result<()> {
//     let mut socket = Socket::connect()?;
//
//     let reply = socket.send(Request::Workspaces)?;
//     let Ok(Response::Workspaces(workspaces)) = reply else {
//         bail!("Failed to get workspaces: {reply:?}");
//     };
//
//     let focused_workspace_id = workspaces.iter().find(|ws| ws.is_focused).map(|ws| ws.id);
//     if let Some(id) = focused_workspace_id {
//         println!("focused_workspace_id = {id}");
//     } else {
//         println!();
//     }
//
//     Ok(())
// }
