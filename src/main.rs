use std::ffi::CString;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, ArgMatches};
use windows::Win32::{
    Foundation::PSTR,
    Graphics::Gdi::{AddFontResourceA, RemoveFontResourceA},
};

fn add_font<P>(path: P) -> anyhow::Result<()>
where
    P: Into<Vec<u8>>,
{
    let mut file = CString::new(path)?.into_bytes_with_nul();
    let ret = unsafe { AddFontResourceA(PSTR(file.as_mut_ptr())) };
    if ret == 0 {
        anyhow::bail!("failed to add font");
    }

    Ok(())
}

fn remove_font<P>(path: P) -> anyhow::Result<()>
where
    P: Into<Vec<u8>>,
{
    let mut file = CString::new(path)?.into_bytes_with_nul();
    let ret = unsafe { RemoveFontResourceA(PSTR(file.as_mut_ptr())) };
    if !ret.as_bool() {
        anyhow::bail!("failed to remove font");
    }

    Ok(())
}

fn exec_cmd_add(matches: &ArgMatches) -> anyhow::Result<()> {
    match matches.values_of("file") {
        Some(files) => {
            for file in files {
                add_font(file)?;
            }
        }
        None => anyhow::bail!("invalid args"),
    }

    Ok(())
}

fn exec_cmd_remove(matches: &ArgMatches) -> anyhow::Result<()> {
    match matches.values_of("file") {
        Some(files) => {
            for file in files {
                remove_font(file)?;
            }
        }
        None => anyhow::bail!("invalid args"),
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            App::new("add")
                .about("add font")
                .arg_from_usage("<file>... 'font files'"),
        )
        .subcommand(
            App::new("remove")
                .about("remove font")
                .arg_from_usage("<file>... 'font files'"),
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(m)) => exec_cmd_add(m),
        ("remove", Some(m)) => exec_cmd_remove(m),
        _ => {
            eprintln!("{}", matches.usage());
            Ok(())
        }
    }
}
