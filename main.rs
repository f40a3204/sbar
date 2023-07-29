use anyhow::Result;
use cnx::text::*;
use cnx::widgets::*;
use cnx::{Cnx, Position};
use cnx_contrib::widgets::cpu::Cpu;
use cnx_contrib::widgets::battery::Battery;
use cnx_contrib::widgets::volume::Volume;
use cnx_contrib::widgets::disk_usage::DiskUsage;

fn main() -> Result<()> {
    let mut cnx = Cnx::new(Position::Top);
    cnx.add_widget(workspaces());
    cnx.add_widget(active_window_title());
    cnx.add_widget(volume_info());
    cnx.add_widget(cpu());
    cnx.add_widget(disk_info());
    cnx.add_widget(battery_info());
    cnx.add_widget(clock());

    cnx.run()?;

    Ok(())
}

fn default_attr() -> Attributes {
    Attributes {
        font: Font::new("Hack Nerd Font"),
        fg_color: Color::white(),
        bg_color: Some(Color::black()),
        padding: Padding::new(8.0, 8.0, 1.0, 1.0),
    }
}

fn workspaces() -> Pager {
    let workspace = PagerAttributes {
        active_attr: Attributes { 
            bg_color: Some(Color::from_hex("#fe8019")),
            fg_color: Color::from_hex("#e9d9b1"),
            ..default_attr() },
        inactive_attr: Attributes { 
            fg_color: Color::from_hex("#3c3836"),
            ..default_attr() },
        non_empty_attr: Attributes { 
            ..default_attr() },
    };

    Pager::new(workspace)
}

fn active_window_title() -> ActiveWindowTitle {
    let window_tile = Attributes {
            bg_color: Some(Color::from_hex("#fe8019")),
            ..default_attr() };
    ActiveWindowTitle::new(window_tile)
}

fn battery_info() -> Battery {
    let default_battery = Attributes {
        ..default_attr()
    };
    Battery::new(default_battery, 
                 Color::from_hex("#fe8019"),
                 Some(String::from("BAT0")),
                 None
                 )
}

fn clock() -> Clock {
    let attr = Attributes { ..default_attr() };
    Clock::new(attr, None)
}

fn cpu() -> Cpu {
    let cpu_info = Attributes { ..default_attr() };
    let output = match Cpu::new(cpu_info, None) {
        Ok(expr) => expr,
        Err(error) => panic!("{}", error),
    };
    return output;
}

fn volume_info() -> Volume {
    let volume_info = Attributes {
        ..default_attr()
    };

    Volume::new(volume_info)
}

fn disk_info() -> DiskUsage {
    let disk_info = Attributes {
        ..default_attr()
    };
    DiskUsage::new(disk_info, "/".into(), None)
}
